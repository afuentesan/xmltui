use ratatui::{buffer::Buffer, layout::Rect, style::Style};

use crate::{rtml::{rtml_attrs::ContainerAttrs, rtml_node::RTMLNodeCommon}, util::draw::clear_area};


#[derive(Debug)]
pub struct RTMLLayout 
{
    pub common : RTMLNodeCommon,
    pub container : ContainerAttrs,
    pub style : Option<Style>
}

impl RTMLLayout
{
    pub fn new( common : RTMLNodeCommon, container : ContainerAttrs, style : Option<Style> ) -> Self
    {
        Self { common, container, style }
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