use std::collections::HashMap;

use ratatui::{layout::{Alignment, Constraint}, style::Style};
use roxmltree::Node;

use crate::{rtml::{rtml_attrs::ContainerAttrs, rtml_padding::RTMLPadding}, xml::styles::xml_style::{StyleSelector, StyleVariant, XMLStyle, style_from_node}};

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

pub fn container_styles( node : Node, styles : &HashMap<StyleSelector, XMLStyle>, variant : Option<StyleVariant> ) -> ( Constraint, Style, ContainerAttrs )
{
    let styles = style_from_node( node, styles, variant );

    let constraint = styles.constraint.0.unwrap_or( DEFAULT_CONSTRAINT );

    let style = styles.style.0.unwrap_or_default();

    let direction = styles.direction.unwrap_or_default();

    let flex = styles.flex.unwrap_or_default();

    let horizontal = styles.inner_padding.0.0.unwrap_or_default();

    let vertical = styles.inner_padding.0.1.unwrap_or_default();

    let padding = RTMLPadding::new( horizontal, vertical );

    let container_attrs = ContainerAttrs::new( direction, flex, padding );

    ( constraint, style, container_attrs )
}

// pub fn span_like_styles( node : Node, styles : &HashMap<StyleSelector, XMLStyle>, variant : Option<StyleVariant> ) -> ( Constraint, Style, RTMLPadding )
// {
//     let styles = style_from_node( node, styles, variant );

//     let constraint = styles.constraint.0.unwrap_or( DEFAULT_CONSTRAINT );

//     let style = styles.style.0.unwrap_or_default();

//     let horizontal = styles.inner_padding.0.0.unwrap_or_default();

//     let vertical = styles.inner_padding.0.1.unwrap_or_default();

//     let padding = RTMLPadding::new( horizontal, vertical );

//     ( constraint, style, padding )
// }

pub fn paragraph_like_styles( node : Node, styles : &HashMap<StyleSelector, XMLStyle>, variant : Option<StyleVariant> ) -> ( Constraint, Style, RTMLPadding, Alignment )
{
    let styles = style_from_node( node, styles, variant );

    let constraint = styles.constraint.0.unwrap_or( DEFAULT_CONSTRAINT );

    let style = styles.style.0.unwrap_or_default();

    let horizontal = styles.inner_padding.0.0.unwrap_or_default();

    let vertical = styles.inner_padding.0.1.unwrap_or_default();

    let padding = RTMLPadding::new( horizontal, vertical );

    let alignment = styles.alignment.unwrap_or_default();

    ( constraint, style, padding, alignment )
}

pub fn input_like_styles( node : Node, styles : &HashMap<StyleSelector, XMLStyle>, variant : Option<StyleVariant> ) -> ( Constraint, Style, Alignment )
{
    let styles = style_from_node( node, styles, variant );

    let constraint = styles.constraint.0.unwrap_or( DEFAULT_CONSTRAINT );

    let style = styles.style.0.unwrap_or_default();

    let alignment = styles.alignment.unwrap_or_default();

    ( constraint, style, alignment )
}

pub fn style_from_styles( node : Node, styles : &HashMap<StyleSelector, XMLStyle>, variant : Option<StyleVariant>, default_style : Option<Style> ) -> Style
{
    let styles = style_from_node( node, styles, variant );

    match default_style
    {
        Some( s ) => styles.style.0.unwrap_or( s ),
        None => styles.style.0.unwrap_or_default()
    }
}