use std::{collections::HashMap, thread, time::Duration};

use ratatui::style::Style;
use serde_json::Value;

use crate::{app::event::{AppEvent, ShowMessage, send_app_event}, rtml::{rtml_command::RTMLCommandOutput, rtml_doc::RTMLDoc, rtml_node::{RTMLNode, RTMLNodeCommon}, rtml_toast::{RTMLToast, ToastLevel, ToastParams}, util::{rtml_attrs::CommonAttrs, rtml_style::{RTMLStyleTemplateType, template_str_from_template}, types::{TextLine, TextLines}}}, util::{log::log_to_file, template::template_to_xml}, xml::{attrs::{default_id, str_id_retry_if_exists}, styles::xml_style::{StyleSelector, XMLStyle}, xml_line::process_text_line, xml_paragraph::process_text_lines}};


pub fn show_message( doc : &mut RTMLDoc, message : ShowMessage ) -> bool
{
    if doc.doc_id != message.doc_id { return false };

    let ( toast, duration ) = toast_from_params( doc, message.toast, message.response );

    if toast.body.is_empty() { return false };

    let id = str_id_retry_if_exists( default_id(), &doc.doc );

    if append_toast_to_root( doc, id.clone(), toast )
    {
        send_close_toast( id, duration );

        true
    }
    else
    {
        false    
    }
}

fn append_toast_to_root( doc : &mut RTMLDoc, id : String, toast : RTMLToast ) -> bool
{
    if let Some( root ) = doc.doc.get_mut( doc.root_id.as_str() )
    {
        root.childs_mut().push( id.clone() );

        doc.doc.insert( id, RTMLNode::Toast( toast ) );

        true
    }
    else
    {
        false
    }
}

fn toast_from_params( doc : &RTMLDoc, params : ToastParams, response : String ) -> ( RTMLToast, Option<Duration> )
{
    let title = title_from_params(
        params.title, 
        params.title_template, 
        &params.output,
        &response,
        &doc.styles,
        &doc.templates, 
        &doc.state
    );
    
    let body = body_from_params( 
        params.body, 
        params.body_template, 
        &params.output,
        &response,
        &doc.styles,
        &doc.templates, 
        &doc.state
    );

    let common = common_from_doc( doc );

    let ( style, focus_style ) = style_from_level( doc, params.level );

    (
        RTMLToast::new( common, title, body, style, focus_style ),
        params.duration
    )
}

fn common_from_doc(
    doc : &RTMLDoc
) -> RTMLNodeCommon
{
    RTMLNodeCommon::new(
        CommonAttrs::default(), 
        vec![], 
        Some( doc.root_id.clone() )
    )
}

fn style_from_level(
    doc : &RTMLDoc,
    level : ToastLevel
) -> ( Style, Style )
{
    match level
    {
        ToastLevel::Error =>
        {
            ( doc.toast_styles.error.0, doc.toast_styles.error.1 )
        },
        ToastLevel::Success =>
        {
            ( doc.toast_styles.success.0, doc.toast_styles.success.1 )
        }
    }
}

fn body_from_params( 
    body : Option<String>, 
    body_template : Option<RTMLStyleTemplateType>,
    output : &RTMLCommandOutput,
    response : &str,
    styles : &HashMap<StyleSelector, XMLStyle>,
    templates : &HashMap<String, String>,
    state : &Value
) -> TextLines
{
    let body = text_from_template( body, body_template, output, response, templates, state );

    if body.trim() == "" { return vec![] };

    text_lines_from_str( &body, styles )
}

fn text_lines_from_str( text : &str, styles : &HashMap<StyleSelector, XMLStyle> ) -> TextLines
{
    let xml = format!( "<container>{text}</container>" );

    match roxmltree::Document::parse(xml.as_str() )
    {
        Ok( doc ) =>
        {
            match process_text_lines( doc.root_element(), styles )
            {
                Ok( lines ) => lines,
                Err( e ) =>
                {
                    log_to_file( &format!( "Fail to process text lines. Err: {e:?}" ) );

                    default_text_lines_from_str( text )
                }
            }
        },
        Err( e ) =>
        {
            log_to_file( &format!( "Fail to parse xml. Err: {e:?}" ) );

            default_text_lines_from_str( text )
        }
    }
}

fn default_text_lines_from_str( str : &str ) -> TextLines
{
    let mut ret = vec![];

    for s in str.split( "\n" )
    {
        if s.trim() == "" { continue; }

        ret.push( vec![ ( s.trim().to_string(), None, None ) ] );
    }

    ret
}

fn title_from_params(
    title : Option<String>,
    title_template : Option<RTMLStyleTemplateType>,
    output : &RTMLCommandOutput,
    response : &str,
    styles : &HashMap<StyleSelector, XMLStyle>,
    templates : &HashMap<String, String>,
    state : &Value
) -> Option<TextLine>
{
    let title = text_from_template( title, title_template, output, response, templates, state );

    if title.trim() == "" { return None };

    Some( text_line_from_str( &title, styles ) )
}

fn text_line_from_str( text : &str, styles : &HashMap<StyleSelector, XMLStyle> ) -> TextLine
{
    let xml = format!( "<container>{text}</container>" );

    match roxmltree::Document::parse(xml.as_str() )
    {
        Ok( doc ) =>
        {
            process_text_line( doc.root_element(), styles )
        },
        Err( e ) =>
        {
            log_to_file( &format!( "Fail to parse xml. Err: {e:?}" ) );

            vec![ ( text.trim().to_string(), None, None ) ]
        }
    }
}

fn text_from_template(
    default : Option<String>,
    title_template : Option<RTMLStyleTemplateType>,
    output : &RTMLCommandOutput,
    response : &str,
    templates : &HashMap<String, String>,
    state : &Value
) -> String
{
    if let Some( t ) = title_template
    {
        match template_str_from_template( &t, templates )
        {
            Some( t ) =>
            {
                match template_to_xml( 
                    response.to_string(), 
                    Some( t ), 
                    *output, 
                    state 
                )
                {
                    Ok( r ) => r,
                    Err( e ) =>
                    {
                        log_to_file( &format!( "Fail evaluate template. Err: {e:?}" ) );

                        default.unwrap_or( "".to_string() )
                    }
                }
            },
            None =>
            {
                default.unwrap_or( "".to_string() )
            }
        }
        
    }
    else
    {
        default.unwrap_or( "".to_string() )    
    }
}

fn send_close_toast( id : String, duration : Option<Duration> )
{
    if let Some( d ) = duration
    {
        spawn_close_toast( id, d );
    }
}

fn spawn_close_toast( id : String, duration : Duration )
{
    thread::spawn(
        move ||
        {
            thread::sleep( duration );

            send_app_event( AppEvent::CloseMessage( id ) );
        }
    );
}