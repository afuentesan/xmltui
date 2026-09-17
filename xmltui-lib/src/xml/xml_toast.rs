use std::{collections::HashMap, time::Duration};

use ratatui::style::Style;
use roxmltree::Node;

use crate::{rtml::{rtml_toast::{ToastLevel, ToastParams, ToastStyles}, util::rtml_style::RTMLStyleTemplateType}, util::log::log_to_file, xml::{attrs::attr_to_template, styles::xml_style::{StyleSelector, XMLStyle}, xml_state::type_from_node, xml_util::style_from_styles}};

pub fn toast_params_from_node( level : ToastLevel, prefix : &str, node : Node ) -> Option<ToastParams>
{
    let ( title, title_template ) = toast_text( node, &format!( "{prefix}-title" ) );
    let ( body, body_template ) = toast_text( node, &format!( "{prefix}-message" ) );

    if body.is_none() && body_template.is_none()
    {
        return None
    }

    let duration = toast_duration( node, &format!( "{prefix}-dur" ) );

    let output = type_from_node( node, format!( "{prefix}-type" ).as_str() );

    Some(
        ToastParams::new(
            level, 
            duration, 
            title, 
            title_template, 
            body, 
            body_template,
            output
        )
    )
}

fn toast_text( node : Node, attr : &str ) -> ( Option<String>, Option<RTMLStyleTemplateType> )
{
    match attr_to_template( node, attr )
    {
        Some( t ) => ( None, Some( t ) ),
        None =>
        {
            if let Some( t ) = node.attribute( attr ) && t.trim() != ""
            {
                ( Some( t.trim().to_string() ), None )
            }
            else
            {
                ( None, None )    
            }
        }
    }
}

fn toast_duration( node : Node, attr : &str ) -> Option<Duration>
{
    match node.attribute( attr )
    {
        Some( val ) if val.trim() != "" =>
        {
            match val.trim().parse::<u64>()
            {
                Ok( n ) => Some( Duration::from_secs( n ) ),
                _ => None
            }
        },
        _ =>
        {
            Some( Duration::from_secs( 5 ) )
        } 
    }
}

pub fn toast_styles(
    styles : &HashMap<StyleSelector, XMLStyle>
) -> ToastStyles
{
    let style_success = match roxmltree::Document::parse( "<toast-success></toast-success>" )
    {
        Ok( doc ) =>
        {
            let ( style, _ ) = style_from_styles( doc.root_element(), styles, None, None );

            style
        },
        Err( e ) =>
        {
            log_to_file( &format!( "toast_styles. Error create doc. Err: {e:?}" ) );

            Style::default()
        }
    };

    let style_err = match roxmltree::Document::parse( "<toast-err></toast-err>" )
    {
        Ok( doc ) =>
        {
            let ( style, _ ) = style_from_styles( doc.root_element(), styles, None, None );

            style
        },
        Err( e ) =>
        {
            log_to_file( &format!( "toast_styles. Error create doc. Err: {e:?}" ) );

            Style::default()
        }
    };

    ToastStyles::new( style_success, style_err )
}