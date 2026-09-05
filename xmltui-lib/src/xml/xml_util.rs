use std::collections::HashMap;

use ratatui::{layout::{Alignment, Constraint}, style::Style};
use roxmltree::Node;

use crate::{rtml::{rtml_attrs::ContainerAttrs, rtml_padding::RTMLPadding, util::rtml_style::RTMLStyleTemplate}, xml::styles::xml_style::{StyleSelector, StyleVariant, XMLStyle, merge_styles, style_from_node}};

const DEFAULT_CONSTRAINT : Constraint = Constraint::Percentage(100);

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

pub fn container_styles( node : Node, styles : &HashMap<StyleSelector, XMLStyle>, variant : Option<StyleVariant> ) -> ( Constraint, Style, RTMLStyleTemplate, ContainerAttrs )
{
    let styles = style_from_node( node, styles, variant );

    let constraint = styles.0.constraint.0.unwrap_or( DEFAULT_CONSTRAINT );

    let style = styles.0.style.0.unwrap_or_default();

    let direction = styles.0.direction.unwrap_or_default();

    let flex = styles.0.flex.unwrap_or_default();

    let horizontal = styles.0.inner_padding.0.0.unwrap_or_default();

    let vertical = styles.0.inner_padding.0.1.unwrap_or_default();

    let padding = RTMLPadding::new( horizontal, vertical );

    let container_attrs = ContainerAttrs::new( direction, flex, padding );

    ( constraint, style, styles.1, container_attrs )
}

pub fn paragraph_like_styles( node : Node, styles : &HashMap<StyleSelector, XMLStyle>, variant : Option<StyleVariant> ) -> ( Constraint, Style, RTMLStyleTemplate, RTMLPadding, Alignment )
{
    let styles = style_from_node( node, styles, variant );

    let constraint = styles.0.constraint.0.unwrap_or( DEFAULT_CONSTRAINT );

    let style = styles.0.style.0.unwrap_or_default();

    let horizontal = styles.0.inner_padding.0.0.unwrap_or_default();

    let vertical = styles.0.inner_padding.0.1.unwrap_or_default();

    let padding = RTMLPadding::new( horizontal, vertical );

    let alignment = styles.0.alignment.unwrap_or_default();

    ( constraint, style, styles.1, padding, alignment )
}

pub fn input_like_styles( node : Node, styles : &HashMap<StyleSelector, XMLStyle>, variant : Option<StyleVariant> ) -> ( Constraint, Style, RTMLStyleTemplate, Alignment )
{
    let styles = style_from_node( node, styles, variant );

    let constraint = styles.0.constraint.0.unwrap_or( DEFAULT_CONSTRAINT );

    let style = styles.0.style.0.unwrap_or_default();

    let alignment = styles.0.alignment.unwrap_or_default();

    ( constraint, style, styles.1, alignment )
}

pub fn style_from_styles( node : Node, styles : &HashMap<StyleSelector, XMLStyle>, variant : Option<StyleVariant>, default_style : Option<Style> ) -> ( Style, RTMLStyleTemplate )
{
    let styles = style_from_node( node, styles, variant );

    match default_style
    {
        Some( s ) => 
        {
            let s = if let Some( s2 ) = styles.0.style.0
            {
                merge_styles( s, s2 )
            }
            else { s };

            ( s, styles.1 )
        },
        None => ( styles.0.style.0.unwrap_or_default(), styles.1 )
    }
}