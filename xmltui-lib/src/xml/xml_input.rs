use std::collections::HashMap;

use ratatui::style::Style;
use roxmltree::Node;

use crate::{rtml::{rtml_input::RTMLInput, rtml_node::{RTMLNode, RTMLNodeCommon, RTMLNodeId}, util::rtml_value::{RTMLValue, RTMLValueAttrs, RTMLValueType}}, xml::{attrs::{attr_alignment, attr_value, id_retry_if_exists, parse_common_attrs}, styles::{default_styles::{default_focus_style, default_normal_style}, xml_style::{StyleSelector, StyleVariant, style_from_node}}}};

const DEFAULT_INPUT_TYPE : &str = "text";

pub fn process_input( 
    node : Node, 
    nodos : &mut HashMap<String, RTMLNode>, 
    parent_id : Option<RTMLNodeId>,
    styles : &HashMap<StyleSelector, Style> 
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    let id = id_retry_if_exists( &node, nodos );
    
    let alignment = attr_alignment( &node )?;

    let value = parse_input_value( &node )?;

    Ok(
        (
            RTMLNode::Input(
                RTMLInput::new( 
                    alignment, 
                    value, 
                    style_from_node( node, styles, default_normal_style(), None ), 
                    style_from_node( node, styles, default_focus_style(), Some( StyleVariant::Focus ) ), 
                    RTMLNodeCommon::new( 
                        parse_common_attrs( &node )?, 
                        vec![], 
                        parent_id
                    )
                )
            ),
            id
        )
    )
}

fn parse_input_value( node : &Node ) -> anyhow::Result<RTMLValue>
{
    match node.attribute( "type" )
    {
        Some( t ) => parse_input_type( t, node ),
        None => parse_input_type( DEFAULT_INPUT_TYPE, node )
    }
}

fn parse_input_type( itype : &str, node : &Node ) -> anyhow::Result<RTMLValue>
{
    let itype = itype.trim();

    match itype
    {
        "" | DEFAULT_INPUT_TYPE => Ok( 
            RTMLValue::Write( 
                RTMLValueAttrs::new(
                    Some( 1 ),
                    RTMLValueType::new_string(
                        attr_value( node ),
                        true,
                        false
                    )
                )
            ) 
        ),
        _ => Err( anyhow::Error::msg( format!( "Input type not valid. Type: {}", itype ) ) )
    }
}