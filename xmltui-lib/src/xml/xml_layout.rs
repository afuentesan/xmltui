use ratatui::style::Style;
use roxmltree::Node;

use crate::{rtml::{rtml_layout::RTMLLayout, rtml_node::{RTMLNode, RTMLNodeCommon, RTMLNodeId}}, xml::{attrs::{container_attrs, id_retry_if_exists, parse_common_attrs}, styles::{default_styles::default_normal_style, xml_style::style_from_container}, xml_doc::XMLDoc, xml2rtml::process_node}};

pub fn process_body_layout( 
    xml_doc : &mut XMLDoc,
    node : Node, 
    // nodos : &mut HashMap<String, RTMLNode>, 
    parent_id : Option<RTMLNodeId>, 
    // styles : &HashMap<StyleSelector, Style>,
    xml : &str 
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    process_container( xml_doc, node, parent_id, default_normal_style(), xml )
}

pub fn process_layout( 
    xml_doc : &mut XMLDoc,
    node : Node, 
    // nodos : &mut HashMap<String, RTMLNode>, 
    parent_id : Option<RTMLNodeId>, 
    // styles : &HashMap<StyleSelector, Style>,
    xml : &str 
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    process_container( xml_doc, node, parent_id, Style::default(), xml )
}

fn process_container( 
    xml_doc : &mut XMLDoc,
    node : Node, 
    // nodos : &mut HashMap<String, RTMLNode>, 
    parent_id : Option<RTMLNodeId>, 
    // styles : &HashMap<StyleSelector, Style>,
    default_style : Style,
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

    Ok(
        (
            RTMLNode::Layout(
                RTMLLayout::new( 
                    RTMLNodeCommon::new( 
                        parse_common_attrs( node )?, 
                        childs, 
                        parent_id
                    ),
                    container_attrs( node )?,
                    style_from_container( node, xml_doc.styles(), default_style )
                )
            ),
            layout_id
        )
    )
}