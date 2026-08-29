
use ratatui::{style::{Modifier, Style}, text::Span};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::util::str::{str_len, substr, take_width};

#[derive(Debug)]
pub struct EditableValue
{
    cursor_position : usize,
    start_at : usize,
    pub value : String
}

impl EditableValue
{
    pub fn new( value : String ) -> Self
    {
        Self { cursor_position : 0, start_at : 0, value }
    }

    pub fn is_empty( &self ) -> bool
    {
        self.value.is_empty()
    }

    pub fn next_col( &mut self, width : usize ) -> bool
    {
        let len = str_len( &self.value );

        if self.cursor_position >= len { return false };

        self.cursor_position += 1;

        self.move_start_at_on_next( width );

        true
    }

    pub fn prev_col( &mut self ) -> bool
    {
        if self.cursor_position == 0 { return false };

        self.cursor_position -= 1;

        self.calcula_start_at_on_back();

        true
    }

    pub fn backspace( &mut self, width : usize ) -> bool
    {
        if self.cursor_position == 0 { return false };

        let remove_position = self.cursor_position - 1;

        self.remove_char_position( remove_position );

        self.cursor_position = remove_position;

        self.calcula_start_at_on_back();

        self.calcula_start_at_on_delete( width );

        true
    }

    pub fn delete( &mut self, width : usize ) -> bool
    {
        let len = str_len( &self.value );

        if self.cursor_position >= len { return false };

        let remove_position = self.cursor_position;

        self.remove_char_position( remove_position );

        self.calcula_start_at_on_delete( width );

        true
    }

    pub fn home( &mut self ) -> bool
    {
        // Si cursor_position es 0 start_at también será 0 así que no lo compruebo
        if self.cursor_position == 0 { return false };

        self.cursor_position = 0;
        self.start_at = 0;

        true
    }

    pub fn end( &mut self, width : usize ) -> bool
    {
        let len = str_len( &self.value );

        if self.cursor_position >= len { return false };

        self.cursor_position = len;

        self.move_start_at_on_next( width );

        true
    }

    pub fn add_char( &mut self, char : char, width : usize ) -> bool
    {
        let len = str_len( &self.value );

        if self.cursor_position >= len
        {
            self.value.push( char );

            self.cursor_position = len + 1;

            self.move_start_at_on_next( width );
        }
        else
        {
            self.value = self.value.graphemes( true )
            .enumerate()
            .fold(
                String::with_capacity( self.value.len() ), 
                | mut acc, ( i, s ) |
                {
                    if i == self.cursor_position
                    {
                        acc.push( char );
                    }

                    acc.push_str( s );

                    acc
                }
            );

            self.cursor_position += 1;
        }

        self.move_start_at_on_next( width );

        true
    }

    pub fn replace_value( &mut self, new_value : String, width : usize )
    {
        let len = str_len( &new_value );

        if self.cursor_position > len { self.cursor_position = len };

        self.start_at = 0;

        self.value = new_value;

        self.move_start_at_on_next( width );
    }

    fn move_start_at_on_next( &mut self, width : usize )
    {
        loop 
        {
            let visible_text = substr( &self.value, self.start_at, self.cursor_position );
            let text_width = visible_text.width();

            let len = str_len( &self.value );

            let cursor_width = if self.cursor_position >= len
            { 
                1 
            } 
            else 
            { 
                substr( &self.value, self.cursor_position, self.cursor_position + 1 ).width() 
            };

            if( text_width + cursor_width ) > width 
            {
                self.start_at += 1;
            } 
            else 
            {
                break;
            }
        }
    }

    fn remove_char_position( &mut self, remove_position : usize )
    {
        let new_val : String= self.value.graphemes( true )
        .enumerate()
        .fold(
            String::with_capacity( self.value.len() ), 
            | mut acc, ( i, c ) |
            {
                if i != remove_position
                {
                    acc.push_str( c );
                }

                acc
            }
        );

        self.value = new_val;
    }

    fn calcula_start_at_on_delete( &mut self, width : usize )
    {
        let len = str_len( &self.value );
        
        while self.start_at > 0 
        {
            let next_start = self.start_at - 1;
            let visible_text = substr( &self.value, next_start, len );
            
            let extra = if self.cursor_position >= len { 1 } else { 0 };

            if( visible_text.width() + extra ) <= width 
            {
                self.start_at = next_start;
            } 
            else 
            {
                break;
            }
        }
    }

    fn calcula_start_at_on_back( &mut self )
    {
        if self.cursor_position < self.start_at
        {
            self.start_at = self.cursor_position;
        }
    }
}

pub fn editable_value_to_spans(
    value : &EditableValue,
    width : usize,
    style : Style,
    show_cursor : bool
) -> Vec<Span<'_>>
{
    let len = str_len( &value.value );
    let mut ret = vec![];

    if show_cursor
    {
        let pre = substr( &value.value, value.start_at, value.cursor_position );
        let cursor_mas_1 = value.cursor_position + 1;

        let cursor = if value.cursor_position >= len 
        {
            " "
        } 
        else 
        {
            substr( &value.value, value.cursor_position, cursor_mas_1 )
        };

        let post = if cursor_mas_1 >= len 
        {
            ""
        } 
        else 
        {
            let pre_width = pre.width();
            let cursor_width = cursor.width();
            
            let available_width = width.saturating_sub( pre_width + cursor_width );
            
            let rest_of_text = substr( &value.value, cursor_mas_1, len );

            take_width( rest_of_text, available_width )
        };
        
        if ! pre.is_empty() 
        {
            ret.push( Span::styled( pre, style ) );
        }

        ret.push( Span::styled( cursor, cursor_style( style ) ) );

        if ! post.is_empty() 
        {
            ret.push( Span::styled( post, style ) );
        }
    }
    else
    {
        let full_text = substr( &value.value, value.start_at, len );
        let visible_text = take_width( full_text, width );
        
        ret.push( Span::styled( visible_text, style ) );
    }

    ret
}

fn cursor_style( style : Style ) -> Style
{
    if style.has_modifier( Modifier::REVERSED )
    {
        style.remove_modifier( Modifier::REVERSED )
    }
    else
    {
        style.add_modifier( Modifier::REVERSED )    
    }
}