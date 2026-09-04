use std::collections::HashMap;

use ratatui::style::{Color, Modifier, Style};
use serde_json::Value;

use crate::util::{log::log_to_file, template::xml_from_template_context};

#[derive(Debug)]
enum RTMLStyleTemplateAttr
{
    Bg( RTMLStyleTemplateType ),
    Fg( RTMLStyleTemplateType ),
    Uc( RTMLStyleTemplateType ),
    FontWeight( RTMLStyleTemplateType ),
    FontStyle( RTMLStyleTemplateType ),
    Dim( RTMLStyleTemplateType ),
    TextDecoration( RTMLStyleTemplateType ),
    Blink( RTMLStyleTemplateType ),
    Invert( RTMLStyleTemplateType ),
    Visibility( RTMLStyleTemplateType )
}

#[derive(Debug)]
pub enum RTMLStyleTemplateType
{
    External( String ),
    Inline( String )
}

#[derive(Debug, Default)]
pub struct RTMLStyleTemplate
{
    attrs : Vec<RTMLStyleTemplateAttr>
}

impl RTMLStyleTemplate
{
    fn attrs( &self ) -> &[RTMLStyleTemplateAttr]
    {
        &self.attrs
    }
}

pub struct RTMLStyleTemplateBuilder
{
    template : RTMLStyleTemplate
}

impl RTMLStyleTemplateBuilder
{
    pub fn new() -> Self
    {
        Self { template : RTMLStyleTemplate::default() }
    }

    pub fn bg( mut self, bg : RTMLStyleTemplateType ) -> Self
    {
        self.template.attrs.push(
            RTMLStyleTemplateAttr::Bg( bg )
        );

        self
    }

    pub fn fg( mut self, fg : RTMLStyleTemplateType ) -> Self
    {
        self.template.attrs.push(
            RTMLStyleTemplateAttr::Fg( fg )
        );

        self
    }

    pub fn uc( mut self, uc : RTMLStyleTemplateType ) -> Self
    {
        self.template.attrs.push(
            RTMLStyleTemplateAttr::Uc( uc )
        );

        self
    }

    pub fn font_weight( mut self, font_weight : RTMLStyleTemplateType ) -> Self
    {
        self.template.attrs.push(
            RTMLStyleTemplateAttr::FontWeight( font_weight )
        );

        self
    }

    pub fn font_style( mut self, font_style : RTMLStyleTemplateType ) -> Self
    {
        self.template.attrs.push(
            RTMLStyleTemplateAttr::FontStyle( font_style )
        );

        self
    }

    pub fn dim( mut self, dim : RTMLStyleTemplateType ) -> Self
    {
        self.template.attrs.push(
            RTMLStyleTemplateAttr::Dim( dim )
        );

        self
    }

    pub fn text_decoration( mut self, text_decoration : RTMLStyleTemplateType ) -> Self
    {
        self.template.attrs.push(
            RTMLStyleTemplateAttr::TextDecoration( text_decoration )
        );

        self
    }

    pub fn blink( mut self, blink : RTMLStyleTemplateType ) -> Self
    {
        self.template.attrs.push(
            RTMLStyleTemplateAttr::Blink( blink )
        );

        self
    }

    pub fn invert( mut self, invert : RTMLStyleTemplateType ) -> Self
    {
        self.template.attrs.push(
            RTMLStyleTemplateAttr::Invert( invert )
        );

        self
    }

    pub fn visibility( mut self, visibility : RTMLStyleTemplateType ) -> Self
    {
        self.template.attrs.push(
            RTMLStyleTemplateAttr::Visibility( visibility )
        );

        self
    }

    pub fn build( self ) -> RTMLStyleTemplate
    {
        self.template
    }
}

pub fn merge_style_with_templates(
    mut style : Style,
    template : &RTMLStyleTemplate,
    context : &Value,
    templates : &HashMap<String, String>
) -> Style
{
    for attr in template.attrs()
    {
        style = merge_attr( style, context, templates, attr );
    }

    style
}

fn merge_attr(
    style : Style,
    context : &Value,
    templates : &HashMap<String, String>,
    attr : &RTMLStyleTemplateAttr
) -> Style
{
    match attr
    {
        RTMLStyleTemplateAttr::Bg( v ) =>
        {
            change_color( style, context, v, templates, | s, c | s.bg( c ) )
        },
        RTMLStyleTemplateAttr::Fg( v ) =>
        {
            change_color( style, context, v, templates, | s, c | s.fg( c ) )
        },
        RTMLStyleTemplateAttr::Uc( v ) =>
        {
            change_color( style, context, v, templates, | s, c | s.underline_color( c ) )
        },
        RTMLStyleTemplateAttr::FontWeight( v ) =>
        {
            change_font_weight( style, context, v, templates )
        },
        RTMLStyleTemplateAttr::FontStyle( v ) =>
        {
            change_font_style( style, context, v, templates )
        },
        RTMLStyleTemplateAttr::Dim( v ) =>
        {
            change_dim( style, context, v, templates )
        },
        RTMLStyleTemplateAttr::TextDecoration( v ) =>
        {
            change_text_decoration( style, context, v, templates )
        },
        RTMLStyleTemplateAttr::Blink( v ) =>
        {
            change_blink( style, context, v, templates )
        },
        
        RTMLStyleTemplateAttr::Invert( v ) =>
        {
            change_invert( style, context, v, templates )
        },
        RTMLStyleTemplateAttr::Visibility( v ) =>
        {
            change_visibility( style, context, v, templates )
        }
    }
}

fn change_visibility(
    style : Style,
    context : &Value,
    template : &RTMLStyleTemplateType,
    templates : &HashMap<String, String>
) -> Style
{
    if let Some( t ) = template_str_from_template( template, templates ) &&
    let Some( s ) = evaluate_template( t, context )
    {
        match s.trim().to_lowercase().as_str()
        {
            "visible" => style.remove_modifier( Modifier::HIDDEN ),
            "hidden" => style.add_modifier( Modifier::HIDDEN ),
            _ => style
        }
    }
    else
    {
        style    
    }
}

fn change_invert(
    style : Style,
    context : &Value,
    template : &RTMLStyleTemplateType,
    templates : &HashMap<String, String>
) -> Style
{
    if let Some( t ) = template_str_from_template( template, templates ) &&
    let Some( s ) = evaluate_template( t, context )
    {
        match s.trim().to_lowercase().as_str()
        {
            "false" => style.remove_modifier( Modifier::REVERSED ),
            "true" => style.add_modifier( Modifier::REVERSED ),
            _ => style
        }
    }
    else
    {
        style    
    }
}

fn change_blink(
    style : Style,
    context : &Value,
    template : &RTMLStyleTemplateType,
    templates : &HashMap<String, String>
) -> Style
{
    if let Some( t ) = template_str_from_template( template, templates ) &&
    let Some( s ) = evaluate_template( t, context )
    {
        match s.trim().to_lowercase().as_str()
        {
            "normal" => style.remove_modifier( Modifier::SLOW_BLINK ).remove_modifier( Modifier::RAPID_BLINK ),
            "slow" => style.remove_modifier( Modifier::RAPID_BLINK ).add_modifier( Modifier::SLOW_BLINK ),
            "rapid" => style.remove_modifier( Modifier::SLOW_BLINK ).add_modifier( Modifier::RAPID_BLINK ),
            _ => style
        }
    }
    else
    {
        style    
    }
}

fn change_text_decoration(
    style : Style,
    context : &Value,
    template : &RTMLStyleTemplateType,
    templates : &HashMap<String, String>
) -> Style
{
    if let Some( t ) = template_str_from_template( template, templates ) &&
    let Some( s ) = evaluate_template( t, context )
    {
        match s.trim().to_lowercase().as_str()
        {
            "none" => style.remove_modifier( Modifier::UNDERLINED ).remove_modifier( Modifier::CROSSED_OUT ),
            "underline" => style.remove_modifier( Modifier::CROSSED_OUT ).add_modifier( Modifier::UNDERLINED ),
            "line-through" => style.remove_modifier( Modifier::UNDERLINED ).add_modifier( Modifier::CROSSED_OUT ),
            _ => style
        }
    }
    else
    {
        style    
    }
}

fn change_dim(
    style : Style,
    context : &Value,
    template : &RTMLStyleTemplateType,
    templates : &HashMap<String, String>
) -> Style
{
    if let Some( t ) = template_str_from_template( template, templates ) &&
    let Some( s ) = evaluate_template( t, context )
    {
        match s.trim().to_lowercase().as_str()
        {
            "false" => style.remove_modifier( Modifier::DIM ),
            "true" => style.add_modifier( Modifier::DIM ),
            _ => style
        }
    }
    else
    {
        style    
    }
}

fn change_font_style(
    style : Style,
    context : &Value,
    template : &RTMLStyleTemplateType,
    templates : &HashMap<String, String>
) -> Style
{
    if let Some( t ) = template_str_from_template( template, templates ) &&
    let Some( s ) = evaluate_template( t, context )
    {
        match s.trim().to_lowercase().as_str()
        {
            "normal" => style.remove_modifier( Modifier::ITALIC ),
            "italic" => style.add_modifier( Modifier::ITALIC ),
            _ => style
        }
    }
    else
    {
        style    
    }
}

fn change_font_weight(
    style : Style,
    context : &Value,
    template : &RTMLStyleTemplateType,
    templates : &HashMap<String, String>
) -> Style
{
    if let Some( t ) = template_str_from_template( template, templates ) &&
    let Some( s ) = evaluate_template( t, context )
    {
        match s.trim().to_lowercase().as_str()
        {
            "normal" => style.remove_modifier( Modifier::BOLD ),
            "bold" => style.add_modifier( Modifier::BOLD ),
            _ => style
        }
    }
    else
    {
        style    
    }
}

fn change_color(
    style : Style,
    context : &Value,
    template : &RTMLStyleTemplateType,
    templates : &HashMap<String, String>,
    fnc : impl FnOnce( Style, Color ) -> Style
) -> Style
{
    if let Some( t ) = template_str_from_template( template, templates ) &&
    let Some( s ) = evaluate_template( t, context )
    {
        if let Ok( c ) = s.trim().parse::<Color>()
        {
            ( fnc )( style, c )
        }
        else
        {
            style    
        }
    }
    else
    {
        style
    }
}

fn evaluate_template( template : &str, context : &Value ) -> Option<String>
{
    match xml_from_template_context( template, context )
    {
        Ok( s ) => Some( s ),
        Err( e ) =>
        {
            log_to_file( &format!( "Se ha producido un error al evaluar el template:\n\n {template}\n\n con el context:\n\n {context:?}.\nErr: {e:?}" ) );

            None
        }
    }
}

fn template_str_from_template<'a>(
    template : &'a RTMLStyleTemplateType,
    templates : &'a HashMap<String, String>
) -> Option<&'a str>
{
    match template
    {
        RTMLStyleTemplateType::External( s ) =>
        {
            if let Some( t ) = templates.get( s )
            {
                Some( t )
            }
            else
            {
                None    
            }
        },
        RTMLStyleTemplateType::Inline( s ) => Some( s )
    }
}