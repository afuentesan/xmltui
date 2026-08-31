use std::collections::HashMap;

use ratatui::{layout::{Constraint, Direction, Flex}, style::Style};
use roxmltree::Node;

use crate::{rtml::{rtml_attrs::ContainerAttrs, rtml_padding::{HorizontalPadding, RTMLPadding, VerticalPadding}}, xml::styles::xml_style::{StyleSelector, StyleVariant, XMLStyle, style_from_node_2}};


pub fn template_from_inner_node( node : Node, xml : &str ) -> Option<String>
{
    for node in node.children()
    {
        if let Some( t ) = template_from_node( node, xml )
        {
            return Some( t );
        }
    }

    None
}

pub fn template_from_node( node : Node, xml : &str ) -> Option<String>
{
    if node.tag_name().name() != "template" { return None; }
        
    return match ( node.first_child(), node.last_child() ) 
    {
        ( Some( primer_hijo ), Some( ultimo_hijo ) ) => 
        {
            let inicio = primer_hijo.range().start;
            let fin = ultimo_hijo.range().end;

            if xml[ inicio..fin ].trim() == "" 
            { 
                None 
            }
            else
            {
                Some( xml[ inicio..fin ].to_string() )    
            }
        }
        _ => None
    }
}

// Este de momento no lo uso así que lo comento
// pub fn template_from_node( node : Node, xml : &str ) -> String
// {
//     xml[ node.range() ].to_string()
// }

pub fn container_styles( node : Node, styles : &HashMap<StyleSelector, XMLStyle>, variant : Option<StyleVariant> ) -> ( Constraint, Style, ContainerAttrs )
{
    let styles = style_from_node_2( node, styles, variant );

    let constraint = styles.constraint.0.unwrap_or_default();

    let style = styles.style.0.unwrap_or_default();

    let direction = styles.direction.unwrap_or_default();

    let flex = styles.flex.unwrap_or_default();

    let horizontal = styles.inner_padding.0.0.unwrap_or_default();

    let vertical = styles.inner_padding.0.1.unwrap_or_default();

    let padding = RTMLPadding::new_parts( horizontal, vertical );

    let container_attrs = ContainerAttrs::new( direction, flex, padding );

    ( constraint, style, container_attrs )
}