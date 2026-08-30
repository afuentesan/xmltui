use std::collections::HashMap;

use ratatui::style::Style;
use regex::regex;
use roxmltree::Node;

use crate::{rtml::{rtml_node::{RTMLNode, RTMLNodeCommon, RTMLNodeId}, rtml_select::RTMLSelect, util::types::{TextLine, TextLines}}, xml::{attrs::{attr_alignment, id_retry_if_exists, parse_common_attrs}, styles::{default_styles::{default_focus_style, default_normal_style}, xml_style::{StyleSelector, StyleVariant, style_from_node}}, xml_doc::{XMLDoc, replace_xml_doc_focus}, xml_event::parse_event_attrs, xml_padding::container_padding_from_node}};


pub fn process_select( 
    xml_doc : &mut XMLDoc,
    node : Node, 
    parent_id : Option<RTMLNodeId>
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    let ( selected_line, values, lines ) = process_options( node, xml_doc.styles() )?;
    let style = style_from_node( node, xml_doc.styles(), default_normal_style(), None );
    let focus_style = style_from_node( node, xml_doc.styles(), default_normal_style(), Some( StyleVariant::Focus ) );
    let selected_style = style_from_node( node, xml_doc.styles(), default_focus_style( &style ), Some( StyleVariant::Selected ) );
    let padding = container_padding_from_node( node );
    
    let id = id_retry_if_exists( node, xml_doc.nodos() );

    let common = RTMLNodeCommon::new( 
        parse_common_attrs( node )?, 
        vec![], 
        parent_id
    );

    let alignment = attr_alignment( node )?;

    replace_xml_doc_focus( xml_doc, node, &id );
    
    Ok(
        (
            RTMLNode::Select(
                RTMLSelect::new( 
                    common, 
                    padding,
                    alignment,
                    style, 
                    focus_style,
                    selected_style,
                    lines,
                    values,
                    parse_event_attrs( node, &id )?,
                    selected_line
                )
            ),
            id
        )
    )
}

fn process_options( node : Node, styles : &HashMap<StyleSelector, Style> ) -> anyhow::Result<( usize, Vec<String>, TextLines )>
{
    let mut selected = 0;
    let mut values = vec![];
    let mut lines = vec![];

    node.children()
    .filter( | n | n.tag_name().name() == "option" )
    .enumerate()
    .for_each(
        | ( idx, option ) |
        {
            let ( selected_line, value, line ) = process_option( option, styles );

            if selected_line { selected = idx };

            values.push( value );
            lines.push( line );
        }
    );
    
    Ok( ( selected, values, lines ) )
}

fn process_option( 
    option : Node, 
    styles : &HashMap<StyleSelector, Style>
) -> ( bool, String, TextLine )
{
    let selected = if let Some( sel ) = option.attribute( "selected" ) && sel.trim() == "true"
    {
        true
    }
    else { false };

    let value = if let Some( val ) = option.attribute( "value" ) { val.to_string() } else { String::from( "" ) };

    let text = process_option_text( option, styles );

    ( selected, value, text )
}

fn process_option_text( 
    option : Node,
    styles : &HashMap<StyleSelector, Style>
) -> TextLine
{
    let mut ret = vec![];

    for child in option.children()
    {
        process_node_text_or_span( child, styles, &mut ret );
    }
    
    ret
}

fn process_node_text_or_span( 
    child : Node,
    styles : &HashMap<StyleSelector, Style>,
    ret : &mut TextLine
)
{
    let re = regex!( "[ \t]*\n[ \t]*" );

    if let Some( text ) = child.text() && child.is_text()
    {
        let text = re.replace_all( &text, " " );

        ret.push( ( text.to_string(), None ) );
    }
    else if child.tag_name().name() == "span"
    {
        if let Some( t ) = child.text() && ! t.is_empty()
        {
            let style = style_from_node( child, styles, default_normal_style(), None );
            let text = t.replace( "\n", " " );

            let val = ( text, Some( style ) );

            ret.push( val );
        }
    }
}