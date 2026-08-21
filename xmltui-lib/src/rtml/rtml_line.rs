use ratatui::{buffer::Buffer, layout::{Alignment, Rect}, style::Style, text::Line, widgets::Widget};

use crate::rtml::{rtml_doc::RTMLDoc, rtml_node::RTMLNodeCommon, rtml_span::spans_from_childs};


#[derive(Debug)]
pub struct RTMLLine
{
    pub common : RTMLNodeCommon,
    pub alignment : Alignment,
    pub style : Style
}

impl RTMLLine
{
    pub fn new( alignment : Alignment, style : Style, common : RTMLNodeCommon ) -> Self
    {
        Self { alignment, style, common }
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

    let spans = spans_from_childs( childs, doc )?;

    let line = Line::from( spans )
    .alignment( rtml_line.alignment )
    .style( rtml_line.style );

    line.render( area, buf );

    Ok( () )
}