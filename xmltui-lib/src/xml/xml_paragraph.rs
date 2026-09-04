use std::collections::HashMap;

use regex::regex;
use roxmltree::Node;

use crate::{rtml::{rtml_node::{RTMLNode, RTMLNodeCommon, RTMLNodeId}, rtml_paragraph::RTMLParagraph, util::types::TextLines}, xml::{attrs::{id_retry_if_exists, parse_common_attrs}, styles::{default_styles::default_focus_style, xml_style::{StyleSelector, StyleVariant, XMLStyle}}, xml_doc::{XMLDoc, replace_xml_doc_focus}, xml_util::{paragraph_like_styles, style_from_styles}}};


pub fn process_paragraph( 
    xml_doc : &mut XMLDoc,
    node : Node, 
    parent_id : Option<RTMLNodeId>
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    let lines = process_lines( node, xml_doc.styles() )?;
    
    let ( constraint, style, style_template, padding, alignment ) = paragraph_like_styles( node, xml_doc.styles(), None );

    let ( focus_style, focus_style_template ) = style_from_styles( node, xml_doc.styles(), Some( StyleVariant::Focus ), Some( default_focus_style( &style ) ) );

    let common = RTMLNodeCommon::new( 
        parse_common_attrs( constraint )?, 
        vec![], 
        parent_id
    );

    let id = id_retry_if_exists( node, xml_doc.nodos() );

    replace_xml_doc_focus( xml_doc, node, &id );

    Ok(
        (
            RTMLNode::Paragraph(
                RTMLParagraph::new( common, padding, alignment, style, style_template, focus_style, focus_style_template, lines )
            ),
            id
        )
    )
}

fn process_lines(
    node : Node,
    styles : &HashMap<StyleSelector, XMLStyle>   
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
    styles : &HashMap<StyleSelector, XMLStyle>,
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
            let ( style, style_template ) = style_from_styles( child, styles, None, None );

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