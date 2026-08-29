use roxmltree::Node;

use crate::{rtml::{rtml_link::RTMLLink, rtml_node::{RTMLNode, RTMLNodeCommon, RTMLNodeId}}, xml::{attrs::{attr_alignment, attr_source, id_retry_if_exists, parse_common_attrs}, styles::{default_styles::{default_link_focus_style, default_link_normal_style}, xml_style::{StyleVariant, style_from_node}}, xml_doc::XMLDoc}};


pub fn process_link( 
    xml_doc : &mut XMLDoc,
    node : Node, 
    // nodos : &mut HashMap<String, RTMLNode>, 
    parent_id : Option<RTMLNodeId>,
    // styles : &HashMap<StyleSelector, Style> 
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    let id = id_retry_if_exists( node, xml_doc.nodos() );
    
    let alignment = attr_alignment( node )?;

    let text = node.text().unwrap_or( " " ).to_string();

    let source = attr_source( node )?;

    Ok(
        (
            RTMLNode::Link(
                RTMLLink::new( 
                    alignment, 
                    source,
                    text,
                    style_from_node( node, xml_doc.styles(), default_link_normal_style(), None ),
                    style_from_node( node, xml_doc.styles(), default_link_focus_style(),Some( StyleVariant::Focus ) ),
                    RTMLNodeCommon::new( 
                        parse_common_attrs( node )?, 
                        vec![], 
                        parent_id
                    )
                )
            ),
            id
        )
    )
}