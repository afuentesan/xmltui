use roxmltree::Node;

use crate::{app::app_doc::chroot, rtml::{rtml_doc::RTMLDoc, rtml_node::{RTMLNode, RTMLNodeId}}, util::file::read_file_in_chroot_with_extension, xml::{attrs::id_retry_if_exists, styles::xml_style::styles_from_head, xml_border::process_border, xml_button::process_button, xml_code::code_from_parent, xml_command::process_command, xml_container::process_childs_container, xml_doc::XMLDoc, xml_input::process_input, xml_layout::{process_body_layout, process_layout}, xml_line::process_line, xml_link::process_link, xml_paragraph::process_paragraph, xml_select::process_select, xml_template::templates_from_parent}};

pub fn xml2rtml_doc( path : &str ) -> anyhow::Result<RTMLDoc>
{
    let xml = read_file_in_chroot_with_extension( path, chroot(), "xml" )?;

    let doc = roxmltree::Document::parse(xml.as_str() )?;

    let body = find_body( doc.root_element() )?;

    let head = find_head( doc.root_element() );

    let styles = styles_from_head( head )?;

    let executors = code_from_parent( head )?;

    let templates = templates_from_parent( head, &xml )?;

    let mut rtml_doc = RTMLDoc::new( styles, executors, templates );

    let ( root, root_id ) = process_first_node( 
        body, 
        &mut rtml_doc, 
        None,
        &xml
    )?
    .ok_or( anyhow::Error::msg( "No root element" ) )?;

    rtml_doc.doc.insert( root_id.clone(), root );

    rtml_doc.root_id = root_id;

    rtml_doc.doc_id = id_retry_if_exists( doc.root_element(), &rtml_doc.doc );

    rtml_doc.sort_nodes();

    Ok( rtml_doc )
}

pub fn replace_node_with_xml(
    rtml_doc : &mut RTMLDoc,
    node_id : RTMLNodeId,
    xml : String
) -> anyhow::Result<RTMLNodeId>
{
    let ( parent_id, position ) = match rtml_doc.doc.get( &node_id )
    {
        Some( n ) if n.parent_id().is_some() => 
        {
            let parent_id = n.parent_id().unwrap().clone();

            let position = match rtml_doc.doc.get( &parent_id )
            {
                Some( p ) =>
                {
                    p.childs()
                    .iter()
                    .enumerate()
                    .find(
                        | ( _, c ) |
                        {
                            *c == &node_id
                        }
                    )
                    .map(
                        | ( i, _ ) |
                        {
                            i
                        }
                    )
                    .unwrap_or( 0 )
                },
                None => 0
            };

            ( parent_id, position )
        },
        _ =>
        {
            return Err( anyhow::Error::msg( format!( "No existe el nodo con id {node_id}" ) ) ); 
        }
    };

    let doc = roxmltree::Document::parse(xml.as_str() )?;

    rtml_doc.remove_node_and_clear_from_parent( &node_id, &parent_id );

    let ( root, root_id ) = process_first_node( 
        doc.root_element(), 
        rtml_doc, 
        Some( parent_id.clone() ),
        &xml
    )?
    .ok_or( anyhow::Error::msg( "No root element" ) )?;

    rtml_doc.append_child_at_position( parent_id, root, root_id.clone(), position );
    
    rtml_doc.sort_nodes();

    Ok( root_id )
}

pub fn replace_node_childs_with_xml( 
    rtml_doc : &mut RTMLDoc,
    node_id : RTMLNodeId,
    xml : &str
) -> anyhow::Result<()>
{
    if ! rtml_doc.doc.contains_key( &node_id ) 
    { 
        return Err( anyhow::Error::msg( format!( "No existe el nodo con id {node_id}" ) ) ); 
    };

    let xml = format!( "<container>{xml}</container>" );

    let doc = roxmltree::Document::parse(&xml )?;

    rtml_doc.remove_childs_nodes( &node_id );

    process_first_node( 
        doc.root_element(), 
        rtml_doc, 
        Some( node_id.clone() ),
        &xml
    )?;
    //.ok_or( anyhow::Error::msg( "No root element" ) )?;

    // rtml_doc.append_child( node_id, root, root_id );
    
    rtml_doc.sort_nodes();

    Ok( () )
}

fn find_head<'a, 'input>( node : Node<'a, 'input> ) -> Option<Node<'a, 'input>>
{
    node.children().find(
        | n | n.tag_name().name() == "head"
    )
}

fn find_body<'a, 'input>( node : Node<'a, 'input> ) -> anyhow::Result<Node<'a, 'input>>
{
    node.children().find(
        | n | n.tag_name().name() == "body"
    )
    .ok_or( anyhow::Error::msg( "body not found" ) )
}

fn process_first_node(
    node : Node, 
    rtml_doc : &mut RTMLDoc,
    parent_id : Option<RTMLNodeId>,
    xml : &str
) -> anyhow::Result<Option<( RTMLNode, RTMLNodeId )>>
{
    let mut xml_doc = XMLDoc::new(
        &mut rtml_doc.doc, 
        &rtml_doc.styles, 
        None
    );

    // process_node( node, &mut rtml_doc.doc, parent_id, &rtml_doc.styles, xml )
    process_node( &mut xml_doc, node, parent_id, xml )
}

pub fn process_node( 
    xml_doc : &mut XMLDoc,
    node : Node, 
    // nodos : &mut HashMap<String, RTMLNode>,
    parent_id : Option<RTMLNodeId>,
    // styles : &HashMap<StyleSelector, Style>,
    xml : &str
) -> anyhow::Result<Option<( RTMLNode, RTMLNodeId )>>
{
    match node.tag_name().name()
    {
        "layout" =>
        {
            Ok( Some( process_layout( xml_doc, node, parent_id, xml )? ) )
        },
        "body" =>
        {
            Ok( Some( process_body_layout( xml_doc, node, parent_id, xml )? ) )
        },
        "p" =>
        {
            Ok( Some( process_paragraph( xml_doc, node, parent_id )? ) )
        },
        "select" =>
        {
            Ok( Some( process_select( xml_doc, node, parent_id )? ) )
        },
        "line" =>
        {
            Ok( Some( process_line( xml_doc, node, parent_id )? ) )
        },
        "input" =>
        {
            Ok( Some( process_input( xml_doc, node, parent_id )? ) )
        },
        "a" =>
        {
            Ok( Some( process_link( xml_doc, node, parent_id )? ) )
        },
        "button" =>
        {
            Ok( Some( process_button( xml_doc, node, parent_id )? ) )
        },
        "command" =>
        {
            Ok( Some( process_command( xml_doc, node, parent_id, xml )? ) )
        },
        "border" =>
        {
            Ok( Some( process_border( xml_doc, node, parent_id, xml )? ) )
        },
        "container" =>
        {
            if parent_id.is_none() || xml_doc.nodos().get( parent_id.as_ref().unwrap() ).is_none()
            {
                return Err( anyhow::Error::msg( "El nodo container siempre debe tener un parent_id y debe estar ya entre los nodos." ) )
            }

            process_childs_container( xml_doc, node, parent_id.unwrap(), xml )?;

            Ok( None )
        },
        _ => Ok( None )
    }
}

#[cfg(test)]
mod test
{
    use crate::{app::app_doc::init_chroot, util::log::log_to_file};

use super::*;

    fn common()
    {
        init_chroot( "./" ).unwrap();
    }

    #[test]
    fn test_parent_id()
    {
        common();
        
        let doc = xml2rtml_doc( "./examples/example1.xml" );

        if doc.is_err()
        {
            let err = format!( "Err: {:?}", doc.err() );

            log_to_file( &format!( "Err: {err}" ) );

            assert!( false, "rtml is err" );

            return;
        }

        assert!( doc.is_ok() );

        let mut doc = doc.unwrap();

        assert_eq!( doc.node_ref_by_id( "root" ).unwrap().parent_id(), None );
        assert_eq!( doc.node_ref_by_id( "input_container" ).unwrap().parent_id(), Some( &"root".to_string() ) );
        assert_eq!( doc.node_ref_by_id( "line_50" ).unwrap().parent_id(), Some( &"root".to_string() ) );
        assert_eq!( doc.node_ref_by_id( "input_1" ).unwrap().parent_id(), Some( &"input_container".to_string() ) );
        assert_eq!( doc.node_ref_by_id( "input_2" ).unwrap().parent_id(), Some( &"input_container".to_string() ) );

        assert_eq!( doc.sorted_nodes.len(), 10 );

        assert_eq!( doc.sorted_nodes[ 0 ], "input_1" );
        assert_eq!( doc.sorted_nodes[ 1 ], "input_2" );
        assert_eq!( doc.sorted_nodes[ 2 ], "input_3" );
        assert_eq!( doc.sorted_nodes[ 3 ], "input_4" );
        assert_eq!( doc.sorted_nodes[ 4 ], "input_5" );
        assert_eq!( doc.sorted_nodes[ 5 ], "input_5_1" );
        assert_eq!( doc.sorted_nodes[ 6 ], "input_5_2" );
        assert_eq!( doc.sorted_nodes[ 7 ], "input_5_3" );
        assert_eq!( doc.sorted_nodes[ 8 ], "input_6" );
        assert_eq!( doc.sorted_nodes[ 9 ], "input_7" );

        assert!( doc.current_focus().is_none() );
        assert!( current_focus_id( &doc ).is_none() );

        doc.focus_next();

        assert!( doc.current_focus().is_some() );
        assert!( current_focus_id( &doc ).is_some() );

        assert_eq!( current_focus_id( &doc ).unwrap(), "input_1" );

        doc.focus_back();

        assert!( current_focus_id( &doc ).is_some() );

        assert_eq!( current_focus_id( &doc ).unwrap(), "input_7" );

        doc.focus_next();

        assert!( doc.current_focus().is_some() );
        assert!( current_focus_id( &doc ).is_some() );

        assert_eq!( current_focus_id( &doc ).unwrap(), "input_1" );

        doc.focus_next();

        assert!( doc.current_focus().is_some() );
        assert!( current_focus_id( &doc ).is_some() );

        assert_eq!( current_focus_id( &doc ).unwrap(), "input_2" );

        doc.focus_back();

        assert!( current_focus_id( &doc ).is_some() );

        assert_eq!( current_focus_id( &doc ).unwrap(), "input_1" );

        doc.focus_back();

        assert!( current_focus_id( &doc ).is_some() );

        assert_eq!( current_focus_id( &doc ).unwrap(), "input_7" );

        doc.focus_back();

        assert!( current_focus_id( &doc ).is_some() );

        assert_eq!( current_focus_id( &doc ).unwrap(), "input_6" );

        focus_id( &mut doc, "input_5_1" );

        assert!( current_focus_id( &doc ).is_some() );

        assert_eq!( current_focus_id( &doc ).unwrap(), "input_5_1" );

        focus_id( &mut doc, "input_5_1_nooo" );

        assert!( current_focus_id( &doc ).is_some() );

        assert_eq!( current_focus_id( &doc ).unwrap(), "input_5_1" );

        doc.focus_back();

        assert!( current_focus_id( &doc ).is_some() );

        assert_eq!( current_focus_id( &doc ).unwrap(), "input_5" );

    }

    fn current_focus_id( doc : &RTMLDoc ) -> Option<&str>
    {
        if let Some( idx ) = doc.focus && idx < doc.sorted_nodes.len()
        {
            match doc.node_ref_by_id( &doc.sorted_nodes[ idx ] )
            {
                Some( _ ) => Some( &doc.sorted_nodes[ idx ] ),
                None => None
            }
        }
        else
        {
            None    
        }
    }

    fn focus_id( doc : &mut RTMLDoc, id : &str )
    {
        let idx = doc.sorted_nodes
        .iter()
        .enumerate()
        .find_map(
            | ( idx, node_id ) |
            {
                if node_id == id
                {
                    Some( idx )
                }
                else
                {
                    None    
                }
            }
        );

        if let Some( idx ) = idx
        {
            doc.focus = Some( idx );
        }
    }
}
