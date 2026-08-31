use ratatui::style::Style;
use roxmltree::Node;

use crate::{rtml::{rtml_line::RTMLLine, rtml_node::{RTMLNode, RTMLNodeCommon, RTMLNodeId}, rtml_span::RTMLSpan}, xml::{attrs::{id_retry_if_exists, parse_common_attrs}, styles::xml_style::merge_styles, xml_doc::XMLDoc, xml_util::{paragraph_like_styles, span_like_styles}}};


pub fn process_line( 
    xml_doc : &mut XMLDoc,
    node : Node, 
    parent_id : Option<RTMLNodeId>
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    let id = id_retry_if_exists( node, xml_doc.nodos() );

    let mut childs = vec![];

    let ( constraint, line_style, padding, alignment ) = paragraph_like_styles( node, xml_doc.styles(), None );

    for n in node.children()
    {
        childs.append( &mut process_span( xml_doc, n, id.clone(), line_style )? );
    }

    let padding = padding.horizontal;

    let common = RTMLNodeCommon::new( 
        parse_common_attrs( node, constraint )?, 
        childs, 
        parent_id
    );

    Ok(
        (
            RTMLNode::Line(
                RTMLLine::new( 
                    alignment, 
                    line_style,
                    padding,
                    common
                )
            ),
            id
        )
    )
}

pub fn process_span(
    xml_doc : &mut XMLDoc, 
    node : Node, 
    parent_id : RTMLNodeId,
    line_style : Style ) -> anyhow::Result<Vec<String>>
{
    if node.is_text()
    {
        let text = span_text( node.text().unwrap_or( "" ) );

        let ( constraint, _, padding ) = span_like_styles( node, xml_doc.styles(), None );

        let common = RTMLNodeCommon::new( 
            parse_common_attrs( node, constraint )?, 
            vec![], 
            Some( parent_id )
        );

        let span = RTMLSpan::new( 
            text,
            common,
            line_style,
            padding.horizontal
        );

        let id = id_retry_if_exists( node, xml_doc.nodos() );

        xml_doc.add_node( RTMLNode::Span( span ), id.clone() );

        Ok( vec![ id ] )
    }
    else
    {
        Ok( process_span_node( xml_doc, node, parent_id, line_style )? )
    }
}

fn process_span_node( 
    xml_doc : &mut XMLDoc, 
    node : Node, 
    parent_id : RTMLNodeId,
    line_style : Style 
) -> anyhow::Result<Vec<String>>
{
    if node.tag_name().name() != "span" || node.text().is_none() { return Ok( vec![] ) };

    let text = node.text().unwrap();

    if text == "" { return Ok( vec![] ) };

    let text = span_text( text );

    let ( constraint, span_style, padding ) = span_like_styles( node, xml_doc.styles(), None );

    let span_style = merge_styles( line_style, span_style );

    let common = RTMLNodeCommon::new( 
        parse_common_attrs( node, constraint )?, 
        vec![], 
        Some( parent_id )
    );

    let span = RTMLSpan::new( 
        text,
        common,
        span_style,
        padding.horizontal
    );
    
    let id = id_retry_if_exists( node, xml_doc.nodos() );

    xml_doc.add_node( RTMLNode::Span( span ), id.clone() );

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