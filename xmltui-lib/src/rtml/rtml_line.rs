use std::collections::HashMap;

use ratatui::{buffer::Buffer, layout::{Alignment, Rect}, style::Style, text::{Line, Span}, widgets::Widget};
use serde_json::Value;

use crate::rtml::{rtml_node::RTMLNodeCommon, rtml_padding::HorizontalPadding, rtml_paragraph::line_from_spans, util::{rtml_style::{RTMLStyleTemplate, merge_style_with_templates}, types::TextLine}};


#[derive(Debug)]
pub struct RTMLLine
{
    pub common : RTMLNodeCommon,
    pub alignment : Alignment,
    pub style : Style,
    pub style_template : RTMLStyleTemplate,
    pub padding : HorizontalPadding,
    pub content : TextLine
}

impl RTMLLine
{
    pub fn new( alignment : Alignment, style : Style, style_template : RTMLStyleTemplate, padding : HorizontalPadding, common : RTMLNodeCommon, content : TextLine ) -> Self
    {
        Self { alignment, style, style_template, padding, common, content }
    }
}

pub fn render_rtml_line( 
    rtml_line : &RTMLLine,
    area : Rect,
    buf : &mut Buffer,
    templates : &HashMap<String, String>,
    context : &Value
) -> anyhow::Result<()>
{
    // let style = merge_style_with_templates( rtml_line.style, &rtml_line.style_template, context, templates );

    // let mut spans = if rtml_line.padding.left > 0
    // {
    //     vec![ padding_span( rtml_line.padding.left , style ) ] 
    // }
    // else
    // {
    //     vec![]
    // };

    // spans.append( &mut line_from_spans( &rtml_line.content, None, templates, context ).spans );

    // if rtml_line.padding.right > 0 { spans.push( padding_span( rtml_line.padding.right , style ) ); }

    // let line = Line::from( spans )
    // .alignment( rtml_line.alignment )
    // .style( style );

    // line.render( area, buf );

    // Ok( () )

    render_text_line( rtml_line.style, &rtml_line.style_template, &rtml_line.padding, &rtml_line.alignment, &rtml_line.content, area, buf, templates, context)
}

pub fn render_text_line(
    style : Style,
    style_template : &RTMLStyleTemplate,
    padding : &HorizontalPadding,
    alignment : &Alignment,
    content : &TextLine,
    area : Rect,
    buf : &mut Buffer,
    templates : &HashMap<String, String>,
    context : &Value
) -> anyhow::Result<()>
{
    let style = merge_style_with_templates( style, style_template, context, templates );

    let mut spans = if padding.left > 0
    {
        vec![ padding_span( padding.left , style ) ] 
    }
    else
    {
        vec![]
    };

    spans.append( &mut line_from_spans( content, None, templates, context ).spans );

    if padding.right > 0 { spans.push( padding_span( padding.right , style ) ); }

    let line = Line::from( spans )
    .alignment( *alignment )
    .style( style );

    line.render( area, buf );

    Ok( () )
}

pub fn padding_span<'a>( padding : usize, style : Style ) -> Span<'a>
{
    Span::styled( " ".repeat( padding ), style )
}