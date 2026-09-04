use std::collections::HashMap;

use ratatui::{buffer::Buffer, layout::{Alignment, Rect}, style::Style, text::{Line, Span}, widgets::{Paragraph, Widget, Wrap}};
use serde_json::Value;

use crate::{input::event::InputEvent, rtml::{rtml_node::{FocusEventResponse, RTMLNodeCommon}, rtml_padding::RTMLPadding, util::{rtml_style::{RTMLStyleTemplate, merge_style_with_templates}, types::TextLines}}, util::draw::clear_area, xml::styles::xml_style::merge_styles};

#[derive(Debug)]
pub struct RTMLParagraph 
{
    pub common : RTMLNodeCommon,
    pub padding : RTMLPadding,
    pub alignment : Alignment,
    pub style : Style,
    pub style_template : RTMLStyleTemplate,
    pub focus_style : Style,
    pub focus_style_template : RTMLStyleTemplate,
    pub lines : TextLines,
    pub start_at : usize,
    pub num_lines : usize,
    pub inner_area : Rect
}

impl RTMLParagraph
{
    pub fn new( 
        common : RTMLNodeCommon, 
        padding : RTMLPadding, 
        alignment : Alignment, 
        style : Style, 
        style_template : RTMLStyleTemplate,
        focus_style : Style, 
        focus_style_template : RTMLStyleTemplate,
        lines : TextLines 
    ) -> Self
    {
        Self { common, padding, alignment, style, style_template, focus_style, focus_style_template, lines, start_at : 0, num_lines : 0, inner_area : Rect::default() }
    }

    pub fn focus_event( &mut self, event : &InputEvent ) -> FocusEventResponse
    {
        match event
        {
            InputEvent::Up =>
            {
                if self.start_at == 0 { return FocusEventResponse::new_without_state( false ) };

                self.start_at -= 1;

                FocusEventResponse::new_without_state( true )
            },
            InputEvent::Down =>
            {
                let max_start_at = self.num_lines.saturating_sub( self.inner_area.height as usize );

                if self.start_at >= max_start_at
                {
                    FocusEventResponse::new_without_state( false )
                }
                else
                {
                    self.start_at += 1;

                    FocusEventResponse::new_without_state( true )    
                }
            },
            _ => FocusEventResponse::new_without_state( false )
        }
    }
}

pub fn render_rtml_paragraph( 
    rtml_paragraph : &RTMLParagraph,
    area : Rect,
    buf : &mut Buffer,
    templates : &HashMap<String, String>,
    context : &Value
) -> anyhow::Result<()>
{
    let mut paragraph = create_paragraph( rtml_paragraph );

    if rtml_paragraph.start_at > 0
    {
        paragraph = paragraph.scroll( ( rtml_paragraph.start_at as u16, 0 ) );
    }

    let style = merge_style_with_templates( rtml_paragraph.style, &rtml_paragraph.style_template, context, templates );

    paragraph = paragraph.style( style );

    render_padding( area, style, buf );

    paragraph.render( rtml_paragraph.inner_area, buf );

    Ok( () )
}

fn render_padding(
    area : Rect,
    style : Style,
    buf : &mut Buffer
)
{
    clear_area( area, style, buf );
}

pub fn render_rtml_paragraph_focus( 
    rtml_paragraph : &RTMLParagraph,
    buf : &mut Buffer,
    templates : &HashMap<String, String>,
    context : &Value
) -> anyhow::Result<()>
{
    let mut paragraph = create_paragraph( rtml_paragraph );

    if rtml_paragraph.start_at > 0
    {
        paragraph = paragraph.scroll( ( rtml_paragraph.start_at as u16, 0 ) );
    }

    let focus_style = merge_style_with_templates( rtml_paragraph.focus_style, &rtml_paragraph.focus_style_template, context, templates );

    paragraph = paragraph.style( focus_style );

    render_padding( rtml_paragraph.common.attrs.area, focus_style, buf );

    paragraph.render( rtml_paragraph.inner_area, buf );

    Ok( () )
}

pub fn create_paragraph<'a>( 
    rtml_paragraph : &'a RTMLParagraph
) -> Paragraph<'a>
{
    let lines = lines_from_text_width_style( &rtml_paragraph.lines, None );

    Paragraph::new( lines )
    .alignment( rtml_paragraph.alignment )
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

pub fn line_from_spans<'a>( spans : &'a Vec<( String, Option<Style> )>, next_style : Option<Style> ) -> Line<'a>
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