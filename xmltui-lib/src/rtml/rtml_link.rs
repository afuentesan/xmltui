use std::collections::HashMap;

use ratatui::{buffer::Buffer, layout::{Alignment, Rect}, style::Style, text::{Line, Span}, widgets::Widget};
use serde_json::Value;

use crate::{app::event::{AppEvent, send_app_event}, input::event::InputEvent, rtml::{rtml_node::{FocusEventResponse, RTMLNodeCommon}, rtml_source::RTMLSource, util::rtml_style::{RTMLStyleTemplate, merge_style_with_templates}}, util::draw::clear_area};


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
    pub text : String
}

impl RTMLLink
{
    pub fn new(
        alignment : Alignment,
        source : RTMLSource,
        text : String,
        style : Style,
        style_template : RTMLStyleTemplate,
        focus_style : Style,
        focus_style_template : RTMLStyleTemplate,
        common : RTMLNodeCommon
    ) -> Self
    {
        Self { common, alignment, style, style_template, focus_style, focus_style_template, source, text }
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

    pub fn replace_value( &mut self, new_value : String ) -> bool
    {
        self.text = new_value;

        true
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
    let style = merge_style_with_templates( rtml_link.style, &rtml_link.style_template, context, templates );

    render_rtml_link_width_style( rtml_link, style, area, buf )
}

pub fn render_rtml_link_focus( 
    rtml_link : &RTMLLink,
    buf : &mut Buffer,
    templates : &HashMap<String, String>,
    context : &Value
) -> anyhow::Result<()>
{
    let focus_style = merge_style_with_templates( rtml_link.focus_style, &rtml_link.focus_style_template, context, templates );

    render_rtml_link_width_style( rtml_link, focus_style, rtml_link.common.attrs.area, buf )
}

fn render_rtml_link_width_style( 
    rtml_link : &RTMLLink,
    style : Style,
    area : Rect,
    buf : &mut Buffer
) -> anyhow::Result<()>
{
    clear_area( area, Style::default().not_underlined(), buf );

    let str = rtml_link.text.as_str();

    let line = Line::from( 
        Span::styled( str, style )
    )
    .alignment( rtml_link.alignment );

    line.render( area, buf );

    Ok( () )
}

