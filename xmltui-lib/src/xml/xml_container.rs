use std::collections::HashMap;

use ratatui::style::Style;
use roxmltree::Node;

use crate::{rtml::rtml_node::{RTMLNode, RTMLNodeId}, xml::{styles::xml_style::StyleSelector, xml2rtml::process_node}};


pub fn process_childs_container(
    node : Node, 
    nodos : &mut HashMap<String, RTMLNode>, 
    parent_id : RTMLNodeId, 
    styles : &HashMap<StyleSelector, Style>,
    xml : &str 
) -> anyhow::Result<()>
{
    for c in node.children()
    {
        match process_node( c, nodos, Some( parent_id.clone() ), styles, xml )?
        {
            Some( ( n, id ) ) =>
            {
                nodos.insert( id.clone(), n );

                append_child( nodos, &parent_id, id )?;
            },
            None => {}
        }
    }

    Ok( () )
}

fn append_child(
    nodos : &mut HashMap<String, RTMLNode>, 
    node_id : &RTMLNodeId, 
    child_id : RTMLNodeId
) -> anyhow::Result<()>
{
    if let Some( nodo ) = nodos.get_mut( node_id )
    {
        nodo.childs_mut().push( child_id );

        Ok( () )
    }
    else
    {
        Err( 
            anyhow::Error::msg(
                format!( "Se esperaba que {node_id} estuviese ya en los nodos." )
            )
        )    
    }
}