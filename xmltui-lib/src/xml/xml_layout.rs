use std::collections::HashMap;

use ratatui::style::Style;
use roxmltree::Node;

use crate::{rtml::{rtml_layout::RTMLLayout, rtml_node::{RTMLNode, RTMLNodeCommon, RTMLNodeId}}, xml::{attrs::{attr_direction, id_retry_if_exists, parse_common_attrs}, styles::{default_styles::default_normal_style, xml_style::{StyleSelector, style_from_container}}, xml2rtml::process_node}};

pub fn process_body_layout( 
    node : Node, 
    nodos : &mut HashMap<String, RTMLNode>, 
    parent_id : Option<RTMLNodeId>, 
    styles : &HashMap<StyleSelector, Style>,
    xml : &str 
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    process_container( node, nodos, parent_id, styles, default_normal_style(), xml )
}

pub fn process_layout( 
    node : Node, 
    nodos : &mut HashMap<String, RTMLNode>, 
    parent_id : Option<RTMLNodeId>, 
    styles : &HashMap<StyleSelector, Style>,
    xml : &str 
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    process_container( node, nodos, parent_id, styles, Style::default(), xml )
}

fn process_container( 
    node : Node, 
    nodos : &mut HashMap<String, RTMLNode>, 
    parent_id : Option<RTMLNodeId>, 
    styles : &HashMap<StyleSelector, Style>,
    default_style : Style,
    xml : &str
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    let layout_id = id_retry_if_exists( &node, nodos );

    let mut childs : Vec<RTMLNodeId> = vec![];

    for c in node.children()
    {
        match process_node( c, nodos, Some( layout_id.clone() ), styles, xml )?
        {
            Some( ( n, id ) ) =>
            {
                nodos.insert( id.clone(), n );

                childs.push( id );
            },
            None => {}
        }
    }

    Ok(
        (
            RTMLNode::Layout(
                RTMLLayout::new(
                    attr_direction( &node )?, 
                    RTMLNodeCommon::new( 
                        parse_common_attrs( &node )?, 
                        childs, 
                        parent_id
                    ),
                    style_from_container( node, styles, default_style )
                )
            ),
            layout_id
        )
    )
}