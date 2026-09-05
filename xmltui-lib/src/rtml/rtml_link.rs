use std::collections::HashMap;

use ratatui::{buffer::Buffer, layout::{Alignment, Rect}, style::Style, };
use serde_json::Value;

use crate::{app::event::{AppEvent, send_app_event}, input::event::InputEvent, rtml::{rtml_line::render_text_line, rtml_node::{FocusEventResponse, RTMLNodeCommon}, rtml_padding::HorizontalPadding, rtml_source::RTMLSource, util::{rtml_style::RTMLStyleTemplate, types::TextLine}}};


#[derive(Debug)]
pub struct RTMLLink
{
    pub common : RTMLNodeCommon,
    pub alignment : Alignment,
    pub style : Style,
    pub style_template : RTMLStyleTemplate,
    pub focus_style : Style,
    pub focus_style_template : RTMLStyleTemplate,
    pub source : RTMLSource,
    pub padding : HorizontalPadding,
    pub content : TextLine
}

impl RTMLLink
{
    pub fn new(
        alignment : Alignment,
        source : RTMLSource,
        padding : HorizontalPadding,
        content : TextLine,
        style : Style,
        style_template : RTMLStyleTemplate,
        focus_style : Style,
        focus_style_template : RTMLStyleTemplate,
        common : RTMLNodeCommon
    ) -> Self
    {
        Self { common, alignment, style, style_template, focus_style, focus_style_template, source, padding, content }
    }

    pub fn focus_event( &mut self, event : &InputEvent ) -> FocusEventResponse
    {
        match event
        {
            InputEvent::Enter =>
            {
                send_app_event(
                    AppEvent::LoadFile( self.source.source().to_string() )
                );

                FocusEventResponse::new_without_state( false )
            },
            _ => FocusEventResponse::new_without_state( false )
        }
    }
}

pub fn render_rtml_link( 
    rtml_link : &RTMLLink,
    area : Rect,
    buf : &mut Buffer,
    templates : &HashMap<String, String>,
    context : &Value
) -> anyhow::Result<()>
{
    render_rtml_link_width_style( rtml_link, rtml_link.style, &rtml_link.style_template, area, buf, templates, context )
}

pub fn render_rtml_link_focus( 
    rtml_link : &RTMLLink,
    buf : &mut Buffer,
    templates : &HashMap<String, String>,
    context : &Value
) -> anyhow::Result<()>
{
    render_rtml_link_width_style( rtml_link, rtml_link.focus_style, &rtml_link.focus_style_template, rtml_link.common.attrs.area, buf, templates, context )
}

fn render_rtml_link_width_style( 
    rtml_link : &RTMLLink,
    style : Style,
    style_template : &RTMLStyleTemplate,
    area : Rect,
    buf : &mut Buffer,
    templates : &HashMap<String, String>,
    context : &Value
) -> anyhow::Result<()>
{
    render_text_line( style, style_template, &rtml_link.padding, &rtml_link.alignment, &rtml_link.content, area, buf, templates, context)
}

