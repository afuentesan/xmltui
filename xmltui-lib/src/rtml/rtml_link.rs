use ratatui::{buffer::Buffer, layout::{Alignment, Rect}, style::Style, text::{Line, Span}, widgets::Widget};

use crate::{app::event::{AppEvent, send_app_event}, input::event::InputEvent, rtml::{rtml_node::RTMLNodeCommon, rtml_source::RTMLSource}, util::draw::clear_area};


#[derive(Debug)]
pub struct RTMLLink
{
    pub common : RTMLNodeCommon,
    pub alignment : Alignment,
    pub style : Style,
    pub focus_style : Style,
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
        focus_style : Style,
        common : RTMLNodeCommon
    ) -> Self
    {
        Self { common, alignment, style, focus_style, source, text }
    }

    pub fn focus_event( &mut self, event : &InputEvent ) -> bool
    {
        match event
        {
            InputEvent::Enter =>
            {
                send_app_event(
                    AppEvent::LoadFile( self.source.source().to_string() )
                );

                false
            },
            _ => false
        }
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

pub fn render_rtml_link( 
    rtml_link : &RTMLLink,
    area : Rect,
    buf : &mut Buffer
) -> anyhow::Result<()>
{
    render_rtml_link_width_style( rtml_link, rtml_link.style, area, buf )
}

pub fn render_rtml_link_focus( 
    rtml_link : &RTMLLink,
    buf : &mut Buffer
) -> anyhow::Result<()>
{
    render_rtml_link_width_style( rtml_link, rtml_link.focus_style, rtml_link.common.attrs.area, buf )
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

