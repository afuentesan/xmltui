use std::collections::HashMap;

use ratatui::{style::Style, widgets::{BorderType, Borders, TitlePosition}};
use roxmltree::Node;

use crate::{rtml::{rtml_border::RTMLBorder, rtml_node::{RTMLNode, RTMLNodeCommon, RTMLNodeId}}, xml::{attrs::{attr_alignment_name, attr_direction, attr_flex, attr_option, id_retry_if_exists, parse_common_attrs}, styles::{default_styles::default_normal_style, xml_style::{StyleSelector, StyleVariant, style_from_node}}, xml2rtml::process_node}};


pub fn process_border( 
    node : Node, 
    nodos : &mut HashMap<String, RTMLNode>, 
    parent_id : Option<RTMLNodeId>, 
    styles : &HashMap<StyleSelector, Style>,
    xml : &str
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    let border_id = id_retry_if_exists( &node, nodos );

    let mut childs : Vec<RTMLNodeId> = vec![];

    for c in node.children()
    {
        match process_node( c, nodos, Some( border_id.clone() ), styles, xml )?
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
            RTMLNode::Border(
                RTMLBorder::new(
                    borders( node ), 
                    border_type( node ),
                    attr_option( node, "title" ),
                    title_position( node ),
                    attr_alignment_name( &node, "title-align" )?,
                    style_from_node( node, styles, default_normal_style(), None ),
                    style_from_node( node, styles, default_normal_style(), Some( StyleVariant::Title ) ),
                    style_from_node( node, styles, default_normal_style(), Some( StyleVariant::Border ) ),
                    attr_direction( &node )?,
                    attr_flex( &node )?,
                    RTMLNodeCommon::new( 
                        parse_common_attrs( &node )?, 
                        childs, 
                        parent_id
                    )
                )
            ),
            border_id
        )
    )
}

fn title_position( node : Node ) -> TitlePosition
{
    match node.attribute( "title-position" )
    {
        Some( t ) =>
        {
            match t.trim()
            {
                "" | "top" => TitlePosition::Top,
                "bottom" => TitlePosition::Bottom,
                _ => TitlePosition::Top    
            }
        },
        None => TitlePosition::Top
    }
}

fn borders( node : Node ) -> Borders
{
    match node.attribute( "borders" )
    {
        Some( b ) if b.trim() != "" =>
        {
            let mut borders = Borders::empty();

            for b in b.split( "," )
            {
                match b.trim()
                {
                    "top" => borders = borders.union( Borders::TOP ),
                    "left" => borders = borders.union( Borders::LEFT ),
                    "right" => borders = borders.union( Borders::RIGHT ),
                    "bottom" => borders = borders.union( Borders::BOTTOM ),
                    _ => continue
                }
            }

            borders
        },
        _ => Borders::all()
    }
}

// TODO: Permitir diferentes tipos de borde
fn border_type( _node : Node ) -> BorderType
{
    BorderType::default()
}