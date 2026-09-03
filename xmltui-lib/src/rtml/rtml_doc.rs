use std::collections::HashMap;

use ratatui::{buffer::Buffer, layout::{Constraint, Direction, Flex, Layout, Rect}, style::Style};
use serde_json::{Map, Value};
use tokio_util::sync::CancellationToken;

use crate::{async_app::async_app::spawn_async_task, code::{event::{CommandExecutorParams, ExecutorEventType, new_command_executor}, executor::Executor}, input::event::InputEvent, rtml::{rtml_border::render_rtml_border, rtml_button::render_rtml_button, rtml_command::{CommandRefresh, RTMLCommandOutput, render_rtml_command}, rtml_input::render_rtml_input, rtml_layout::render_rtml_layout, rtml_line::render_rtml_line, rtml_link::render_rtml_link, rtml_node::{FocusEventResponse, RTMLNode, RTMLNodeId, XMLNodeWrapper, render_focus_node}, rtml_padding::RTMLPadding, rtml_paragraph::{create_paragraph, render_rtml_paragraph}, rtml_select::render_rtml_select, util::rtml_event::{CallbackChangeState, RTMLCallbackAction}}, state::{command_state::CommandState, state_executor::StateExecutor, var_state::change_var_state}, util::{json::{create_or_replace_path, json_value_to_string}, log::log_to_file}, xml::styles::xml_style::{StyleSelector, XMLStyle}};

#[derive(Debug)]
pub struct RTMLDoc 
{
    pub doc_id : RTMLNodeId,
    pub doc : HashMap<RTMLNodeId, RTMLNode>,
    pub root_id : RTMLNodeId,
    pub focus : Option<usize>,
    pub sorted_nodes : Vec<RTMLNodeId>,
    pub style : Style,
    pub styles : HashMap<StyleSelector, XMLStyle>,
    pub executors : HashMap<String, Executor>,
    pub cancellation_tokens : HashMap<String, CancellationToken>,
    pub templates : HashMap<String, String>,
    pub state : Value,
    pub state_executors : HashMap<String, StateExecutor>
}

impl RTMLDoc
{
    pub fn new(
        styles : HashMap<StyleSelector, XMLStyle>,
        executors : HashMap<String, Executor>,
        templates : HashMap<String, String>,
        state_executors : HashMap<String, StateExecutor>
    ) -> Self
    {
        let mut doc = Self::empty();

        doc.styles = styles;
        doc.executors = executors;
        doc.templates = templates;
        doc.state_executors = state_executors;

        doc
    }

    pub fn empty() -> Self
    {
        Self 
        { 
            styles : HashMap::new(), 
            style : Style::default(), 
            doc_id : "".to_string(), 
            doc : HashMap::new(), 
            root_id : "".to_string(), 
            focus : None, 
            sorted_nodes : vec![], 
            executors : HashMap::new(),
            cancellation_tokens : HashMap::new(),
            templates : HashMap::new(),
            state : Value::Object( Map::new() ),
            state_executors : HashMap::new()
        }
    }

    pub fn node_mut_by_id( &mut self, id : &str ) -> Option<&mut RTMLNode>
    {
        self.doc.get_mut( id )
    }

    pub fn node_ref_by_id( &self, id : &str ) -> Option<&RTMLNode>
    {
        self.doc.get( id )
    }

    pub fn sort_nodes( &mut self )
    {
        let focus = if let Some( index ) = self.focus && index < self.sorted_nodes.len()
        {
            Some( self.sorted_nodes.swap_remove( index ) )
        }
        else
        {
            None    
        };

        self.focus = None;

        self.sorted_nodes = self.sorted_nodes();

        if let Some( focus ) = focus
        {
            match self.sorted_nodes.iter().position( | x | x == &focus )
            {
                Some( p ) =>
                {
                    self.focus = Some( p );
                },
                None => {}
            }
        }
    }

    fn sorted_nodes( &self ) -> Vec<String>
    {
        RTMLDocIterator::from( self ).map(
            | e |
            {
                e
            }
        ).collect()
    }

    pub fn focus_next( &mut self )
    {
        if self.sorted_nodes.len() == 0
        {
            self.focus = None;

            return;
        }

        let next = if let Some( idx ) = self.focus && idx < ( self.sorted_nodes.len() - 1 )
        {
            idx + 1
        }
        else
        {
            0    
        };

        self.focus = Some( next );
    }

    pub fn focus_back( &mut self )
    {
        if self.sorted_nodes.len() == 0
        {
            self.focus = None;

            return;
        }

        let prev = if let Some( idx ) = self.focus && idx > 0
        {
            idx - 1
        }
        else
        {
            self.sorted_nodes.len() - 1
        };

        self.focus = Some( prev );
    }

    pub fn change_focus( &mut self, id : &RTMLNodeId )
    {
        if self.sorted_nodes.len() == 0 { return };

        let next_focus = self.sorted_nodes
        .iter()
        .enumerate()
        .find_map(
            | ( i, n ) |
            {
                if n == id
                {
                    Some( i )
                }
                else
                {
                    None    
                }
            }
        );

        if let Some( idx ) = next_focus
        {
            self.focus = Some( idx );
        }
    }

    pub fn current_focus( &self ) -> Option<&RTMLNode>
    {
        if let Some( idx ) = self.focus && idx < self.sorted_nodes.len()
        {
            self.node_ref_by_id( &self.sorted_nodes[ idx ] )
        }
        else
        {
            None
        }
    }

    pub fn current_focus_mut( &mut self ) -> Option<&mut RTMLNode>
    {
        if let Some( idx ) = self.focus && idx < self.sorted_nodes.len()
        {
            let id = self.sorted_nodes[ idx ].clone();

            self.node_mut_by_id( &id )
        }
        else
        {
            None    
        }
    }

    pub fn focus_event( &mut self, event : &InputEvent ) -> bool
    {
        match self.current_focus_mut()
        {
            Some( n ) =>
            {
                let response = n.focus_event( event );

                self.process_focus_event_response( response )
            },
            None => false
        }
    }

    fn process_focus_event_response( &mut self, response : FocusEventResponse ) -> bool
    {
        match response.state
        {
            Some( ( path, val ) ) =>
            {
                create_or_replace_path( path.as_str(), &mut self.state, val );

                response.changed
            },
            None => response.changed 
        }
    }

    pub fn init_state( &mut self )
    {
        let ids = self.doc.keys().map( | k | k.to_string() ).collect::<Vec<_>>();

        self.sincronize_state_from_ids( ids );

        for ( _, val ) in &self.state_executors
        {
            match val
            {
                StateExecutor::Var( v ) =>
                {
                    change_var_state( v, &mut self.state );
                },
                StateExecutor::Command( c ) =>
                {
                    if ! c.on_init { continue };

                    self.exec_command_state( c );
                }
            }
        }
    }

    fn exec_command_state( &self, state : &CommandState )
    {
        if let Some( executors ) = self.executors_from_ids( &state.executors )
        {
            let doc_id: String = self.doc_id.clone();
            let node_id = self.root_id.clone();
            let node_data = HashMap::new();
            let node_value = HashMap::new();
            let args = self.state_from_key_path( &state.args );
            let envs = self.state_from_key_path( &state.envs );

            let params = CommandExecutorParams::new(
                doc_id, 
                node_id, 
                node_data, 
                node_value,
                args,
                envs,
                CommandRefresh::Once, 
                executors, 
                ExecutorEventType::Callback(
                    RTMLCallbackAction::ChangeState(
                        CallbackChangeState::new(
                            state.common.path.clone(),  
                            state.common.stype.clone(),
                            state.template.clone(),
                            state.output.clone()
                        )
                    )
                ), 
                None, 
                None
            );

            spawn_async_task(
                async move 
                {
                    new_command_executor( params ).await
                }
            );
        }
    }

    pub fn init_state_from_childs( &mut self, parent_id : &RTMLNodeId )
    {
        let ids = self.all_childs_ids( parent_id );

        self.sincronize_state_from_ids( ids );
    }

    pub fn init_state_for_node_and_childs( &mut self, node_id : &RTMLNodeId )
    {
        let mut ids = self.all_childs_ids( node_id );

        ids.push( node_id.clone() );

        self.sincronize_state_from_ids( ids );
    }

    fn sincronize_state_from_ids( &mut self, ids : Vec<String> )
    {
        for id in ids
        {
            self.sincronize_state_from_id( &id );
        }
    }

    fn sincronize_state_from_id( &mut self, id : &str )
    {
        if let Some( n ) = self.doc.get( id )
        {
            if let Some( ( p, v ) ) = n.state_value()
            {
                create_or_replace_path( p.as_str(), &mut self.state, v );
            }
        }
    }

    pub fn init_commands( &mut self, cancellation_token : CancellationToken )
    {
        let ids = self.doc.keys().map( | k | k.to_string() ).collect::<Vec<_>>();

        self.init_commands_from_ids( cancellation_token, ids );
    }

    pub fn init_commands_for_childs( &mut self, cancellation_token : CancellationToken, parent_id : &RTMLNodeId )
    {
        let ids = self.all_childs_ids( parent_id );

        self.init_commands_from_ids( cancellation_token, ids );
    }

    pub fn init_commands_for_node_and_childs( &mut self, cancellation_token : CancellationToken, node_id : &RTMLNodeId )
    {
        let mut ids = self.all_childs_ids( node_id );

        ids.push( node_id.clone() );

        self.init_commands_from_ids( cancellation_token, ids );
    }

    fn init_commands_from_ids( &mut self, cancellation_token : CancellationToken, ids : Vec<String> )
    {
        let tokens = self.option_refresh_commands( Some( cancellation_token ), ids, None );

        for ( node_id, token ) in tokens
        {
            self.cancellation_tokens.insert( node_id, token );
        }
    }

    pub fn refresh_commands(
        &self,
        ids : Vec<String>
    )
    {
        self.option_refresh_commands( None, ids, Some( CommandRefresh::Once ) );
    }

    fn option_refresh_commands( 
        &self, 
        cancellation_token : Option<CancellationToken>, 
        ids : Vec<String>, 
        refresh : Option<CommandRefresh> 
    ) -> Vec<( String, CancellationToken )>
    {
        let mut ret = vec![];

        for node_id in ids
        {
            if let Some( node ) = self.doc.get( &node_id )
            {
                match node
                {
                    RTMLNode::Command( c ) =>
                    {
                        if let Some( executor ) = self.executors_from_ids( &c.executors )
                        {
                            let global_cancel = cancellation_token.clone();

                            let local_cancel_send = if global_cancel.as_ref().is_some()
                            {
                                let local_cancel = CancellationToken::new();
                                let local_cancel_send = Some( local_cancel.clone() );

                                // self.cancellation_tokens.insert( node_id.clone(), local_cancel );

                                ret.push( ( node_id.clone(), local_cancel ) );

                                local_cancel_send
                            }
                            else
                            {
                                None    
                            };

                            let doc_id = self.doc_id.clone();
                            let node_id = node_id.clone();

                            let refresh = match refresh.as_ref()
                            {
                                Some( r ) => r.clone(),
                                None => c.refresh.clone()
                            };

                            let executor = executor.clone();
                            
                            let node_data = self.data_from_nodes_id( c.cdata.as_ref() );
                            let node_value = self.value_from_nodes_id( c.cvalue.as_ref() );

                            let args = self.state_from_key_path( &c.args );
                            let envs = self.state_from_key_path( &c.envs );

                            let params = CommandExecutorParams::new(
                                doc_id, 
                                node_id,
                                node_data, 
                                node_value,
                                args,
                                envs,
                                refresh, 
                                executor, 
                                ExecutorEventType::CommandChild, 
                                global_cancel, 
                                local_cancel_send
                            );

                            spawn_async_task(
                                async move 
                                {
                                    new_command_executor( params ).await
                                }
                            );
                        }
                    },
                    RTMLNode::Input( _ ) |
                    RTMLNode::Layout( _ ) |
                    RTMLNode::Line( _ ) |
                    RTMLNode::Link( _ ) |
                    RTMLNode::Button( _ ) |
                    RTMLNode::Border( _ ) |
                    RTMLNode::Paragraph( _ ) |
                    RTMLNode::Select( _ ) => {}
                }
            }
        }

        ret
    }

    pub fn state_from_key_path( &self, keys : &HashMap<String, String> ) -> HashMap<String, String>
    {
        let mut ret = HashMap::new();

        keys.iter()
        .flat_map(
            | ( k, v ) |
            {
                Some( ( k, json_value_to_string( self.state.pointer( v )? ) ) )
            }
        )
        .for_each(
            | ( k, v ) |
            {
                ret.insert( k.to_string(), v );
            }
        );

        ret
    }

    pub fn executors_from_ids( &self, ids : &Vec<String> ) -> Option<Vec<Executor>>
    {
        let mut executors = vec![];

        for id in ids
        {
            if let Some( e ) = self.executors.get( id )
            {
                executors.push( e.clone() );
            }
            else
            {
                log_to_file( &format!( "No se encontró el Executor con id {id}" ) );

                return None;    
            }
        }

        Some( executors )
    }

    fn all_childs_ids( &self, node_id : &RTMLNodeId ) -> Vec<String>
    {
        let mut ret = vec![];

        if let Some( node ) = self.doc.get( node_id )
        {
            for child in node.childs()
            {
                ret.push( child.to_string() );

                ret.append( &mut self.all_childs_ids( child ) );
            }
        }

        ret
    }

    pub fn remove_childs_nodes( &mut self, node_id : &RTMLNodeId )
    {
        let childs = match self.doc.get_mut( node_id )
        {
            Some( n ) =>
            {
                let mut childs = vec![];

                std::mem::swap( &mut childs, n.childs_mut() );

                childs
            },
            None => vec![]
        };

        for child in childs
        {
            self.remove_node( &child );
        }
    }

    pub fn remove_node_and_clear_from_parent( &mut self, node_id : &RTMLNodeId, parent_id : &RTMLNodeId )
    {
        if let Some( parent ) = self.doc.get_mut( parent_id )
        {
            let childs = parent.childs_mut();

            childs.retain( | c | c != node_id );
        }

        self.remove_node( node_id );
    }

    fn remove_node( &mut self, node_id : &RTMLNodeId )
    {
        self.cancel_command( node_id );

        self.remove_from_sorted_nodes( node_id );

        match self.doc.get_mut( node_id )
        {
            Some( n ) =>
            {
                let mut childs = vec![];

                std::mem::swap( &mut childs, n.childs_mut() );

                for child in childs
                {
                    self.remove_node( &child );
                }

                self.doc.remove( node_id );
            },
            None => {}
        }
    }

    fn cancel_command( &mut self, node_id : &RTMLNodeId )
    {
        if let Some( token ) = self.cancellation_tokens.get( node_id )
        {
            token.cancel();

            self.cancellation_tokens.remove( node_id );
        }
    }

    fn remove_from_sorted_nodes( &mut self, node_id : &RTMLNodeId )
    {
        match self.sorted_nodes.iter().position( | x | x == node_id )
        {
            Some( index ) =>
            {
                if let Some( f ) = self.focus && f >= index
                {
                    if f == 0
                    {
                        self.focus = None;
                    }
                    else
                    {
                        self.focus = Some( f - 1 );
                    }
                }

                self.sorted_nodes.remove( index );
            },
            None => {}
        }
    }

    pub fn append_child_at_position( 
        &mut self, 
        node_id : RTMLNodeId, 
        child : RTMLNode, 
        child_id : RTMLNodeId,
        position : usize
    )
    {
        match self.doc.get_mut( &node_id )
        {
            Some( n ) =>
            {
                let childs = n.childs_mut();

                let len = childs.len();

                let position = if position > len
                {
                    0
                }
                else
                {
                    position    
                };

                childs.insert( position, child_id.clone() );
                
                self.doc.insert( child_id, child );
            },
            None => {}
        }
    }

    pub fn node_wrapper( &self, node_id : &RTMLNodeId ) -> Option<&XMLNodeWrapper>
    {
        match self.doc.get( node_id )
        {
            Some( n ) => n.node_wrapper(),
            None => None
        }
    }

    pub fn node_template( &self, node_id : &RTMLNodeId ) -> Option<&String>
    {
        match self.doc.get( node_id )
        {
            Some( n ) => n.node_template( &self.templates ),
            None => None
        }
    }

    pub fn command_output( &self, node_id : &RTMLNodeId ) -> RTMLCommandOutput
    {
        match self.doc.get( node_id )
        {
            Some( n ) => n.command_output(),
            None => RTMLCommandOutput::String
        }
    }

    pub fn data_from_nodes_id( &self, nodes_id : &Vec<String> ) -> HashMap<String, String>
    {
        let mut ret = HashMap::new();

        for node_id in nodes_id
        {
            match self.doc.get( node_id )
            {
                Some( n ) =>
                {
                    for ( key, value ) in n.data()
                    {
                        ret.insert( key.to_string(), value.to_string() );
                    }
                },
                None => continue
            }
        }

        ret
    }

    pub fn value_from_nodes_id( &self, nodes_id : &Vec<String> ) -> HashMap<String, String>
    {
        let mut ret = HashMap::new();

        for node_id in nodes_id
        {
            match self.doc.get( node_id )
            {
                Some( n ) =>
                {
                    match n.value()
                    {
                        Some( v ) => ret.insert( node_id.clone(), v.to_string() ),
                        None => continue
                    };
                },
                None => continue
            }
        }

        ret
    }

    pub fn replace_node_value( &mut self, node_id : &str, new_value : String ) -> bool
    {
        if let Some( n ) = self.doc.get_mut( node_id )
        {
            if n.replace_value( new_value )
            {
                self.sincronize_state_from_id( node_id );
                
                true
            }
            else
            {
                false    
            }
        }
        else
        {
            false    
        }
    }
}

impl<'a> From<&'a RTMLDoc> for RTMLDocIterator<'a>
{
    fn from( value: &'a RTMLDoc ) -> Self 
    {
        Self 
        { 
            doc : value, 
            current : None
        }
    }
}

pub struct RTMLDocIterator<'a> 
{
    doc : &'a RTMLDoc,
    current : Option<RTMLNodeId>,
}

impl<'a> Iterator for RTMLDocIterator<'a>
{
    type Item = RTMLNodeId;

    fn next( &mut self ) -> Option<RTMLNodeId>
    {
        match match &self.current
        {
            Some( c ) =>
            {
                find_next( c, &self.doc )
            },
            None =>
            {
                find_first( &self.doc )
            }
        }
        {
            Some( c ) =>
            {
                self.current = Some( c.clone() );

                Some( c )
            },
            None => None
        }
    }
}

fn find_first( doc : &RTMLDoc ) -> Option<RTMLNodeId>
{
    Some( find_next_child( None, doc.node_ref_by_id( &doc.root_id )?.childs(), doc )?.to_string() )
}

fn find_next( current_id : &str, doc : &RTMLDoc ) -> Option<RTMLNodeId>
{
    let node = doc.node_ref_by_id( current_id )?;

    let parent_id = node.parent_id()?;

    let parent = doc.node_ref_by_id( parent_id )?;

    if let Some( n ) = find_next_child( Some( current_id ), parent.childs(), doc )
    {
        Some( n.clone() )
    }
    else
    {
        find_next( parent_id, doc )
    }
}

fn find_next_child<'a>( current_id : Option<&'a str>, childs : &'a Vec<String>, doc : &'a RTMLDoc ) -> Option<&'a RTMLNodeId>
{
    if childs.len() == 0 { return None };

    let next_idx = if let Some( current_id ) = current_id
    {
        childs.iter().enumerate().find_map( 
            | ( idx, id ) |
            {
                if id == current_id
                {
                    Some( idx + 1 )
                }
                else
                {
                    None    
                }
            }
        )
    }
    else
    {
        Some( 0 )    
    }?;

    ( next_idx..childs.len() ).into_iter().find_map(
        | idx |
        {
            let node = doc.node_ref_by_id( &childs[ idx ] );

            if let Some( n ) = node
            {
                if n.is_focusable()
                {
                    Some( &childs[ idx ] )
                }
                else if n.childs().len() > 0
                {
                    find_next_child( None, n.childs(), doc )
                }
                else
                {
                    None    
                }
            }
            else
            {
                None    
            }
        }
    )
}

pub fn render_rtml_doc(
    area : Rect,
    buf : &mut Buffer,
    doc : &mut RTMLDoc
) -> anyhow::Result<()>
{
    render_node(
        &doc.root_id.clone(), 
        area, 
        buf, 
        doc
    )?;

    render_focus( buf, doc )
}

fn render_focus(
    buf : &mut Buffer,
    doc : &RTMLDoc
) -> anyhow::Result<()>
{
    match doc.current_focus()
    {
        Some( n ) => render_focus_node( buf, n ),
        None => Ok( () )
    }
}

fn render_node(
    id : &RTMLNodeId,
    area : Rect,
    buf : &mut Buffer,
    doc : &mut RTMLDoc
) -> anyhow::Result<()>
{
    let childs = change_area_and_get_childs( id, area, doc )?;

    let areas = render_node_and_get_child_areas( id, area, buf, doc )?;

    for i in 0..areas.len()
    {
        let area = areas[ i ];
        let id = &childs[ i ];

        render_node( id, area, buf, doc )?;
    }

    Ok( () )
}

fn change_area_and_get_childs<'a, 'b>(
    id : &'a str,
    area : Rect,
    doc : &'b mut RTMLDoc
) -> anyhow::Result<Vec<RTMLNodeId>>
{
    let root = doc.node_mut_by_id( &id ).ok_or(
        anyhow::Error::msg( format!( "No se encontró el nodo con id {id} en change_area_and_get_childs" ) )
    )?;

    root.set_area( area );

    // TODO: Hay que ver como hacer esto mejor, estamos creando el Paragraph 2 veces, una aquí para guardar las líneas y otra cuando lo renderizamos
    match root
    {
        RTMLNode::Paragraph( rtml_paragraph ) =>
        {
            let p = create_paragraph( rtml_paragraph );

            let inner_area = area_con_padding( area, &rtml_paragraph.padding );

            let num_lines = p.line_count( inner_area.width );

            rtml_paragraph.num_lines = num_lines;
            rtml_paragraph.inner_area = inner_area;
        },
        RTMLNode::Select( rtml_select ) =>
        {
            rtml_select.inner_area = area_con_padding( area, &rtml_select.padding );
        }
        _ => {}
    };

    Ok( root.childs().clone() )
}

fn render_node_and_get_child_areas(
    id : &str,
    area : Rect,
    buf : &mut Buffer,
    doc : &RTMLDoc
) -> anyhow::Result<Vec<Rect>>
{
    let root = doc.node_ref_by_id( &id ).ok_or(
        anyhow::Error::msg( format!( "No se encontró el nodo con id {id} en render_node_and_get_child_areas" ) )
    )?;
        
    match &root
    {
        RTMLNode::Layout( l ) =>
        {
            render_rtml_layout( l, area, buf );
            
            child_areas( root.childs(), &l.container.direction, &l.container.flex, &l.container.padding, area, doc )
        },
        RTMLNode::Border( b ) =>
        {
            let inner_area = render_rtml_border( b, area, buf );

            child_areas( root.childs(), &b.container.direction, &b.container.flex, &b.container.padding, inner_area, doc )
        },
        RTMLNode::Command( c ) =>
        {
            render_rtml_command( c, area, buf );

            child_areas( root.childs(), &c.container.direction, &c.container.flex, &c.container.padding, area, doc )
        },
        RTMLNode::Paragraph( p ) =>
        {
            render_rtml_paragraph( p, area, buf )?;

            Ok( vec![] )
        },
        RTMLNode::Select( s ) =>
        {
            render_rtml_select( s, buf )?;

            Ok( vec![] )
        },
        RTMLNode::Line( l ) =>
        {
            render_rtml_line( l, area, buf )?;

            Ok( vec![] )
        },
        RTMLNode::Input( i ) =>
        {
            render_rtml_input( i, area, buf )?;

            Ok( vec![] )
        },
        RTMLNode::Link( l ) =>
        {
            render_rtml_link( l, area, buf )?;

            Ok( vec![] )
        },
        RTMLNode::Button( b ) =>
        {
            render_rtml_button( b, area, buf )?;

            Ok( vec![] )
        }
    }
}

fn child_areas(
    childs : &Vec<RTMLNodeId>,
    direction : &Direction,
    flex : &Flex,
    padding : &RTMLPadding,
    area : Rect,
    doc : &RTMLDoc
) -> anyhow::Result<Vec<Rect>>
{
    let area = area_con_padding( area, padding );

    let constraints = childs_constraint( childs, doc )?;

    // Creo que no necesito esto, de momento lo quito
    // let childs_len = childs.len();

    // let constraints = match direction
    // {
    //     Direction::Horizontal =>
    //     {
    //         if childs_len > area.width as usize
    //         {
    //             &constraints[ 0..( area.width as usize ) ]
    //         }
    //         else
    //         {
    //             constraints.as_slice()
    //         }
    //     },
    //     Direction::Vertical =>
    //     {
    //         if childs_len > area.height as usize
    //         {
    //             &constraints[ 0..( area.height as usize ) ]
    //         }
    //         else
    //         {
    //             constraints.as_slice()
    //         }
    //     }
    // };

    let areas = calc_areas( area, direction, flex, constraints.as_slice() );

    Ok( areas )
}

fn area_con_padding( mut area : Rect, padding : &RTMLPadding ) -> Rect
{
    let top_y_bottom = padding.vertical.top + padding.vertical.bottom;

    if top_y_bottom < area.height as usize
    {
        area.y = area.y + padding.vertical.top as u16;

        area.height = area.height - top_y_bottom as u16;
    }

    let left_y_right = padding.horizontal.left + padding.horizontal.right;

    if left_y_right < area.width as usize
    {
        area.x = area.x + padding.horizontal.left as u16;

        area.width = area.width - left_y_right as u16;
    }

    area
}

fn childs_constraint<'a, 'b>(
    childs : &'a Vec<RTMLNodeId>,
    doc : &'b RTMLDoc
) -> anyhow::Result<Vec<&'b Constraint>>
{
    let mut constraints : Vec<&Constraint> = vec![];

    for child in childs
    {
        constraints.push( &doc.node_ref_by_id( &child ).ok_or(
                anyhow::Error::msg( format!( "No se encontró el nodo con id {child} en childs_constraint" ) )
            )?
            .constraint()
        );
    }

    Ok( constraints )
}

fn calc_areas(
    container : Rect,
    direction : &Direction,
    flex : &Flex,
    constraints : &[&Constraint]
) -> Vec<Rect>
{
    match direction
    {
        Direction::Horizontal =>
        {
            container.layout_vec( &Layout::horizontal( constraints.to_vec() ).flex( *flex ) ).to_vec()
        },
        Direction::Vertical =>
        {
            container.layout_vec( &Layout::vertical( constraints.to_vec() ).flex( *flex ) ).to_vec()
        }
    }
}
