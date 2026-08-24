use ratatui::{buffer::Buffer, layout::{Alignment, Rect}, style::Style, text::Line, widgets::Widget};

use crate::rtml::{rtml_doc::RTMLDoc, rtml_node::RTMLNodeCommon, rtml_padding::HorizontalPadding, rtml_span::{padding_span, spans_from_childs}};


#[derive(Debug)]
pub struct RTMLLine
{
    pub common : RTMLNodeCommon,
    pub alignment : Alignment,
    pub style : Style,
    pub padding : HorizontalPadding
}

impl RTMLLine
{
    pub fn new( alignment : Alignment, style : Style, padding : HorizontalPadding, common : RTMLNodeCommon ) -> Self
    {
        Self { alignment, style, padding, common }
    }
}

pub fn render_rtml_line( 
    rtml_line : &RTMLLine,
    childs : &Vec<String>,
    area : Rect,
    buf : &mut Buffer,
    doc : &RTMLDoc
) -> anyhow::Result<()>
{
    if childs.len() == 0 { return Ok( () ) }

    let mut spans = if rtml_line.padding.left > 0
    {
        vec![ padding_span( rtml_line.padding.left , rtml_line.style ) ] 
    }
    else
    {
        vec![]
    };

    spans.append( &mut spans_from_childs( childs, doc )? );

    if rtml_line.padding.right > 0 { spans.push( padding_span( rtml_line.padding.right , rtml_line.style ) ); }

    let line = Line::from( spans )
    .alignment( rtml_line.alignment )
    .style( rtml_line.style );

    line.render( area, buf );

    Ok( () )
}