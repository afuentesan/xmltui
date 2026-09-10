use std::{collections::HashMap, str::FromStr};

use convert_case::ccase;
use ratatui::layout::{Constraint, Direction, Flex, Rect};
use serde_json::Value;

use crate::{rtml::util::{rtml_padding::{HorizontalPadding, RTMLPadding, VerticalPadding}, rtml_style::{RTMLStyleTemplateType, evaluate_template, template_str_from_template}}, xml::{attrs::str_to_uint, styles::xml_padding::padding_from_str}};


#[derive(Debug)]
pub struct CommonAttrs
{
    pub area : Rect,
    pub constraint : Constraint,
    pub constraint_template : ConstraintTemplate
}

#[derive(Debug)]
pub enum ConstraintTemplate
{
    Min( RTMLStyleTemplateType ),
    Max( RTMLStyleTemplateType ),
    Length( RTMLStyleTemplateType ),
    Percentage( RTMLStyleTemplateType ),
    Ratio( RTMLStyleTemplateType ),
    Fill( RTMLStyleTemplateType ),
    None
}

pub fn constraint_from_template( 
    constraint : &Constraint, 
    template : &ConstraintTemplate, 
    templates : &HashMap<String, String>, 
    context : &Value 
) -> Constraint
{
    match template
    {
        ConstraintTemplate::Min( t ) =>
        {
            template_to_uint_constraint( t, templates, context, constraint, | n | Constraint::Min( n ) )
        },
        ConstraintTemplate::Max( t ) =>
        {
            template_to_uint_constraint( t, templates, context, constraint, | n | Constraint::Max( n ) )
        },
        ConstraintTemplate::Length( t ) =>
        {
            template_to_uint_constraint( t, templates, context, constraint, | n | Constraint::Length( n ) )
        },
        ConstraintTemplate::Percentage( t ) =>
        {
            template_to_uint_constraint( t, templates, context, constraint, | n | Constraint::Percentage( n ) )
        },
        ConstraintTemplate::Ratio( t ) =>
        {
            template_to_ratio_constraint( t, templates, context, constraint )
        },
        ConstraintTemplate::Fill( t ) =>
        {
            template_to_uint_constraint( t, templates, context, constraint, | n | Constraint::Fill( n ) )
        },
        ConstraintTemplate::None => *constraint
    }
}

fn template_to_ratio_constraint(
    template : &RTMLStyleTemplateType, 
    templates : &HashMap<String, String>, 
    context : &Value,
    constraint : &Constraint
) -> Constraint
{
    if let Some( t ) = template_str_from_template( template, templates ) &&
    let Some( s ) = evaluate_template( t, context )
    {
        let parts: Vec<&str> = s.split(',').collect();

        {
            (
                ||
                if parts.len() == 2 
                {
                    let num = parts[ 0 ].trim().parse::<u32>().ok()?;
                    let den = parts[ 1 ].trim().parse::<u32>().ok()?;

                    Some( Constraint::Ratio( num, den ) )
                }
                else
                {
                    None    
                }
            )()
        }
        .unwrap_or( *constraint )
    }
    else
    {
        *constraint    
    }
}

fn template_to_uint_constraint( 
    template : &RTMLStyleTemplateType, 
    templates : &HashMap<String, String>, 
    context : &Value,
    constraint : &Constraint,
    fnc : impl FnOnce( u16 ) -> Constraint
) -> Constraint
{
    if let Some( t ) = template_str_from_template( template, templates ) &&
    let Some( s ) = evaluate_template( t, context ) &&
    let Ok( d ) = str_to_uint::<u16>( s.as_str() )
    {
        fnc( d )
    }
    else
    {
        *constraint    
    }
}

#[derive(Debug)]
pub struct ContainerAttrs
{
    pub direction : Direction,
    pub flex : Flex,
    pub padding : RTMLPadding,
    pub template : ContainerTemplate
}

impl ContainerAttrs
{
    pub fn new( direction : Direction, flex : Flex, padding : RTMLPadding, template : ContainerTemplate ) -> Self
    {
        Self { direction, flex, padding, template }
    }
}

pub fn merge_container_attrs_with_template( 
    attrs : &ContainerAttrs,
    context : &Value,
    templates : &HashMap<String, String>
) -> ContainerAttrs
{
    let mut merged = ContainerAttrs::new( attrs.direction, attrs.flex, attrs.padding.clone(), ContainerTemplate::default() );

    for attr in &attrs.template.attrs
    {
        merged = merge_container_attr( attr, merged, context, templates );
    }

    merged
}

fn merge_container_attr(
    attr : &ContainerTemplateAttr,
    attrs : ContainerAttrs,
    context : &Value,
    templates : &HashMap<String, String>
) -> ContainerAttrs
{
    match attr
    {
        ContainerTemplateAttr::Direction( d ) =>
        {
            merge_direction( d, attrs, context, templates )
        },
        ContainerTemplateAttr::Flex( f ) =>
        {
            merge_flex( f, attrs, context, templates )
        },
        ContainerTemplateAttr::Padding( p ) =>
        {
            merge_padding( p, attrs, context, templates )
        },
        ContainerTemplateAttr::PaddingTop( p ) =>
        {
            merge_single_padding( 
                p, 
                attrs, 
                context, 
                templates, 
                | mut c, n |
                {
                    c.padding.vertical.top = n;

                    c
                }
            )
        },
        ContainerTemplateAttr::PaddingRight( p ) =>
        {
            merge_single_padding( 
                p, 
                attrs, 
                context, 
                templates, 
                | mut c, n |
                {
                    c.padding.horizontal.right = n;

                    c
                }
            )
        },
        ContainerTemplateAttr::PaddingBottom( p ) =>
        {
            merge_single_padding( 
                p, 
                attrs, 
                context, 
                templates, 
                | mut c, n |
                {
                    c.padding.vertical.bottom = n;

                    c
                }
            )
        },
        ContainerTemplateAttr::PaddingLeft( p ) =>
        {
            merge_single_padding( 
                p, 
                attrs, 
                context, 
                templates, 
                | mut c, n |
                {
                    c.padding.horizontal.left = n;

                    c
                }
            )
        }
    }
}

fn merge_single_padding(
    template : &RTMLStyleTemplateType,
    mut attrs : ContainerAttrs,
    context : &Value,
    templates : &HashMap<String, String>,
    fnc : impl FnOnce( ContainerAttrs, usize ) -> ContainerAttrs
) -> ContainerAttrs
{
    if let Some( t ) = template_str_from_template( template, templates ) &&
    let Some( s ) = evaluate_template( t, context ) &&
    let Ok( n ) = s.parse::<usize>()
    {
        attrs = fnc( attrs, n );
    }

    attrs
}

fn merge_padding(
    template : &RTMLStyleTemplateType,
    mut attrs : ContainerAttrs,
    context : &Value,
    templates : &HashMap<String, String>
) -> ContainerAttrs
{
    if let Some( t ) = template_str_from_template( template, templates ) &&
    let Some( s ) = evaluate_template( t, context ) &&
    let Some( ( t, r, b, l ) ) = padding_from_str( s.as_str() )
    {
        let padding = RTMLPadding::new( 
            HorizontalPadding::new( l, r ), 
            VerticalPadding::new( t, b )
        );

        attrs.padding = padding;
    }
    
    attrs
}

fn merge_flex( 
    template : &RTMLStyleTemplateType,
    mut attrs : ContainerAttrs,
    context : &Value,
    templates : &HashMap<String, String>
) -> ContainerAttrs
{
    if let Some( t ) = template_str_from_template( template, templates ) &&
    let Some( s ) = evaluate_template( t, context ) &&
    let Some( f ) = str_to_type_kebab( s.as_str() )
    {
        attrs.flex = f;
    }
    
    attrs
}

fn merge_direction( 
    template : &RTMLStyleTemplateType,
    mut attrs : ContainerAttrs,
    context : &Value,
    templates : &HashMap<String, String>
) -> ContainerAttrs
{
    if let Some( t ) = template_str_from_template( template, templates ) &&
    let Some( s ) = evaluate_template( t, context ) &&
    let Some( d ) = str_to_type_kebab( s.as_str() )
    {
        attrs.direction = d;
    }
    
    attrs
}

fn str_to_type_kebab<T: FromStr>( str : &str ) -> Option<T>
{
    match ccase!( pascal, str.trim() ).parse::<T>()
    {
        Ok( n ) => Some( n ),
        Err( _ ) => None
    }
}

#[derive(Debug)]
pub enum ContainerTemplateAttr
{
    Direction( RTMLStyleTemplateType ),
    Flex( RTMLStyleTemplateType ),
    Padding( RTMLStyleTemplateType ),
    PaddingTop( RTMLStyleTemplateType ),
    PaddingRight( RTMLStyleTemplateType ),
    PaddingBottom( RTMLStyleTemplateType ),
    PaddingLeft( RTMLStyleTemplateType )
}

#[derive(Debug, Default)]
pub struct ContainerTemplate
{
    attrs : Vec<ContainerTemplateAttr>
}

pub struct ContainerTemplateBuilder
{
    template : ContainerTemplate
}

impl ContainerTemplateBuilder
{
    pub fn new() -> Self
    {
        Self { template : ContainerTemplate::default() }
    }

    pub fn direction( mut self, dir : RTMLStyleTemplateType ) -> Self
    {
        self.template.attrs.push(
            ContainerTemplateAttr::Direction( dir )
        );

        self
    }

    pub fn flex( mut self, flex : RTMLStyleTemplateType ) -> Self
    {
        self.template.attrs.push(
            ContainerTemplateAttr::Flex( flex )
        );

        self
    }

    pub fn padding( mut self, padding : RTMLStyleTemplateType ) -> Self
    {
        self.template.attrs.push(
            ContainerTemplateAttr::Padding( padding )
        );

        self
    }

    pub fn padding_top( mut self, padding_top : RTMLStyleTemplateType ) -> Self
    {
        self.template.attrs.push(
            ContainerTemplateAttr::PaddingTop( padding_top )
        );

        self
    }

    pub fn padding_right( mut self, padding_right : RTMLStyleTemplateType ) -> Self
    {
        self.template.attrs.push(
            ContainerTemplateAttr::PaddingRight( padding_right )
        );

        self
    }

    pub fn padding_bottom( mut self, padding_bottom : RTMLStyleTemplateType ) -> Self
    {
        self.template.attrs.push(
            ContainerTemplateAttr::PaddingBottom( padding_bottom )
        );

        self
    }

    pub fn padding_left( mut self, padding_left : RTMLStyleTemplateType ) -> Self
    {
        self.template.attrs.push(
            ContainerTemplateAttr::PaddingLeft( padding_left )
        );

        self
    }

    pub fn build( self ) -> ContainerTemplate
    {
        self.template
    }
}