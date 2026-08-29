use roxmltree::Node;

use crate::{rtml::{rtml_link::RTMLLink, rtml_node::{RTMLNode, RTMLNodeCommon, RTMLNodeId}}, xml::{attrs::{attr_alignment, attr_source, id_retry_if_exists, parse_common_attrs}, styles::{default_styles::{default_link_focus_style, default_link_normal_style}, xml_style::{StyleVariant, style_from_node}}, xml_doc::{XMLDoc, replace_xml_doc_focus}}};


pub fn process_link( 
    xml_doc : &mut XMLDoc,
    node : Node, 
    parent_id : Option<RTMLNodeId>
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    let id = id_retry_if_exists( node, xml_doc.nodos() );
    
    let alignment = attr_alignment( node )?;

    let text = node.text().unwrap_or( " " ).to_string();

    let source = attr_source( node )?;

    replace_xml_doc_focus( xml_doc, node, &id );

    let style = style_from_node( node, xml_doc.styles(), default_link_normal_style(), None );

    let focus_style = style_from_node( node, xml_doc.styles(), default_link_focus_style( &style ),Some( StyleVariant::Focus ) );

    Ok(
        (
            RTMLNode::Link(
                RTMLLink::new( 
                    alignment, 
                    source,
                    text,
                    style,
                    focus_style,
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