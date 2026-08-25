use ratatui::{buffer::Buffer, layout::{Alignment, Rect}, style::Style, widgets::{Block, BorderType, Borders, TitlePosition, Widget}};

use crate::rtml::{rtml_attrs::ContainerAttrs, rtml_node::RTMLNodeCommon};

#[derive(Debug)]
pub struct RTMLBorder
{
    pub common : RTMLNodeCommon,
    pub container : ContainerAttrs,
    pub borders : Borders,
    pub border_type : BorderType,
    pub style : Style,
    pub title_style : Style,
    pub border_style : Style,
    pub title : Option<String>,
    pub title_position : TitlePosition,
    pub title_alignment : Alignment
}

impl RTMLBorder
{
    pub fn new(
        borders : Borders,
        border_type : BorderType,
        title : Option<String>,
        title_position : TitlePosition,
        title_alignment : Alignment,
        style : Style,
        title_style : Style,
        border_style : Style,
        container : ContainerAttrs,
        common : RTMLNodeCommon
    ) -> Self
    {
        Self { common, borders, border_type, style, title_style, border_style, title, title_position, title_alignment, container }
    }
}

pub fn render_rtml_border(
    border : &RTMLBorder,
    area : Rect,
    buf : &mut Buffer
) -> Rect
{
    let mut block = Block::default()
    .borders( border.borders )
    .border_type( border.border_type )
    .style( border.style )
    .border_style( border.border_style );

    if let Some( t ) = border.title.as_ref()
    {
        block = block.title( t.as_str() )
        .title_style( border.title_style )
        .title_position( border.title_position )
        .title_alignment( border.title_alignment );
    }

    let inner_area = block.inner( area );

    block.render( area, buf );
    
    inner_area
}