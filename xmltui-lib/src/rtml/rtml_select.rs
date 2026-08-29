use ratatui::{buffer::Buffer, layout::{Alignment, Constraint, Layout}, style::Style, widgets::Widget};

use crate::{app::event::{AppEvent, send_app_event}, input::event::InputEvent, rtml::{rtml_node::RTMLNodeCommon, rtml_paragraph::lines_from_text_width_style, util::{rtml_event::RTMLEvent, types::TextLines}}, util::draw::clear_area};

#[derive(Debug)]
pub struct RTMLSelect 
{
    pub common : RTMLNodeCommon,
    pub alignment : Alignment,
    pub style : Style,
    pub focus_style : Style,
    pub selected_style : Style,
    pub lines : TextLines,
    pub values : Vec<String>,
    pub events : Vec<RTMLEvent>,
    selected_line : usize,
    start_at : usize
}

impl RTMLSelect
{
    pub fn new( 
        common : RTMLNodeCommon, 
        alignment : Alignment, 
        style : Style, 
        focus_style : Style, 
        selected_style : Style,
        lines : TextLines, 
        values : Vec<String>,
        events : Vec<RTMLEvent>,
        selected_line : usize
    ) -> Self
    {
        Self { common, alignment, style, focus_style, selected_style, lines, values, events, selected_line, start_at : 0 }
    }

    pub fn focus_event( &mut self, event : &InputEvent ) -> bool
    {
        match event
        {
            InputEvent::Up => self.move_up(),
            InputEvent::Down => self.move_down(),
            InputEvent::Enter => self.enter_event(),
            InputEvent::End => self.move_last(),
            InputEvent::Home => self.move_first(),
            _ => false
        }
    }

    fn move_down( &mut self ) -> bool
    {
        if self.values.len() == 0 { return false };

        let last = self.values.len() - 1;

        if self.selected_line >= last
        {
            return self.move_first();
        }

        self.selected_line += 1;

        self.move_start_at();

        true
    }

    fn move_start_at( &mut self )
    {
        if self.selected_line < self.start_at
        {
            self.start_at = self.selected_line;
        }

        let min_start_at = ( self.selected_line + 1 ).saturating_sub( self.common.attrs.area.height as usize );

        if self.start_at < min_start_at
        {
            self.start_at = min_start_at;
        }
    }

    fn move_up( &mut self ) -> bool
    {
        if self.values.len() == 0 { return false };

        if self.selected_line == 0
        {
            return self.move_last();
        }

        self.selected_line -= 1;

        self.move_start_at();

        true
    }

    fn move_first( &mut self ) -> bool
    {
        if self.values.len() == 0 || self.selected_line == 0 { return false };

        self.start_at = 0;
        self.selected_line = 0;

        true
    }

    fn move_last( &mut self ) -> bool
    {
        let last = self.values.len() - 1;

        if self.values.len() == 0 || self.selected_line == last { return false };

        self.selected_line = last;

        self.move_start_at();

        true
    }

    fn enter_event( &mut self ) -> bool
    {
        if self.values.len() == 0 { return false };
        
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

    // if rtml_select.selected_line < lines.len()
    // {
    //     lines[ rtml_select.selected_line ] = lines[ rtml_select.selected_line ].clone().style( rtml_select.selected_style );
    // }   

    let constraints = vec![ Constraint::Length( 1 ); lines.len() ];

    let areas = rtml_select.common.attrs.area.layout_vec( &Layout::vertical( constraints ) );

    lines.into_iter()
    .enumerate()
    .for_each( 
        | ( idx, mut line ) |
        {
            if idx == rtml_select.selected_line
            {
                line = line.style( rtml_select.selected_style );
            }
            else
            {
                line = line.style( style );    
            }

            line.alignment( rtml_select.alignment ).render( areas[ idx ], buf );
        }
    );
}