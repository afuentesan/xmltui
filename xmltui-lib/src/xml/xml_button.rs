use std::collections::HashMap;

use ratatui::style::Style;
use roxmltree::Node;

use crate::{rtml::{rtml_button::RTMLButton, rtml_node::{RTMLNode, RTMLNodeCommon, RTMLNodeId}}, xml::{attrs::{attr_alignment, id_retry_if_exists, parse_common_attrs}, styles::{default_styles::{default_link_focus_style, default_link_normal_style}, xml_style::{StyleSelector, StyleVariant, style_from_node}}, xml_event::parse_event_attrs}};


pub fn process_button( 
    node : Node, 
    nodos : &mut HashMap<String, RTMLNode>, 
    parent_id : Option<RTMLNodeId>,
    styles : &HashMap<StyleSelector, Style> 
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    let id = id_retry_if_exists( node, nodos );
    
    let alignment = attr_alignment( node )?;

    let text = node.text().unwrap_or( " " ).to_string();

    Ok(
        (
            RTMLNode::Button(
                RTMLButton::new( 
                    alignment, 
                    parse_event_attrs( node, &id )?,
                    text,
                    style_from_node( node, styles, default_link_normal_style(), None ),
                    style_from_node( node, styles, default_link_focus_style(),Some( StyleVariant::Focus ) ),
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