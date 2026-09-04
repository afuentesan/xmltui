use std::collections::HashMap;

use ratatui::{buffer::Buffer, layout::Rect, style::Style};
use serde_json::Value;

use crate::{rtml::{rtml_attrs::ContainerAttrs, rtml_node::RTMLNodeCommon, util::rtml_style::{RTMLStyleTemplate, merge_style_with_templates}}, util::draw::clear_area};


#[derive(Debug)]
pub struct RTMLLayout 
{
    pub common : RTMLNodeCommon,
    pub container : ContainerAttrs,
    pub style : Option<Style>,
    pub style_template : RTMLStyleTemplate
}

impl RTMLLayout
{
    pub fn new( common : RTMLNodeCommon, container : ContainerAttrs, style : Option<Style>, style_template : RTMLStyleTemplate ) -> Self
    {
        Self { common, container, style, style_template }
    }
}

pub fn render_rtml_layout(
    layout : &RTMLLayout,
    area : Rect,
    buf : &mut Buffer,
    templates : &HashMap<String, String>,
    context : &Value
)
{
    let style = if let Some( s ) = layout.style
    {
        s
    }
    else
    {
        Style::default()    
    };

    let style = merge_style_with_templates( style, &layout.style_template, context, templates );

    clear_area( area, style, buf );
}