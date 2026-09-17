use std::{collections::HashMap, time::Duration};

use ratatui::{buffer::Buffer, layout::{Alignment, Rect}, style::{Modifier, Style}, widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap}};
use serde_json::Value;

use crate::{input::event::InputEvent, rtml::{rtml_doc::RTMLDoc, rtml_line::render_text_line, rtml_node::{FocusEventResponse, RTMLNode, RTMLNodeCommon}, rtml_paragraph::lines_from_text_with_style, util::{rtml_padding::HorizontalPadding, rtml_style::{RTMLStyleTemplateBuilder, RTMLStyleTemplateType}, types::{TextLine, TextLines}}}, state::state_executor::TypeState};

#[derive(Debug, Clone)]
pub struct ToastParams
{
    pub level : ToastLevel,
    pub duration : Option<Duration>,
    pub title : Option<String>,
    pub title_template : Option<RTMLStyleTemplateType>,
    pub body : Option<String>,
    pub body_template : Option<RTMLStyleTemplateType>,
    pub output : TypeState
}

impl ToastParams
{
    pub fn new(
        level : ToastLevel,
        duration : Option<Duration>,
        title : Option<String>,
        title_template : Option<RTMLStyleTemplateType>,
        body : Option<String>,
        body_template : Option<RTMLStyleTemplateType>,
        output : TypeState
    ) -> Self
    {
        Self { level, duration, title, title_template, body, body_template, output }
    }
}


#[derive(Debug, Clone)]
pub enum ToastLevel
{
    Error,
    Success
}

#[derive(Debug)]
pub struct RTMLToast
{
    pub common : RTMLNodeCommon,
    pub title : Option<TextLine>,
    pub body : TextLines,
    pub style : Style
}

impl RTMLToast
{
    pub fn new( 
        common : RTMLNodeCommon,
        title : Option<TextLine>, 
        body : TextLines, 
        style : Style
    ) -> Self
    {
        Self { common, title, body, style }
    }

    pub fn focus_event( &mut self, event : &InputEvent ) -> FocusEventResponse
    {
        match event
        {
            _ => FocusEventResponse::new_without_state( false )
        }
    }
}

#[derive(Debug, Default)]
pub struct ToastStyles
{
    pub success : Style,
    pub error : Style
}

impl ToastStyles
{
    pub fn new( success : Style, error : Style ) -> Self
    {
        Self { success, error }
    }
}

pub fn close_first_toast(
    doc : &mut RTMLDoc
) -> bool
{
    if let Some( n ) = doc.doc.get( &doc.root_id )
    {
        let mut id = None;

        for child in n.childs()
        {
            if let Some( RTMLNode::Toast( _ ) ) = doc.doc.get( child )
            {
                id = Some( child.to_string() );

                break;
            }
        }

        if let Some( id ) = id
        {
            doc.remove_node_and_clear_from_parent( &id, &doc.root_id.clone() );

            true
        }
        else
        {
            false    
        }
    }
    else
    {
        false    
    }
}

pub fn render_toasts(
    doc : &RTMLDoc,
    buf : &mut Buffer,
    context : &Value
)
{
    if let Some( n ) = doc.doc.get( &doc.root_id )
    {
        let area = n.area();

        let width = if ( area.width / 2 ) < 30
        {
            area.width / 2
        }
        else
        {
            30    
        };

        let mut next_y = 1;
        let x = area.width - width - 1;
        let last_row = area.height - 1;

        for child in n.childs()
        {
            next_y = render_toast_id( 
                doc, 
                buf, 
                context,
                child,
                width,
                next_y,
                x,
                last_row
            );
        }
    }
}

fn render_toast_id(
    doc : &RTMLDoc,
    buf : &mut Buffer,
    context : &Value,
    id : &str,
    width : u16,
    next_y : u16,
    x : u16,
    last_row : u16
) -> u16
{
    if let Some( RTMLNode::Toast( t ) ) = doc.doc.get( id )
    {
        render_toast(
            t, 
            buf, 
            context,
            &doc.templates,
            width, 
            next_y, 
            x,
            last_row
        )
    }
    else
    {
        next_y    
    }
}

fn render_toast(
    toast : &RTMLToast,
    buf : &mut Buffer,
    context : &Value,
    templates : &HashMap<String, String>,
    width : u16,
    next_y : u16,
    x : u16,
    last_row : u16
) -> u16
{
    let lines = lines_from_text_with_style( &toast.body, None, templates, context );

    let paragraph = Paragraph::new( lines )
    .alignment( Alignment::Left )
    .wrap( Wrap { trim: false } )
    .style( toast.style );

    let mut num_lines = paragraph.line_count( width - 2 );

    let title = if let Some( title ) = toast.title.as_ref()
    {
        num_lines += 2;

        title
    }
    else
    {
        &vec![]    
    };

    num_lines += 2;

    if num_lines >= last_row as usize
    {
        num_lines = last_row as usize - 1;
    }

    let new_next_y = next_y + num_lines as u16;

    let ( current_y, next_y ) = if new_next_y <= last_row
    {
        ( next_y, new_next_y )
    }
    else if ( 1 + num_lines as u16 ) <= last_row
    {
        ( 1, 1 + num_lines as u16 )
    }
    else
    {
        ( 1, 1 )    
    };

    let block_area = Rect::new( x, current_y, width, num_lines as u16 );

    Clear.render( block_area, buf );

    let block = Block::default().borders( Borders::ALL ).style( toast.style );

    block.render( block_area, buf );

    let paragraph_area = if ! title.is_empty()
    {
        let title_area = Rect::new( x + 1, current_y +  1, width - 2, 1 );

        let style_title = toast.style.add_modifier( Modifier::UNDERLINED );

        let _ = render_text_line(
            style_title, 
            &RTMLStyleTemplateBuilder::new().build(), 
            &HorizontalPadding::new( 0, 0 ), 
            &Alignment::Left, 
            title, 
            title_area, 
            buf, 
            templates, 
            context
        );

        Rect::new( x + 1, current_y + 3, width - 2, num_lines as u16 - 4 )
    }
    else
    {
        Rect::new( x + 1, current_y + 1, width - 2, num_lines as u16 - 2 )
    };
    
    paragraph.render( paragraph_area, buf );

    next_y
}