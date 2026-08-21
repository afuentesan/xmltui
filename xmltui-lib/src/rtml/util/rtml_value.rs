use std::str::FromStr;

use ratatui::{layout::Rect, style::{Color, Style}, text::Span};
use unicode_segmentation::UnicodeSegmentation;

use crate::util::str::{str_len, substr};

#[derive(Debug)]
pub enum RTMLValueType
{
    Text( Vec<String> )
}

impl RTMLValueType
{
    pub fn new_string( str : String, remove_empty_lines : bool, trim_values : bool ) -> Self
    {
        let mut val = str_to_vec( str, remove_empty_lines, trim_values );

        if val.len() == 0 { val.push( "".to_string() ); }

        RTMLValueType::Text( val )
    }

    fn lines( &self ) -> &Vec<String>
    {
        match self
        {
            RTMLValueType::Text( v ) => v
        }
    }

    fn lines_mut( &mut self ) -> &mut Vec<String>
    {
        match self
        {
            RTMLValueType::Text( v ) => v
        }
    }

    pub fn delete_char( &mut self, row : usize, col : usize ) -> bool
    {
        let lines = self.lines_mut();

        if row >= lines.len() { return false };

        let new_line = lines[ row ].graphemes( true )
        .enumerate()
        .filter_map(
            | ( i, s ) | 
            {
                if i == col
                {
                    None
                }
                else
                {
                    Some( s )    
                }
            }
        )
        .collect::<String>();

        lines[ row ] = new_line;

        true
    }
}

#[derive(Debug)]
pub struct RTMLValueAttrs
{
    max_rows : Option<usize>,

    col_idx : usize,
    row_idx : usize,

    col_cursor : usize,
    row_cursor : usize,

    value : RTMLValueType
}

impl RTMLValueAttrs
{
    pub fn new(
        max_rows : Option<usize>,
        value : RTMLValueType
    ) -> Self
    {
        RTMLValueAttrs { max_rows, col_idx : 0, row_idx : 0, col_cursor : 0, row_cursor : 0, value }
    }
}

#[derive(Debug)]
pub enum RTMLValue 
{
    Read( RTMLValueAttrs ),
    Write( RTMLValueAttrs )
}

impl RTMLValue
{
    pub fn next_col( &mut self, width : usize ) -> bool
    {
        // TODO: Pasar a la siguiente línea si estás al final y hay más líneas disponibles
        let attrs = self.attrs_mut();

        let lines = attrs.value.lines();

        if attrs.row_idx >= lines.len()
        {
            // TODO: Mostrar algún error
            return false;
        }

        let max_len = str_len( &lines[ attrs.row_idx ] );

        if attrs.col_cursor >= max_len { return false };

        attrs.col_cursor += 1;

        let diff = attrs.col_cursor - attrs.col_idx;

        if diff >= width
        {
            attrs.col_idx = attrs.col_cursor + 1 - width;
        }

        true
    }

    pub fn prev_col( &mut self ) -> bool
    {
        // TODO: Ir a la línea anterior si estás al principio y hay línea previa

        let attrs = self.attrs_mut();

        if attrs.col_cursor == 0 { return false };

        attrs.col_cursor -= 1;

        if attrs.col_cursor < attrs.col_idx
        {
            attrs.col_idx = attrs.col_cursor;
        }

        true
    }

    pub fn next_row( &mut self, height : usize ) -> bool
    {
        // TODO: Reposicionar las posiciones de la columna si la línea tiene menor longitud
        let attrs = self.attrs_mut();

        let lines = attrs.value.lines();

        let max_len = lines.len();

        if attrs.row_cursor >= max_len { return false };

        attrs.row_cursor += 1;

        let diff = attrs.row_cursor - attrs.row_idx;

        if diff >= height
        {
            attrs.row_idx = attrs.row_cursor + 1 - height;
        }

        true
    }

    pub fn prev_row( &mut self ) -> bool
    {
        // TODO: Reposicionar las posiciones de la columna si la línea tiene menor longitud
        let attrs = self.attrs_mut();

        if attrs.row_cursor == 0 { return false };

        attrs.row_cursor -= 1;

        if attrs.row_cursor < attrs.row_idx
        {
            attrs.row_idx = attrs.row_cursor;
        }

        true
    }

    pub fn backspace( &mut self, width : usize ) -> bool
    {
        // TODO: Implementar que pase a la fila anterior si el col es 0
        // TODO: Si hay caracteres ocultos al principio y se ha hecho espacio mostrar los caracteres del principio

        let attrs = self.attrs_mut();

        // TODO: Implementar que pase a la fila anterior si el col es 0
        if attrs.col_cursor == 0 { return false; }

        let delete_char = attrs.col_cursor - 1;

        if ! attrs.value.delete_char( attrs.row_cursor, delete_char ) { return false };

        self.prev_col();

        self.move_col_on_delete( width );

        true
    }

    pub fn delete( &mut self, width : usize ) -> bool
    {
        // TODO: Implementar que borre la fila siguiente si el col es len(line)
        // TODO: Si hay caracteres ocultos al principio y se ha hecho espacio mostrar los caracteres del principio

        let attrs = self.attrs_mut();

        if attrs.value.delete_char( attrs.row_cursor, attrs.col_cursor + 1 )
        {
            self.move_col_on_delete( width );

            true
        }
        else
        {
            false    
        }
    }

    fn move_col_on_delete( &mut self, width : usize )
    {
        let attrs = self.attrs_mut();

        if attrs.col_idx == 0 { return };

        let len = str_len( &attrs.value.lines()[ attrs.row_cursor ] );

        if len >= attrs.col_idx && len >= width && ( len - attrs.col_idx ) < width
        {
            attrs.col_idx = len - width;
        }
    }

    pub fn home( &mut self ) -> bool
    {
        let attrs = self.attrs_mut();

        if attrs.col_cursor == 0 && attrs.row_cursor == 0
        {
            false
        }
        else
        {
            attrs.col_cursor = 0;
            attrs.row_cursor = 0;
            attrs.row_idx = 0;
            attrs.col_idx = 0;

            true    
        }
    }

    pub fn end( &mut self, area : Rect ) -> bool
    {
        let attrs = self.attrs_mut();

        let lines = attrs.value.lines();

        if lines.len() == 0 { return false };

        let len_last = str_len( &lines[ lines.len() - 1 ] );

        if attrs.row_idx >= lines.len() && attrs.col_idx >= len_last
        {
            false
        }
        else
        {
            attrs.row_cursor = lines.len() - 1;
            attrs.col_cursor = len_last;

            let diff = attrs.col_cursor - attrs.col_idx;

            if diff >= area.width as usize
            {
                attrs.col_idx = attrs.col_cursor + 1 - area.width as usize;
            }

            let diff = attrs.row_cursor - attrs.row_idx;

            if diff >= area.height as usize
            {
                attrs.row_idx = attrs.row_cursor + 1 - area.height as usize;
            }

            true   
        }
    }

    pub fn add_char( &mut self, char : char, area : Rect ) -> bool
    {
        let attrs = self.attrs_mut();

        let lines = attrs.value.lines_mut();

        if attrs.row_cursor >= lines.len()
        {
            // TODO: Mostrar algún error
            return false;
        }

        let mut new_row = String::from( "" );

        if attrs.col_cursor >= str_len( &lines[ attrs.row_cursor ] )
        {
            new_row = format!( "{}{}", lines[ attrs.row_cursor ], char );
        }
        else
        {
            lines[ attrs.row_cursor ].graphemes( true )
            .enumerate()
            .for_each(
                | ( idx, str ) |
                {
                    if idx == attrs.col_cursor
                    {
                        new_row.push( char );
                        new_row.push_str( str );
                    }
                    else
                    {
                        new_row.push_str( str );    
                    }
                }
            );
        }

        lines[ attrs.row_cursor ] = new_row;

        self.next_col( area.width as usize );

        true
    }

    fn visible_text( &self, width : usize, height : usize ) -> Vec<&str>
    {
        let mut ret = vec![];

        let attrs = self.attrs();

        let lines = attrs.value.lines();

        let max_rows = if let Some( m ) = attrs.max_rows
        {
            m
        }
        else
        {
            usize::MAX   
        };

        let min_row_idx = attrs.row_idx;
        let min_col_idx = attrs.col_idx;

        let max_row_idx = ( min_row_idx + height ).min( max_rows );
        let max_col_idx = min_col_idx + width;

        let lines_len = lines.len();

        for idx in min_row_idx..max_row_idx
        {
            if idx >= lines_len { break };

            let line = &lines[ idx ];

            ret.push( substr( line, min_col_idx, max_col_idx ) );
        }

        ret
    }

    fn attrs( &self ) -> &RTMLValueAttrs
    {
        match self
        {
            RTMLValue::Read( a ) | RTMLValue::Write( a ) => a
        }
    }

    fn attrs_mut( &mut self ) -> &mut RTMLValueAttrs
    {
        match self
        {
            RTMLValue::Read( a ) | RTMLValue::Write( a ) => a
        }
    }
}

fn str_to_vec( 
    str : String,
    remove_empty_lines : bool,
    trim_values : bool
) -> Vec<String>
{
    str.split( "\n" )
    .filter_map(
        | s |
        {
            if remove_empty_lines && s.trim() == ""
            {
                None
            }
            else if trim_values
            {
                Some( s.trim().to_string() )
            }
            else
            {
                Some( s.to_string() )
            }
        }
    )
    .collect()
}

pub fn rtml_value_to_spans( 
    value : &RTMLValue, 
    area : Rect, 
    style : Style,
    show_cursor : bool
) -> Vec<Vec<Span<'_>>>
{
    let text = value.visible_text( area.width as usize, area.height as usize );

    let attrs = value.attrs();

    let col_cursor = attrs.col_cursor - attrs.col_idx;
    let row_cursor = attrs.row_cursor - attrs.row_idx;

    text
    .iter()
    .enumerate()
    .map(
        | ( i, s ) |
        {
            if show_cursor && i == row_cursor
            {
                let mut spans = s.graphemes( true )
                .enumerate()
                .map(
                    | ( i, s ) |
                    {
                        if i == col_cursor
                        {
                            Span::from( str_to_space_if_empty( s ) ).style( cursor_style( style ) )
                        }
                        else
                        {
                            Span::from( str_to_space_if_empty( s ) ).style( style )
                        }
                    }
                )
                .collect::<Vec<_>>();

                if spans.len() <= col_cursor
                {
                    spans.push( Span::from( " " ).style( cursor_style( style ) ) );
                }

                spans
            }
            else
            {
                vec![ Span::from( str_to_space_if_empty( s ) ).style( style ) ]
            }
        }
    )
    .collect()
}

fn str_to_space_if_empty( s : &str ) -> &str
{
    if s.len() == 0 { " " } else { s }
}

fn cursor_style( style : Style ) -> Style
{
    match ( style.fg, style.bg )
    {
        ( Some( fg ), Some( bg ) ) =>
        {
            style.bg( fg ).fg( bg )
        },
        _ =>
        {
            style.bg( Color::from_str( "#ffffff" ).unwrap_or( Color::White ) ).fg( Color::from_str( "#000000" ).unwrap_or( Color::Black ) )
        }
    }
}