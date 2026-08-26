use std::collections::HashMap;

use ratatui::style::Style;
use roxmltree::Node;

use crate::{rtml::{rtml_node::{RTMLNode, RTMLNodeCommon, RTMLNodeId}, rtml_paragraph::RTMLParagraph}, xml::{attrs::{id_retry_if_exists, parse_common_attrs}, styles::{default_styles::default_normal_style, xml_style::{StyleSelector, style_from_node}}}};


pub fn process_paragraph( 
    node : Node, 
    nodos : &mut HashMap<String, RTMLNode>, 
    parent_id : Option<RTMLNodeId>,
    styles : &HashMap<StyleSelector, Style> 
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    let lines = process_lines( node, styles )?;
    let style = style_from_node( node, styles, default_normal_style(), None );
    let common = RTMLNodeCommon::new( 
        parse_common_attrs( node )?, 
        vec![], 
        parent_id
    );

    let id = id_retry_if_exists( node, nodos );

    Ok(
        (
            RTMLNode::Paragraph(
                RTMLParagraph::new( common, style, lines )
            ),
            id
        )
    )
}

fn process_lines(
    node : Node,
    styles : &HashMap<StyleSelector, Style>   
) -> anyhow::Result<Vec<Vec<( String, Option<Style> )>>>
{
    let mut ret = vec![];

    for child in node.children()
    {
        if let Some( text ) = child.text() && child.is_text()
        {
            if ret.len() == 0 { ret.push( vec![] ); }

            let trim_right = text.ends_with( "\n" );
            let trim_left = text.starts_with( "\n" );
            let trim_all = trim_right && trim_left;

            text.split( "\n" )
            .enumerate()
            .for_each(
                | ( i, s ) |
                {
                    let val = ( trimmed_string( s, trim_all, trim_left, trim_right ), None );

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

    Ok( ret )
}

fn trimmed_string( str : &str, trim : bool, trim_left : bool, trim_right : bool ) -> String
{
    if trim
    {
        str.trim().to_string()
    }
    else if trim_left
    {
        str.trim_start().to_string()
    }
    else if trim_right
    {
        str.trim_end().to_string()
    }
    else
    {
        str.to_string()    
    }
}