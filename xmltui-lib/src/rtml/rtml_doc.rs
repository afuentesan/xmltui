use std::collections::HashMap;

use ratatui::{buffer::Buffer, layout::{Constraint, Direction, Flex, Layout, Rect}, style::Style};
use tokio_util::sync::CancellationToken;

use crate::{async_app::async_app::spawn_async_task, code::{event::{CommandExecutorParams, ExecutorEventType, new_command_executor}, executor::Executor}, input::event::InputEvent, rtml::{rtml_border::render_rtml_border, rtml_button::render_rtml_button, rtml_command::RTMLCommandOutput, rtml_input::render_rtml_input, rtml_layout::render_rtml_layout, rtml_line::render_rtml_line, rtml_link::render_rtml_link, rtml_node::{RTMLNode, RTMLNodeId, XMLNodeWrapper, render_focus_node}}, util::log::log_to_file, xml::styles::xml_style::StyleSelector};

#[derive(Debug)]
pub struct RTMLDoc 
{
    pub doc_id : RTMLNodeId,
    pub doc : HashMap<RTMLNodeId, RTMLNode>,
    pub root_id : RTMLNodeId,
    pub focus : Option<usize>,
    pub sorted_nodes : Vec<RTMLNodeId>,
    pub style : Style,
    pub styles : HashMap<StyleSelector, Style>,
    pub executors : HashMap<String, Executor>,
    pub cancellation_tokens : HashMap<String, CancellationToken>,
    pub templates : HashMap<String, String>
}

impl RTMLDoc
{
    pub fn new(
        styles : HashMap<StyleSelector, Style>,
        executors : HashMap<String, Executor>,
        templates : HashMap<String, String>
    ) -> Self
    {
        let mut doc = Self::empty();

        doc.styles = styles;
        doc.executors = executors;
        doc.templates = templates;

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
            templates : HashMap::new()
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
                n.focus_event( event )
            },
            None => false
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
                            let local_cancel = CancellationToken::new();
                            let local_cancel_send = local_cancel.clone();

                            self.cancellation_tokens.insert( node_id.clone(), local_cancel );


                            let doc_id = self.doc_id.clone();
                            let node_id = node_id.clone();
                            let refresh = c.refresh.clone();
                            let executor = executor.clone();
                            let data = node.data().clone();

                            let params = CommandExecutorParams::new(
                                doc_id, 
                                node_id,
                                data, 
                                refresh, 
                                executor, 
                                ExecutorEventType::CommandChild, 
                                Some( global_cancel ), 
                                Some( local_cancel_send )
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
                    RTMLNode::Span( _ ) => {}
                }
            }
        }
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

    pub fn replace_node_value( &mut self, node_id : &str, new_value : String ) -> bool
    {
        if let Some( n ) = self.doc.get_mut( node_id )
        {
            n.replace_value( new_value )
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

    // TODO: Probablemente aquí haya que ver si hay que hacer scroll. Si la longitud de childs es mayor que areas habrá que hacer scroll

    // if childs.len() > areas.len()
    // {
    //     log_to_file( &format!( "CL: {}, AL: {}", childs.len(), areas.len() ) );
    // }

    for i in 0..areas.len()
    {
        let area = areas[ i ];
        let id = &childs[ i ];

        render_node( id, area, buf, doc )?;
    }

    Ok( () )
}

fn change_area_and_get_childs(
    id : &str,
    area : Rect,
    doc : &mut RTMLDoc
) -> anyhow::Result<Vec<RTMLNodeId>>
{
    let root = doc.node_mut_by_id( &id ).ok_or(
        anyhow::Error::msg( format!( "No se encontró el nodo con id {id} en change_area_and_get_childs" ) )
    )?;

    root.set_area( area );

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
            
            child_areas( root.childs(), &l.direction, &l.flex, area, doc )
        },
        RTMLNode::Border( b ) =>
        {
            let inner_area = render_rtml_border( b, area, buf );

            child_areas( root.childs(), &b.direction, &b.flex, inner_area, doc )
        },
        RTMLNode::Line( l ) =>
        {
            render_rtml_line( l, root.childs(), area, buf, doc )?;

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
        },
        RTMLNode::Command( c ) =>
        {
            child_areas( root.childs(), &c.direction, &c.flex, area, doc )
        },
        RTMLNode::Span( _ ) =>
        {
            Err( anyhow::Error::msg( "Span not expected" ) )
        }
    }
}

fn child_areas(
    childs : &Vec<RTMLNodeId>,
    direction : &Direction,
    flex : &Flex,
    area : Rect,
    doc : &RTMLDoc
) -> anyhow::Result<Vec<Rect>>
{
    let constraints = childs_constraint( childs, doc )?;

    let childs_len = childs.len();

    let constraints = match direction
    {
        Direction::Horizontal =>
        {
            if childs_len > area.width as usize
            {
                &constraints[ 0..( area.width as usize ) ]
            }
            else
            {
                constraints.as_slice()
            }
        },
        Direction::Vertical =>
        {
            if childs_len > area.height as usize
            {
                &constraints[ 0..( area.height as usize ) ]
            }
            else
            {
                constraints.as_slice()
            }
        }
    };

    let areas = calc_areas( area, direction, flex, constraints );

    Ok( areas )
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
