use std::collections::HashMap;

use ratatui::{buffer::Buffer, layout::{Alignment, Rect}, style::Style};
use serde_json::Value;

use crate::{app::event::{AppEvent, send_app_event}, input::event::InputEvent, rtml::{rtml_line::render_text_line, rtml_node::{FocusEventResponse, RTMLNodeCommon}, rtml_padding::HorizontalPadding, util::{rtml_event::RTMLEvent, rtml_style::RTMLStyleTemplate, types::TextLine}}};


#[derive(Debug)]
pub struct RTMLButton
{
    pub common : RTMLNodeCommon,
    pub alignment : Alignment,
    pub style : Style,
    pub style_template : RTMLStyleTemplate,
    pub focus_style : Style,
    pub focus_style_template : RTMLStyleTemplate,
    pub events : Vec<RTMLEvent>,
    pub padding : HorizontalPadding,
    pub content : TextLine
}

impl RTMLButton
{
    pub fn new(
        alignment : Alignment,
        events : Vec<RTMLEvent>,
        padding : HorizontalPadding,
        content : TextLine,
        style : Style,
        style_template : RTMLStyleTemplate,
        focus_style : Style,
        focus_style_template : RTMLStyleTemplate,
        common : RTMLNodeCommon
    ) -> Self
    {
        Self { common, alignment, style, style_template, focus_style, focus_style_template, events, padding, content }
    }

    pub fn focus_event( &mut self, event : &InputEvent ) -> FocusEventResponse
    {
        match event
        {
            InputEvent::Enter =>
            {
                self.enter_event()
            },
            _ => FocusEventResponse::new_without_state( false )
        }
    }

    fn enter_event( &mut self ) -> FocusEventResponse
    {
        for ev in &self.events
        {
            match ev
            {
                RTMLEvent::Enter( e ) =>
                {
                    send_app_event( AppEvent::Callback( e.clone() ) );

                    break;
                }    
            }
        }

        FocusEventResponse::new_without_state( false )
    }
}

pub fn render_rtml_button( 
    rtml_button : &RTMLButton,
    area : Rect,
    buf : &mut Buffer,
    templates : &HashMap<String, String>,
    context : &Value
) -> anyhow::Result<()>
{
    render_rtml_button_width_style( rtml_button, rtml_button.style, &rtml_button.style_template, area, buf, templates, context )
}

pub fn render_rtml_button_focus( 
    rtml_button : &RTMLButton,
    buf : &mut Buffer,
    templates : &HashMap<String, String>,
    context : &Value
) -> anyhow::Result<()>
{
    render_rtml_button_width_style( rtml_button, rtml_button.focus_style, &rtml_button.focus_style_template, rtml_button.common.attrs.area, buf, templates, context )
}

fn render_rtml_button_width_style( 
    rtml_button : &RTMLButton,
    style : Style,
    style_template : &RTMLStyleTemplate,
    area : Rect,
    buf : &mut Buffer,
    templates : &HashMap<String, String>,
    context : &Value
) -> anyhow::Result<()>
{
    render_text_line( style, style_template, &rtml_button.padding, &rtml_button.alignment, &rtml_button.content, area, buf, templates, context)
}