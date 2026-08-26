use std::collections::HashMap;

use ratatui::style::Style;
use roxmltree::Node;

use crate::{rtml::{rtml_line::RTMLLine, rtml_node::{RTMLNode, RTMLNodeCommon, RTMLNodeId}, rtml_span::RTMLSpan}, xml::{attrs::{attr_alignment, id_retry_if_exists, parse_common_attrs}, styles::{default_styles::default_normal_style, xml_style::{StyleSelector, style_from_node}}, xml_padding::horizontal_padding_from_node}};


pub fn process_line( 
    node : Node, 
    nodos : &mut HashMap<String, RTMLNode>, 
    parent_id : Option<RTMLNodeId>,
    styles : &HashMap<StyleSelector, Style> 
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    let id = id_retry_if_exists( node, nodos );

    let mut childs = vec![];

    let line_style = style_from_node( node, styles, default_normal_style(), None );

    for n in node.children()
    {
        childs.append( &mut process_span( n, nodos, id.clone(), styles, line_style )? );
    }

    let alignment = attr_alignment( node )?;

    let padding = horizontal_padding_from_node( node );

    let common_attrs = parse_common_attrs( node )?;

    Ok(
        (
            RTMLNode::Line(
                RTMLLine::new( 
                    alignment, 
                    line_style,
                    padding,
                    RTMLNodeCommon::new( 
                        common_attrs, 
                        childs, 
                        parent_id
                    )
                )
            ),
            id
        )
    )
}

pub fn process_span( 
    node : Node, 
    nodos : &mut HashMap<String, RTMLNode>, 
    parent_id : RTMLNodeId,
    styles : &HashMap<StyleSelector, Style>, 
    line_style : Style ) -> anyhow::Result<Vec<String>>
{
    if node.is_text()
    {
        let text = span_text( node.text().unwrap_or( "" ) );

        let padding = horizontal_padding_from_node( node );

        let common = parse_common_attrs( node )?;

        let span = RTMLSpan::new( 
            text,
            RTMLNodeCommon::new( 
                        common, 
                        vec![], 
                        Some( parent_id )
                    ),
                    line_style,
                    padding
        );

        let id = id_retry_if_exists( node, nodos );

        nodos.insert(
            id.clone(), 
            RTMLNode::Span( span )
        );

        Ok( vec![ id ] )
    }
    else
    {
        Ok( process_span_node( node, nodos, parent_id, styles, line_style )? )
    }
}

fn process_span_node( 
    node : Node, 
    nodos : &mut HashMap<String, RTMLNode>, 
    parent_id : RTMLNodeId,
    styles : &HashMap<StyleSelector, Style>, 
    line_style : Style 
) -> anyhow::Result<Vec<String>>
{
    if node.tag_name().name() != "span" || node.text().is_none() { return Ok( vec![] ) };

    let text = node.text().unwrap();

    if text == "" { return Ok( vec![] ) };

    let text = span_text( text );

    let span_style = style_from_node( node, styles, line_style, None );

    let padding = horizontal_padding_from_node( node );

    let common = parse_common_attrs( node )?;

    let span = RTMLSpan::new( 
        text,
        RTMLNodeCommon::new( 
                    common, 
                    vec![], 
                    Some( parent_id )
                ),
                span_style,
                padding
    );
    
    let id = id_retry_if_exists( node, nodos );

        nodos.insert(
            id.clone(), 
            RTMLNode::Span( span )
        );

    Ok( vec![ id ] )
}

fn span_text( text : &str ) -> String
{
    text
    .split( "\n" )
    .fold(
        String::new(), 
        | mut acc, s |
        {
            acc.push_str( s.trim() );

            acc
        }
    )   
}