use roxmltree::Node;

use crate::{rtml::{rtml_layout::RTMLLayout, rtml_node::{RTMLNode, RTMLNodeCommon, RTMLNodeId}}, xml::{attrs::{id_retry_if_exists, parse_common_attrs}, xml_doc::XMLDoc, xml_util::container_styles, xml2rtml::process_node}};

pub fn process_body_layout( 
    xml_doc : &mut XMLDoc,
    node : Node, 
    parent_id : Option<RTMLNodeId>, 
    xml : &str 
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    process_container( xml_doc, node, parent_id, xml )
}

pub fn process_layout( 
    xml_doc : &mut XMLDoc,
    node : Node, 
    parent_id : Option<RTMLNodeId>, 
    xml : &str 
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    process_container( xml_doc, node, parent_id, xml )
}

fn process_container( 
    xml_doc : &mut XMLDoc,
    node : Node, 
    parent_id : Option<RTMLNodeId>, 
    xml : &str
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    let layout_id = id_retry_if_exists( node, xml_doc.nodos() );

    let mut childs : Vec<RTMLNodeId> = vec![];

    for c in node.children()
    {
        match process_node( xml_doc, c, Some( layout_id.clone() ), xml )?
        {
            Some( ( n, id ) ) =>
            {
                xml_doc.add_node( n, id.clone() );

                childs.push( id );
            },
            None => {}
        }
    }

    let ( constraint, style, style_template, container_attrs ) = container_styles( node, xml_doc.styles(), None );

    Ok(
        (
            RTMLNode::Layout(
                RTMLLayout::new( 
                    RTMLNodeCommon::new( 
                        parse_common_attrs( constraint )?, 
                        childs, 
                        parent_id
                    ),
                    container_attrs,
                    Some( style ),
                    style_template
                )
            ),
            layout_id
        )
    )
}

