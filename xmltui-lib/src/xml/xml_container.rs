use std::collections::HashMap;

use roxmltree::Node;

use crate::{rtml::rtml_node::{RTMLNode, RTMLNodeId}, xml::{xml_doc::XMLDoc, xml2rtml::process_node}};


pub fn process_childs_container(
    xml_doc : &mut XMLDoc, 
    node : Node,  
    parent_id : RTMLNodeId, 
    xml : &str 
) -> anyhow::Result<()>
{
    for c in node.children()
    {
        match process_node( xml_doc, c, Some( parent_id.clone() ), xml )?
        {
            Some( ( n, id ) ) =>
            {
                xml_doc.add_node( n, id.clone() );

                append_child( xml_doc.nodos_mut(), &parent_id, id )?;
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