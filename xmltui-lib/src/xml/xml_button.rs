use roxmltree::Node;

use crate::{rtml::{rtml_button::RTMLButton, rtml_node::{RTMLNode, RTMLNodeCommon, RTMLNodeId}}, xml::{attrs::{id_retry_if_exists, parse_common_attrs}, styles::{default_styles::default_focus_style , xml_style::StyleVariant}, xml_doc::{XMLDoc, replace_xml_doc_focus}, xml_event::parse_event_attrs, xml_util::{input_like_styles, style_from_styles}}};


pub fn process_button( 
    xml_doc : &mut XMLDoc,
    node : Node, 
    parent_id : Option<RTMLNodeId>
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    let id = id_retry_if_exists( node, xml_doc.nodos() );
    
    let ( constraint, style, alignment ) = input_like_styles( node, xml_doc.styles(), None );
    
    let focus_style = style_from_styles( node, xml_doc.styles(), Some( StyleVariant::Focus ), Some( default_focus_style( &style ) ) );

    let common = RTMLNodeCommon::new( 
        parse_common_attrs( constraint )?, 
        vec![], 
        parent_id
    );

    let text = node.text().unwrap_or( " " ).to_string();

    replace_xml_doc_focus( xml_doc, node, &id );

    Ok(
        (
            RTMLNode::Button(
                RTMLButton::new( 
                    alignment, 
                    parse_event_attrs( node )?,
                    text,
                    style,
                    focus_style,
                    common
                )
            ),
            id
        )
    )
}