use std::collections::HashMap;

use ratatui::{buffer::Buffer, layout::Rect, style::Style};
use serde_json::Value;

use crate::{rtml::{util::rtml_attrs::ContainerAttrs, rtml_node::RTMLNodeCommon, util::rtml_style::{RTMLStyleTemplate, merge_style_with_templates}}, util::draw::clear_area};


#[derive(Debug)]
pub struct RTMLState
{
    pub common : RTMLNodeCommon,
    pub container : ContainerAttrs,
    pub style : Option<Style>,
    pub style_template : RTMLStyleTemplate,
    pub template_name : Option<String>,
    pub template : Option<String>,
    pub reload_with_state : bool,
    pub reload_with_state_path : Vec<String>
}

impl RTMLState
{
    pub fn new(
        common : RTMLNodeCommon, 
        container : ContainerAttrs,
        style : Option<Style>,
        style_template : RTMLStyleTemplate,
        template_name : Option<String>,
        template : Option<String>,
        reload_with_state : bool,
        reload_with_state_path : Vec<String>
    ) -> Self
    {
        Self { common, container, style, style_template, template_name, template, reload_with_state, reload_with_state_path }
    }

    pub fn node_template<'a>( &'a self, templates : &'a HashMap<String, String> ) -> Option<&'a String>
    {
        if let Some( n ) = self.template_name.as_ref() && templates.contains_key( n )
        {
            return templates.get( n )
        }

        self.template.as_ref()
    }
}

pub fn render_rtml_state(
    layout : &RTMLState,
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