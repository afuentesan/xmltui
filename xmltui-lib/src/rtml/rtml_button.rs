use std::collections::HashMap;

use ratatui::{buffer::Buffer, layout::{Alignment, Rect}, style::Style, text::{Line, Span}, widgets::Widget};
use serde_json::Value;

use crate::{app::event::{AppEvent, send_app_event}, input::event::InputEvent, rtml::{rtml_node::{FocusEventResponse, RTMLNodeCommon}, util::{rtml_event::RTMLEvent, rtml_style::{RTMLStyleTemplate, merge_style_with_templates}}}, util::draw::clear_area};


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
    pub text : String
}

impl RTMLButton
{
    pub fn new(
        alignment : Alignment,
        events : Vec<RTMLEvent>,
        text : String,
        style : Style,
        style_template : RTMLStyleTemplate,
        focus_style : Style,
        focus_style_template : RTMLStyleTemplate,
        common : RTMLNodeCommon
    ) -> Self
    {
        Self { common, alignment, style, style_template, focus_style, focus_style_template, events, text }
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

    pub fn replace_value( &mut self, new_value : String ) -> bool
    {
        self.text = new_value;

        true
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
    let style = merge_style_with_templates( rtml_button.style, &rtml_button.style_template, context, templates );

    render_rtml_button_width_style( rtml_button, style, area, buf )
}

pub fn render_rtml_button_focus( 
    rtml_button : &RTMLButton,
    buf : &mut Buffer,
    templates : &HashMap<String, String>,
    context : &Value
) -> anyhow::Result<()>
{
    let focus_style = merge_style_with_templates( rtml_button.focus_style, &rtml_button.focus_style_template, context, templates );

    render_rtml_button_width_style( rtml_button, focus_style, rtml_button.common.attrs.area, buf )
}

fn render_rtml_button_width_style( 
    rtml_button : &RTMLButton,
    style : Style,
    area : Rect,
    buf : &mut Buffer
) -> anyhow::Result<()>
{
    clear_area( area, Style::default().not_underlined(), buf );

    let str = rtml_button.text.as_str();

    let line = Line::from( 
        Span::styled( str, style )
    )
    .alignment( rtml_button.alignment );

    line.render( area, buf );

    Ok( () )
}