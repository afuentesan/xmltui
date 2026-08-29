use std::collections::HashMap;

use ratatui::style::Style;
use regex::regex;
use roxmltree::Node;

use crate::{rtml::{rtml_node::{RTMLNode, RTMLNodeCommon, RTMLNodeId}, rtml_paragraph::RTMLParagraph, util::types::TextLines}, xml::{attrs::{attr_alignment, id_retry_if_exists, parse_common_attrs}, styles::{default_styles::{default_focus_style, default_normal_style}, xml_style::{StyleSelector, StyleVariant, style_from_node}}, xml_doc::{XMLDoc, replace_xml_doc_focus}, xml_padding::container_padding_from_node}};


pub fn process_paragraph( 
    xml_doc : &mut XMLDoc,
    node : Node, 
    parent_id : Option<RTMLNodeId>
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    let lines = process_lines( node, xml_doc.styles() )?;
    let style = style_from_node( node, xml_doc.styles(), default_normal_style(), None );
    let focus_style = style_from_node( node, xml_doc.styles(), default_focus_style( &style ), Some( StyleVariant::Focus ) );
    let padding = container_padding_from_node( node );

    let common = RTMLNodeCommon::new( 
        parse_common_attrs( node )?, 
        vec![], 
        parent_id
    );

    let id = id_retry_if_exists( node, xml_doc.nodos() );

    let alignment = attr_alignment( node )?;

    replace_xml_doc_focus( xml_doc, node, &id );

    Ok(
        (
            RTMLNode::Paragraph(
                RTMLParagraph::new( common, padding, alignment, style, focus_style, lines )
            ),
            id
        )
    )
}

fn process_lines(
    node : Node,
    styles : &HashMap<StyleSelector, Style>   
) -> anyhow::Result<TextLines>
{
    let mut ret = vec![];

    for child in node.children()
    {
        process_node_text_or_span( child, styles, &mut ret );
    }

    while let Some( l ) = ret.last() 
    {
        if l.len() == 1 && l[ 0 ].0.trim().is_empty() 
        {
            ret.pop();
        } 
        else 
        {
            break;
        }
    }

    Ok( ret )
}

fn process_node_text_or_span( 
    child : Node,
    styles : &HashMap<StyleSelector, Style>,
    ret : &mut TextLines
)
{
    let re = regex!( "[ \t]*\n[ \t]*" );

    if let Some( mut text ) = child.text() && child.is_text()
    {
        if ret.is_empty() 
        {
            text = text.trim_start_matches( | c | matches!( c, ' ' | '\t' | '\n' ) );

            if text.is_empty()
            {
                return; 
            }

            ret.push( vec![] );
        }

        let text = re.replace_all( &text, "\n" );

        text.split( "\n" )
        .enumerate()
        .for_each(
            | ( i, s ) |
            {
                let val = ( s.to_string(), None );

                if i == 0
                {
                    ret.last_mut().unwrap().push( val );
                }
                else
                {
                    ret.push( vec![ val ] );
                }
            }
        );
    }
    else if child.tag_name().name() == "span"
    {
        if let Some( t ) = child.text() && ! t.is_empty()
        {
            let style = style_from_node( child, styles, default_normal_style(), None );
            let text = t.replace( "\n", " " );

            let val = ( text, Some( style ) );

            if let Some( l ) = ret.last_mut()
            {
                l.push( val );
            }
            else
            {
                ret.push( vec![ val ] );
            }
        }
    }
}