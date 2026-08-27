use ratatui::{buffer::Buffer, layout::Rect, style::Style, text::{Line, Span}, widgets::{Paragraph, Widget, Wrap}};

use crate::{input::event::InputEvent, rtml::rtml_node::RTMLNodeCommon};

#[derive(Debug)]
pub struct RTMLParagraph 
{
    pub common : RTMLNodeCommon,
    pub style : Style,
    pub lines : Vec<Vec<( String, Option<Style> )>>,
    pub start_at : usize,
    pub num_lines : usize
}

impl RTMLParagraph
{
    pub fn new( common : RTMLNodeCommon, style : Style, lines : Vec<Vec<( String, Option<Style> )>> ) -> Self
    {
        Self { common, style, lines, start_at : 0, num_lines : 0 }
    }

    pub fn focus_event( &mut self, event : &InputEvent ) -> bool
    {
        match event
        {
            InputEvent::Up =>
            {
                if self.start_at == 0 { return false };

                self.start_at -= 1;

                true
            },
            InputEvent::Down =>
            {
                let max_start_at = self.num_lines.saturating_sub( self.common.attrs.area.height as usize );

                if self.start_at >= max_start_at
                {
                    false
                }
                else
                {
                    self.start_at += 1;

                    true    
                }
            },
            _ => false
        }
    }
}

pub fn render_rtml_paragraph( 
    rtml_paragraph : &RTMLParagraph,
    area : Rect,
    buf : &mut Buffer
) -> anyhow::Result<()>
{
    let mut paragraph = create_paragraph( rtml_paragraph );

    if rtml_paragraph.start_at > 0
    {
        paragraph = paragraph.scroll( ( rtml_paragraph.start_at as u16, 0 ) );
    }

    paragraph.render( area, buf );

    Ok( () )
}

pub fn create_paragraph<'a>( 
    rtml_paragraph : &'a RTMLParagraph
) -> Paragraph<'a>
{
    let lines = rtml_paragraph.lines
    .iter()
    .map(
        | l |
        {
            line_from_spans( l )
        }
    ).collect::<Vec<_>>();

    Paragraph::new( lines )
    .style( rtml_paragraph.style )
    .wrap( Wrap { trim: false } )
}

fn line_from_spans<'a>( spans : &'a Vec<( String, Option<Style> )> ) -> Line<'a>
{
    let content = spans.iter()
    .map(
        | ( text, style ) |
        {
            match style
            {
                Some( s ) => Span::styled( text.as_str(), *s ),
                None => Span::raw( text.as_str() )
            }
        }
    )
    .collect::<Vec<_>>();

    Line::from( content )
}