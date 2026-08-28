use ratatui::{buffer::Buffer, layout::Rect, style::Style, text::{Line, Span}, widgets::{Paragraph, Widget, Wrap}};

use crate::{input::event::InputEvent, rtml::{rtml_node::RTMLNodeCommon, util::types::TextLines}, xml::styles::xml_style::merge_styles};

#[derive(Debug)]
pub struct RTMLParagraph 
{
    pub common : RTMLNodeCommon,
    pub style : Style,
    pub lines : TextLines,
    pub start_at : usize,
    pub num_lines : usize
}

impl RTMLParagraph
{
    pub fn new( common : RTMLNodeCommon, style : Style, lines : TextLines ) -> Self
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
    let lines = lines_from_text_width_style( &rtml_paragraph.lines, None );

    Paragraph::new( lines )
    .style( rtml_paragraph.style )
    .wrap( Wrap { trim: false } )
}

pub fn lines_from_text_width_style( lines : &TextLines, style : Option<( usize, Style )> ) -> Vec<Line<'_>>
{
    lines
    .iter()
    .enumerate()
    .map(
        | ( i, l ) |
        {
            if let Some( ( selected, style ) ) = style && selected == i
            {
                line_from_spans( l, Some( style ) )
            }
            else
            {
                line_from_spans( l, None )    
            }
        }
    ).collect::<Vec<_>>()
}

fn line_from_spans<'a>( spans : &'a Vec<( String, Option<Style> )>, next_style : Option<Style> ) -> Line<'a>
{
    let content = spans.iter()
    .map(
        | ( text, style ) |
        {
            span_from_str_and_styles( text, *style, next_style )
        }
    )
    .collect::<Vec<_>>();

    Line::from( content )
}

fn span_from_str_and_styles<'a>( text : &'a str, style_1 : Option<Style>, style_2 : Option<Style> ) -> Span<'a>
{
    match ( style_1, style_2 )
    {
        ( Some( s1 ), Some( s2 ) ) =>
        {
            let s = merge_styles( s1, s2 );

            Span::styled( text, s )
        },
        ( Some( s ), None ) | ( None, Some( s ) ) =>
        {
            Span::styled( text, s )
        },
        ( None, None ) =>
        {
            Span::raw( text )
        }
    }
}