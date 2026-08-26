use ratatui::{buffer::Buffer, layout::{Alignment, Rect}, style::Style, text::{Line, Span}, widgets::Widget};

use crate::{app::event::{AppEvent, send_app_event}, input::event::InputEvent, rtml::{rtml_node::RTMLNodeCommon, util::rtml_event::RTMLEvent}, util::draw::clear_area};


#[derive(Debug)]
pub struct RTMLButton
{
    pub common : RTMLNodeCommon,
    pub alignment : Alignment,
    pub style : Style,
    pub focus_style : Style,
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
        focus_style : Style,
        common : RTMLNodeCommon
    ) -> Self
    {
        Self { common, alignment, style, focus_style, events, text }
    }

    pub fn focus_event( &mut self, event : &InputEvent ) -> bool
    {
        match event
        {
            InputEvent::Enter =>
            {
                self.enter_event()
            },
            _ => false
        }
    }

    fn enter_event( &mut self ) -> bool
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

        false
    }

    pub fn replace_value( &mut self, new_value : String ) -> bool
    {
        self.text = new_value;

        true
    }

    pub fn value( &self ) -> &str
    {
        &self.text
    }
}

pub fn render_rtml_button( 
    rtml_button : &RTMLButton,
    area : Rect,
    buf : &mut Buffer
) -> anyhow::Result<()>
{
    render_rtml_button_width_style( rtml_button, rtml_button.style, area, buf )
}

pub fn render_rtml_button_focus( 
    rtml_button : &RTMLButton,
    buf : &mut Buffer
) -> anyhow::Result<()>
{
    render_rtml_button_width_style( rtml_button, rtml_button.focus_style, rtml_button.common.attrs.area, buf )
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
        Span::from( str ).style( style )
    )
    .alignment( rtml_button.alignment );

    line.render( area, buf );

    Ok( () )
}