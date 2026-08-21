use ratatui::{buffer::Buffer, layout::{Alignment, Rect}, style::Style, text::{Line, Span}, widgets::Widget};

use crate::{input::event::InputEvent, rtml::{rtml_node::RTMLNodeCommon, util::rtml_value::{RTMLValue, RTMLValueAttrs, RTMLValueType, rtml_value_to_spans}}};


#[derive(Debug)]
pub struct RTMLInput
{
    pub common : RTMLNodeCommon,
    pub alignment : Alignment,
    value : RTMLValue,
    pub style : Style,
    pub focus_style : Style
}

impl RTMLInput
{
    pub fn new( alignment : Alignment, value : RTMLValue, style : Style, focus_style : Style, common : RTMLNodeCommon ) -> Self
    {
        Self { alignment, value, style, focus_style, common }
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
            _ => false
        }
    }

    pub fn replace_value( &mut self, new_value : String ) -> bool
    {
        self.value = RTMLValue::Write( 
            RTMLValueAttrs::new(
                Some( 1 ),
                RTMLValueType::new_string(
                    new_value,
                    true,
                    false
                )
            )
        );

        true
    }

    fn add_char( &mut self, char : char, area : Rect ) -> bool
    {
        self.value.add_char( char, area )
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
        self.value.end( area )
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
    let mut spans = rtml_value_to_spans( 
        &rtml_input.value, 
        area, 
        rtml_input.style, 
        false
    );

    let spans = if spans.len() == 0
    {
        vec![ Span::from( " " ).style( rtml_input.focus_style ) ]
    }
    else
    {
        let mut spans = spans.remove( 0 );

        if spans.len() == 0 { spans.push( Span::from( " " ).style( rtml_input.focus_style ) ); }

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
    let mut spans = rtml_value_to_spans( 
        &rtml_input.value, 
        rtml_input.common.attrs.area, 
        rtml_input.focus_style, 
        true
    );

    let spans = if spans.len() == 0
    {
        vec![ Span::from( " " ).style( rtml_input.focus_style ) ]
    }
    else
    {
        let mut spans = spans.remove( 0 );

        if spans.len() == 0 { spans.push( Span::from( " " ).style( rtml_input.focus_style ) ); }

        spans
    };

    let line = Line::from( spans )
    .alignment( rtml_input.alignment )
    .style( rtml_input.focus_style );

    line.render( rtml_input.common.attrs.area, buf );

    Ok( () )
}