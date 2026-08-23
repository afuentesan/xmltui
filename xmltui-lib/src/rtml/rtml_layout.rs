use ratatui::{buffer::Buffer, layout::{Direction, Flex, Rect}, style::Style};

use crate::{rtml::rtml_node::RTMLNodeCommon, util::draw::clear_area};


#[derive(Debug)]
pub struct RTMLLayout 
{
    pub common : RTMLNodeCommon,
    pub direction : Direction,
    pub flex : Flex,
    pub style : Option<Style>
}

impl RTMLLayout
{
    pub fn new( direction : Direction, flex : Flex, common : RTMLNodeCommon, style : Option<Style> ) -> Self
    {
        Self { direction, common, style, flex }
    }
}

pub fn render_rtml_layout(
    layout : &RTMLLayout,
    area : Rect,
    buf : &mut Buffer
)
{
    match layout.style
    {
        Some( s ) =>
        {
            clear_area( area, s, buf );
        },
        None => {}
    }
}