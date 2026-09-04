use std::collections::HashMap;

use ratatui::style::{Color, Style};
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
        RTMLStyleTemplateAttr::FontWeight( v ) => todo!(),
        RTMLStyleTemplateAttr::FontStyle( v ) => todo!(),
        RTMLStyleTemplateAttr::Dim( v ) => todo!(),
        RTMLStyleTemplateAttr::TextDecoration( v ) => todo!(),
        RTMLStyleTemplateAttr::Blink( v ) => todo!(),
        RTMLStyleTemplateAttr::Invert( v ) => todo!(),
        RTMLStyleTemplateAttr::Visibility( v ) => todo!()
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