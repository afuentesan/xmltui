use std::time::Duration;

use ratatui::{buffer::Buffer, style::Style};

use crate::{input::event::InputEvent, rtml::{rtml_command::RTMLCommandOutput, rtml_doc::RTMLDoc, rtml_node::{FocusEventResponse, RTMLNode, RTMLNodeCommon}, util::{rtml_style::RTMLStyleTemplateType, types::{TextLine, TextLines}}}};

#[derive(Debug, Clone)]
pub struct ToastParams
{
    pub level : ToastLevel,
    pub duration : Option<Duration>,
    pub title : Option<String>,
    pub title_template : Option<RTMLStyleTemplateType>,
    pub body : Option<String>,
    pub body_template : Option<RTMLStyleTemplateType>,
    pub output : RTMLCommandOutput
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
        output : RTMLCommandOutput
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
    pub style : Style,
    pub focus_style : Style
}

impl RTMLToast
{
    pub fn new( 
        common : RTMLNodeCommon,
        title : Option<TextLine>, 
        body : TextLines, 
        style : Style, 
        focus_style : Style
    ) -> Self
    {
        Self { common, title, body, style, focus_style }
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
    pub success : ( Style, Style ),
    pub error : ( Style, Style )
}

impl ToastStyles
{
    pub fn new( success : Style, success_focus : Style, err : Style, err_focus : Style ) -> Self
    {
        Self { success : ( success, success_focus ), error : ( err, err_focus ) }
    }
}

pub fn render_toasts(
    doc : &RTMLDoc,
    buf : &mut Buffer
)
{
    let current_focus_id = doc.current_focus_id().unwrap_or( "" );

    if let Some( n ) = doc.doc.get( &doc.root_id )
    {
        let area = n.area();

        let mut next_y = 1;
        let x = area.width - 26;
        let with = 25;

        for child in n.childs()
        {
            next_y = render_toast_id( 
                doc, 
                buf, 
                child,
                current_focus_id,
                with,
                next_y,
                x
            );
        }
    }
}

fn render_toast_id(
    doc : &RTMLDoc,
    buf : &mut Buffer,
    id : &str,
    focus_id : &str,
    width : u16,
    next_y : u16,
    x : u16
) -> u16
{
    if let Some( RTMLNode::Toast( t ) ) = doc.doc.get( id )
    {
        render_toast(
            t, 
            buf, 
            id, 
            focus_id, 
            width, 
            next_y, 
            x
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
    id : &str,
    focus_id : &str,
    width : u16,
    next_y : u16,
    x : u16
) -> u16
{
    let style = if id == focus_id
    {
        toast.focus_style
    }
    else
    {
        toast.style    
    };
    
    todo!()
}