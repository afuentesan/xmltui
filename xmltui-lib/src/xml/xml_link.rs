use roxmltree::Node;

use crate::{rtml::{rtml_link::RTMLLink, rtml_node::{RTMLNode, RTMLNodeCommon, RTMLNodeId}}, xml::{attrs::{attr_source, id_retry_if_exists, parse_common_attrs}, styles::{default_styles::default_focus_style , xml_style::StyleVariant}, xml_doc::{XMLDoc, replace_xml_doc_focus}, xml_line::process_text_line, xml_util::{paragraph_like_styles, style_from_styles}}};


pub fn process_link( 
    xml_doc : &mut XMLDoc,
    node : Node, 
    parent_id : Option<RTMLNodeId>
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    let id = id_retry_if_exists( node, xml_doc.nodos() );
    
    let ( constraint, style, style_template, padding, alignment ) = paragraph_like_styles( node, xml_doc.styles(), None );
    
    let padding = padding.horizontal;

    let ( focus_style, focus_style_template ) = style_from_styles( node, xml_doc.styles(), Some( StyleVariant::Focus ), Some( default_focus_style( &style ) ) );

    let common = RTMLNodeCommon::new( 
        parse_common_attrs( constraint )?, 
        vec![], 
        parent_id
    );

    let source = attr_source( node )?;

    let text = process_text_line( node, xml_doc.styles() );
    
    replace_xml_doc_focus( xml_doc, node, &id );

    Ok(
        (
            RTMLNode::Link(
                RTMLLink::new( 
                    alignment, 
                    source,
                    padding,
                    text,
                    style,
                    style_template,
                    focus_style,
                    focus_style_template,
                    common
                )
            ),
            id
        )
    )
}