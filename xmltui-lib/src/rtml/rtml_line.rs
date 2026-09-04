use ratatui::{buffer::Buffer, layout::{Alignment, Rect}, style::Style, text::{Line, Span}, widgets::Widget};

use crate::rtml::{rtml_node::RTMLNodeCommon, rtml_padding::HorizontalPadding, rtml_paragraph::line_from_spans, util::types::TextLine};


#[derive(Debug)]
pub struct RTMLLine
{
    pub common : RTMLNodeCommon,
    pub alignment : Alignment,
    pub style : Style,
    pub padding : HorizontalPadding,
    pub content : TextLine
}

impl RTMLLine
{
    pub fn new( alignment : Alignment, style : Style, padding : HorizontalPadding, common : RTMLNodeCommon, content : TextLine ) -> Self
    {
        Self { alignment, style, padding, common, content }
    }
}

pub fn render_rtml_line( 
    rtml_line : &RTMLLine,
    area : Rect,
    buf : &mut Buffer
) -> anyhow::Result<()>
{
    let mut spans = if rtml_line.padding.left > 0
    {
        vec![ padding_span( rtml_line.padding.left , rtml_line.style ) ] 
    }
    else
    {
        vec![]
    };

    spans.append( &mut line_from_spans( &rtml_line.content, None ).spans );

    if rtml_line.padding.right > 0 { spans.push( padding_span( rtml_line.padding.right , rtml_line.style ) ); }

    let line = Line::from( spans )
    .alignment( rtml_line.alignment )
    .style( rtml_line.style );

    line.render( area, buf );

    Ok( () )
}

fn padding_span<'a>( padding : usize, style : Style ) -> Span<'a>
{
    Span::styled( " ".repeat( padding ), style )
}