use std::collections::HashMap;

use ratatui::{buffer::Buffer, layout::{Alignment, Rect}, style::Style, text::{Line, Span}, widgets::Widget};
use serde_json::Value;

use crate::{app::event::{AppEvent, send_app_event}, input::event::InputEvent, rtml::{rtml_form::FieldAttrs, rtml_node::{FocusEventResponse, RTMLNodeCommon}, util::{editable_value::{EditableValue, editable_value_to_spans}, rtml_event::RTMLEvent, rtml_style::{RTMLStyleTemplate, merge_style_with_templates}}}};


#[derive(Debug)]
pub struct RTMLInput
{
    pub common : RTMLNodeCommon,
    pub alignment : Alignment,
    value : EditableValue,
    pub style : Style,
    pub style_template : RTMLStyleTemplate,
    pub focus_style : Style,
    pub focus_style_template : RTMLStyleTemplate,
    pub events : Vec<RTMLEvent>,
    pub field : FieldAttrs
}

impl RTMLInput
{
    pub fn new( 
        alignment : Alignment, 
        events : Vec<RTMLEvent>,
        value : EditableValue, 
        style : Style, 
        style_template : RTMLStyleTemplate,
        focus_style : Style, 
        focus_style_template : RTMLStyleTemplate,
        common : RTMLNodeCommon,
        field : FieldAttrs
    ) -> Self
    {
        Self { alignment, events, value, style, style_template, focus_style, focus_style_template, common, field }
    }

    pub fn focus_event( &mut self, event : &InputEvent ) -> FocusEventResponse
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
        self.value.replace_value( new_value, self.common.attrs.area.width as usize );

        true
    }

    pub fn state_value( &self ) -> ( String, Value )
    {
        (
            self.field.path.clone(),
            Value::String( self.value.value.clone() )
        )
    }

    fn add_char( &mut self, char : char, area : Rect ) -> FocusEventResponse
    {
        if self.value.add_char( char, area.width as usize )
        {
            self.create_focus_event_response_with_state()
        }
        else
        {
            FocusEventResponse::new_without_state( false )    
        }
    }

    fn create_focus_event_response_with_state( &self ) -> FocusEventResponse
    {
        let ( path, val ) = self.state_value();

        FocusEventResponse::new( true, Some( ( path, val ) ) )
    }

    fn move_cursor_right( &mut self, area : Rect ) -> FocusEventResponse
    {
        FocusEventResponse::new_without_state( self.value.next_col( area.width as usize ) )
    }

    fn move_cursor_left( &mut self ) -> FocusEventResponse
    {
        FocusEventResponse::new_without_state( self.value.prev_col() )
    }

    fn backspace( &mut self, area : Rect ) -> FocusEventResponse
    {
        if self.value.backspace( area.width as usize )
        {
            self.create_focus_event_response_with_state()
        }
        else
        {
            FocusEventResponse::new_without_state( false )    
        }
    }

    fn delete( &mut self, area : Rect ) -> FocusEventResponse
    {
        if self.value.delete( area.width as usize )
        {
            self.create_focus_event_response_with_state()
        }
        else
        {
            FocusEventResponse::new_without_state( false )    
        }
    }

    fn end( &mut self, area : Rect ) -> FocusEventResponse
    {
        FocusEventResponse::new_without_state( self.value.end( area.width as usize ) )
    }

    fn home( &mut self ) -> FocusEventResponse
    {
        FocusEventResponse::new_without_state( self.value.home() )
    }
}

pub fn render_rtml_input( 
    rtml_input : &RTMLInput,
    area : Rect,
    buf : &mut Buffer,
    templates : &HashMap<String, String>,
    context : &Value
) -> anyhow::Result<()>
{
    let style = merge_style_with_templates( rtml_input.style, &rtml_input.style_template, context, templates );

    let spans = if rtml_input.value.is_empty()
    {
        vec![ Span::styled( " ", style  ) ]
    } 
    else 
    {
        editable_value_to_spans( 
            &rtml_input.value, 
            area.width as usize, 
            style, 
            false
        )
    };

    let spans = if spans.len() == 0
    {
        vec![ Span::styled( " ", style  ) ]
    }
    else
    {
        spans
    };

    let line = Line::from( spans )
    .alignment( rtml_input.alignment )
    .style( style );

    line.render( area, buf );

    Ok( () )
}

pub fn render_input_cursor(
    rtml_input : &RTMLInput,
    buf : &mut Buffer,
    templates : &HashMap<String, String>,
    context : &Value
) -> anyhow::Result<()>
{
    let focus_style = merge_style_with_templates( rtml_input.focus_style, &rtml_input.focus_style_template, context, templates );

    let spans = editable_value_to_spans( 
        &rtml_input.value, 
        rtml_input.common.attrs.area.width as usize, 
        focus_style, 
        true
    );

    let spans = if spans.len() == 0
    {
        vec![ Span::styled( " ", focus_style ) ]
    }
    else
    {
        spans
    };

    let line = Line::from( spans )
    .alignment( rtml_input.alignment )
    .style( focus_style );

    line.render( rtml_input.common.attrs.area, buf );

    Ok( () )
}