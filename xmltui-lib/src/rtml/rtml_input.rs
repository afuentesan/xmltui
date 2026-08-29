use ratatui::{buffer::Buffer, layout::{Alignment, Rect}, style::Style, text::{Line, Span}, widgets::Widget};

use crate::{app::event::{AppEvent, send_app_event}, input::event::InputEvent, rtml::{rtml_node::RTMLNodeCommon, util::{editable_value::{EditableValue, editable_value_to_spans}, rtml_event::RTMLEvent}}};


#[derive(Debug)]
pub struct RTMLInput
{
    pub common : RTMLNodeCommon,
    pub alignment : Alignment,
    value : EditableValue,
    pub style : Style,
    pub focus_style : Style,
    pub events : Vec<RTMLEvent>
}

impl RTMLInput
{
    pub fn new( 
        alignment : Alignment, 
        events : Vec<RTMLEvent>,
        value : EditableValue, 
        style : Style, 
        focus_style : Style, 
        common : RTMLNodeCommon 
    ) -> Self
    {
        Self { alignment, events, value, style, focus_style, common }
    }

    pub fn focus_event( &mut self, event : &InputEvent ) -> bool
    {
        match event
        {
            InputEvent::Left => self.move_cursor_left(),
            InputEvent::Right => self.move_cursor_right( self.common.attrs.area ),
            InputEvent::Char( ( _, c ) ) => self.add_char( *c, self.common.attrs.area ),
            InputEvent::Backspace => self.backspace( self.common.attrs.area ),
            InputEvent::Delete => self.delete( self.common.attrs.area ),
            InputEvent::End => self.end( self.common.attrs.area ),
            InputEvent::Home => self.home(),
            InputEvent::Enter => self.enter_event(),
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
        self.value.replace_value( new_value, self.common.attrs.area.width as usize );

        true
    }

    pub fn value( &self ) -> &str
    {
        &self.value.value
    }

    fn add_char( &mut self, char : char, area : Rect ) -> bool
    {
        self.value.add_char( char, area.width as usize )
    }

    fn move_cursor_right( &mut self, area : Rect ) -> bool
    {
        self.value.next_col( area.width as usize )
    }

    fn move_cursor_left( &mut self ) -> bool
    {
        self.value.prev_col()
    }

    fn backspace( &mut self, area : Rect ) -> bool
    {
        self.value.backspace( area.width as usize )      
    }

    fn delete( &mut self, area : Rect ) -> bool
    {
        self.value.delete( area.width as usize )
    }

    fn end( &mut self, area : Rect ) -> bool
    {
        self.value.end( area.width as usize )
    }

    fn home( &mut self ) -> bool
    {
        self.value.home()
    }
}

pub fn render_rtml_input( 
    rtml_input : &RTMLInput,
    area : Rect,
    buf : &mut Buffer
) -> anyhow::Result<()>
{
    let spans = if rtml_input.value.is_empty()
    {
        vec![ Span::styled( " ", rtml_input.style  ) ]
    } 
    else 
    {
        editable_value_to_spans( 
            &rtml_input.value, 
            area.width as usize, 
            rtml_input.style, 
            false
        )
    };

    let spans = if spans.len() == 0
    {
        vec![ Span::styled( " ", rtml_input.style  ) ]
    }
    else
    {
        spans
    };

    let line = Line::from( spans )
    .alignment( rtml_input.alignment )
    .style( rtml_input.style );

    line.render( area, buf );

    Ok( () )
}

pub fn render_input_cursor(
    rtml_input : &RTMLInput,
    buf : &mut Buffer
) -> anyhow::Result<()>
{
    let spans = editable_value_to_spans( 
        &rtml_input.value, 
        rtml_input.common.attrs.area.width as usize, 
        rtml_input.focus_style, 
        true
    );

    let spans = if spans.len() == 0
    {
        vec![ Span::styled( " ", rtml_input.focus_style ) ]
    }
    else
    {
        spans
    };

    let line = Line::from( spans )
    .alignment( rtml_input.alignment )
    .style( rtml_input.focus_style );

    line.render( rtml_input.common.attrs.area, buf );

    Ok( () )
}