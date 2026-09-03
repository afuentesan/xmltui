use std::collections::HashMap;

use regex::regex;
use roxmltree::Node;

use crate::{rtml::{rtml_line::RTMLLine, rtml_node::{RTMLNode, RTMLNodeCommon, RTMLNodeId}, util::types::TextLine}, xml::{attrs::{id_retry_if_exists, parse_common_attrs}, styles::xml_style::{StyleSelector, XMLStyle}, xml_doc::XMLDoc, xml_util::{paragraph_like_styles, style_from_styles}}};


pub fn process_line( 
    xml_doc : &mut XMLDoc,
    node : Node, 
    parent_id : Option<RTMLNodeId>
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    let id = id_retry_if_exists( node, xml_doc.nodos() );

    let ( constraint, line_style, padding, alignment ) = paragraph_like_styles( node, xml_doc.styles(), None );

    let padding = padding.horizontal;

    let common = RTMLNodeCommon::new( 
        parse_common_attrs( node, constraint )?, 
        vec![], 
        parent_id
    );

    let text_line = process_text_line( node, xml_doc.styles() );

    Ok(
        (
            RTMLNode::Line(
                RTMLLine::new( 
                    alignment, 
                    line_style,
                    padding,
                    common,
                    text_line
                )
            ),
            id
        )
    )
}

pub fn process_text_line( 
    node : Node,
    styles : &HashMap<StyleSelector, XMLStyle>
) -> TextLine
{
    let mut ret = vec![];

    for child in node.children()
    {
        process_node_text_or_span( child, styles, &mut ret );
    }
    
    ret
}

fn process_node_text_or_span( 
    child : Node,
    styles : &HashMap<StyleSelector, XMLStyle>,
    ret : &mut TextLine
)
{
    let re = regex!( "[ \t]*\n[ \t]*" );

    if let Some( text ) = child.text() && child.is_text()
    {
        let text = re.replace_all( &text, "" );

        ret.push( ( text.to_string(), None ) );
    }
    else if child.tag_name().name() == "span"
    {
        if let Some( t ) = child.text() && ! t.is_empty()
        {
            let style = style_from_styles( child, styles, None, None );

            let text = t.replace( "\n", "" );

            let val = ( text, Some( style ) );

            ret.push( val );
        }
    }
}