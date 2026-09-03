use ratatui::{buffer::Buffer, layout::{Alignment, Constraint, Layout, Rect}, style::Style, widgets::Widget};
use serde_json::Value;

use crate::{app::event::{AppEvent, send_app_event}, input::event::InputEvent, rtml::{rtml_form::FieldAttrs, rtml_node::{FocusEventResponse, RTMLNodeCommon}, rtml_padding::RTMLPadding, rtml_paragraph::lines_from_text_width_style, util::{rtml_event::RTMLEvent, types::TextLines}}, util::draw::clear_area};

#[derive(Debug)]
pub struct RTMLSelect 
{
    pub common : RTMLNodeCommon,
    pub padding : RTMLPadding,
    pub alignment : Alignment,
    pub style : Style,
    pub focus_style : Style,
    pub selected_style : Style,
    pub lines : TextLines,
    pub values : Vec<String>,
    pub events : Vec<RTMLEvent>,
    selected_line : usize,
    start_at : usize,
    pub inner_area : Rect,
    pub field : FieldAttrs
}

impl RTMLSelect
{
    pub fn new( 
        common : RTMLNodeCommon, 
        padding : RTMLPadding,
        alignment : Alignment, 
        style : Style, 
        focus_style : Style, 
        selected_style : Style,
        lines : TextLines, 
        values : Vec<String>,
        events : Vec<RTMLEvent>,
        selected_line : usize,
        field : FieldAttrs
    ) -> Self
    {
        Self { common, padding, alignment, style, focus_style, selected_style, lines, values, events, selected_line, start_at : 0, inner_area : Rect::default(), field }
    }

    pub fn focus_event( &mut self, event : &InputEvent ) -> FocusEventResponse
    {
        match event
        {
            InputEvent::Up => self.move_up(),
            InputEvent::Down => self.move_down(),
            InputEvent::Enter => self.enter_event(),
            InputEvent::End => self.move_last(),
            InputEvent::Home => self.move_first(),
            _ => FocusEventResponse::new_without_state( false )
        }
    }

    fn move_down( &mut self ) -> FocusEventResponse
    {
        if self.values.len() == 0 { return FocusEventResponse::new_without_state( false ) };

        let last = self.values.len() - 1;

        if self.selected_line >= last
        {
            return self.move_first();
        }

        self.selected_line += 1;

        self.move_start_at();

        self.create_focus_event_response_with_state()
    }

    fn move_start_at( &mut self )
    {
        if self.selected_line < self.start_at
        {
            self.start_at = self.selected_line;
        }

        let min_start_at = ( self.selected_line + 1 ).saturating_sub( self.inner_area.height as usize );

        if self.start_at < min_start_at
        {
            self.start_at = min_start_at;
        }
    }

    fn move_up( &mut self ) -> FocusEventResponse
    {
        if self.values.len() == 0 { return FocusEventResponse::new_without_state( false ) };

        if self.selected_line == 0
        {
            return self.move_last();
        }

        self.selected_line -= 1;

        self.move_start_at();

        self.create_focus_event_response_with_state()
    }

    fn create_focus_event_response_with_state( &self ) -> FocusEventResponse
    {
        let ( path, val ) = self.state_value();

        FocusEventResponse::new( true, Some( ( path, val ) ) )
    }

    fn move_first( &mut self ) -> FocusEventResponse
    {
        if self.values.len() == 0 || self.selected_line == 0 { return FocusEventResponse::new_without_state( false ) };

        self.start_at = 0;
        self.selected_line = 0;

        self.create_focus_event_response_with_state()
    }

    fn move_last( &mut self ) -> FocusEventResponse
    {
        let last = self.values.len() - 1;

        if self.values.len() == 0 || self.selected_line == last { return FocusEventResponse::new_without_state( false ) };

        self.selected_line = last;

        self.move_start_at();

        self.create_focus_event_response_with_state()
    }

    fn enter_event( &mut self ) -> FocusEventResponse
    {
        if self.values.len() == 0 { return FocusEventResponse::new_without_state( false ) };
        
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
        for ( idx, val ) in self.values.iter().enumerate()
        {
            if val == &new_value
            {
                self.selected_line = idx;

                return true;
            }
        }

        false
    }

    pub fn value( &self ) -> &str
    {
        if self.selected_line < self.values.len()
        {
            &self.values[ self.selected_line ]
        }
        else
        {
            ""    
        }
    }

    pub fn state_value( &self ) -> ( String, Value )
    {
        (
            self.field.path.clone(),
            Value::String( self.value().to_string() )
        )
    }
}

pub fn render_rtml_select( 
    rtml_select : &RTMLSelect,
    buf : &mut Buffer
) -> anyhow::Result<()>
{
    render_options( rtml_select, rtml_select.style, buf );

    Ok( () )
}

pub fn render_rtml_select_focus( 
    rtml_select : &RTMLSelect,
    buf : &mut Buffer
) -> anyhow::Result<()>
{
    render_options( rtml_select, rtml_select.focus_style, buf );

    Ok( () )
}

fn render_options( 
    rtml_select : &RTMLSelect, 
    style : Style,
    buf : &mut Buffer
)
{
    clear_area( rtml_select.common.attrs.area, style, buf );
    
    let lines = lines_from_text_width_style( 
        &rtml_select.lines, 
        Some( ( rtml_select.selected_line, rtml_select.selected_style ) ) 
    );

    let line_overflow = rtml_select.start_at + rtml_select.inner_area.height as usize;

    let constraints = vec![ Constraint::Length( 1 ); rtml_select.inner_area.height as usize ];

    let areas = rtml_select.inner_area.layout_vec( &Layout::vertical( constraints ) );

    let mut area_idx = 0;

    for ( idx, mut line ) in lines.into_iter().enumerate().skip( rtml_select.start_at )
    {
        if idx >= line_overflow { break };

        if idx == rtml_select.selected_line
        {
            line = line.style( rtml_select.selected_style );
        }
        else
        {
            line = line.style( style );    
        }

        line.alignment( rtml_select.alignment ).render( areas[ area_idx ], buf );

        area_idx += 1;
    }
}