use ratatui::{buffer::Buffer, layout::Rect, style::Style, widgets::{Block, Widget}};


pub fn clear_area(
    area : Rect,
    style : Style,
    buf : &mut Buffer
)
{
    let block = Block::default().style( style );

    block.render( area, buf );
}