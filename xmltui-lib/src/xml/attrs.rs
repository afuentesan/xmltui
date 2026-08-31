use std::{collections::HashMap, str::FromStr};

use convert_case::ccase;
use ratatui::layout::{Alignment, Constraint, Rect};
use regex::regex;
use roxmltree::Node;
use uuid::Uuid;

use crate::rtml::{rtml_attrs::CommonAttrs, rtml_node::{RTMLNode, RTMLNodeId}, rtml_source::RTMLSource};

const DEFAULT_ALIGNMENT : Alignment = Alignment::Left;


pub fn attr_id( node : Node ) -> Option<RTMLNodeId>
{
    if let Some( id ) = node.attribute( "id" )
    {
        Some( id.to_string() )
    }
    else
    {
        None
    }
}

pub fn default_id() -> RTMLNodeId
{
    Uuid::new_v4().to_string()
}

pub fn id_retry_if_exists( node : Node, nodos : &HashMap<String, RTMLNode> ) -> RTMLNodeId
{
    let mut id = attr_id( node ).unwrap_or( default_id() );

    if ! nodos.contains_key( &id ) { return id };

    for _ in 0..10
    {
        id = default_id();

        if ! nodos.contains_key( &id ) { return id };
    }

    default_id()
}

pub fn parse_common_attrs( node : Node, constraint : Constraint ) -> anyhow::Result<CommonAttrs>
{
    Ok(
        CommonAttrs
        {
            area : Rect::ZERO,
            constraint,
            data : attr_data( node )
        }
    )
}

fn attr_data( node : Node ) -> HashMap<String, String>
{
    node.attributes()
    .fold(
        HashMap::new(),
        | mut acc, attr |
        {
            let name = attr.name().trim();

            if name.starts_with( "data-" ) && 
               name != "data-from-node" &&
               name.len() > 5
            {
                let val = attr.value();

                let name = &name[5..];

                acc.insert( name.to_string(), val.to_string() );
            }
            
            acc
        }
    )
}

// fn attr_constraint( node : Node ) -> anyhow::Result<Constraint>
// {
//     let attrs = [ "fill", "percentage", "min", "max", "length", "ratio" ];

//     for attr in attrs
//     {
//         match node.attribute( attr )
//         {
//             Some( val ) =>
//             {
//                 return parse_attr_constraint( attr, val )
//             },
//             None => continue
//         }
//     }

//     let default = match node.tag_name().name()
//     {
//         "line" | "span" | "button" | "a" =>
//         {
//             match horizontal_text_length_from_node( node )
//             {
//                 Some( l ) => Constraint::Length( l as u16 ),
//                 None => DEFAULT_CONSTRAINT    
//             }
//         },
//         _ => DEFAULT_CONSTRAINT
//     };

//     Ok( default )
// }

pub fn attr_constraint( node : Node ) -> Option<Constraint>
{
    let attrs = [ "fill", "percentage", "min", "max", "length", "ratio" ];

    for attr in attrs
    {
        match node.attribute( attr )
        {
            Some( val ) =>
            {
                match parse_attr_constraint( attr, val )
                {
                    Some( c ) => return Some( c ),
                    None => continue
                }
            },
            None => continue
        }
    }

    None
}

fn parse_attr_constraint( attr : &str, val : &str ) -> Option<Constraint>
{
    match attr
    {
        "fill" =>
        {
            Some( Constraint::Fill( str_to_uint::<u16>( val ).ok()? ) )
        },
        "percentage" =>
        {
            Some( Constraint::Percentage( str_to_uint::<u16>( val ).ok()? ) )
        },
        "min" =>
        {
            Some( Constraint::Min( str_to_uint::<u16>( val ).ok()? ) )
        },
        "max" =>
        {
            Some( Constraint::Max( str_to_uint::<u16>( val ).ok()? ) )
        },
        "length" =>
        {
            Some( Constraint::Length( str_to_uint::<u16>( val ).ok()? ) )
        },
        "ratio" =>
        {
            let numbers = pair_str_to_pair_of_uints::<u32>( val ).ok()?;

            Some( Constraint::Ratio( numbers.0, numbers.1 ) )
        },
        _ => unreachable!()
    }
}

fn pair_str_to_pair_of_uints<T: FromStr>( str : &str ) -> anyhow::Result<( T, T )>
{
    let mut num1 : Option<T> = None;
    let mut num2 : Option<T> = None;

    for ( i, s ) in str.split( "," ).enumerate()
    {
        match i
        {
            0 =>
            {
                num1 = Some( str_to_uint( s )? )
            },
            1 =>
            {
                num2 = Some( str_to_uint( s )? )
            },
            _ => return Err( anyhow::Error::msg( format!( "Espected two uints separated by commas. Value: {}", str ) ) ) 
        }
    }

    Ok(
        (
            num1.ok_or( 
                anyhow::Error::msg( format!( "Espected two uints separated by commas. Value: {}", str ) )  
            )?,
            num2.ok_or( 
                anyhow::Error::msg( format!( "Espected two uints separated by commas. Value: {}", str ) )
            )?
        )
    )
}

fn str_to_uint<T: FromStr>( str : &str ) -> anyhow::Result<T>
{
    let str = str.trim();

    let re = regex!( r"^[0-9]+" );

    if ! re.is_match( str )
    {
        return Err( anyhow::Error::msg( format!( "Not valid uint. Value: {}", str ) ) )
    }

    match str.parse::<T>()
    {
        Ok( val ) => Ok( val ),
        Err( _ ) =>
        {
            Err( anyhow::Error::msg( format!( "Not valid uint. Value: {}", str ) ) )
        }
    }
}

pub fn attr_alignment_name( node : Node, attr : &str ) -> anyhow::Result<Alignment>
{
    match node.attribute( attr )
    {
        Some( d ) =>
        {
            parse_attr_align( d )
        },
        None => Ok( DEFAULT_ALIGNMENT )
    }
}

fn parse_attr_align( align : &str ) -> anyhow::Result<Alignment>
{
    match align.trim().to_lowercase().as_str()
    {
        "" => Ok( DEFAULT_ALIGNMENT ),
        "left" => Ok( Alignment::Left ),
        "right" => Ok( Alignment::Right ),
        "center" => Ok( Alignment::Center ),
        e => Err( anyhow::Error::msg(
            format!( "{} is not a valid align", e )
        ) )
    }
}

pub fn attr_value( node : Node ) -> String
{
    node.attribute( "value" ).unwrap_or( "" ).to_string()
}

pub fn attr_result( node : Node, attr : &str ) -> anyhow::Result<String>
{
    Ok(
        node.attribute( attr ).ok_or(
            anyhow::Error::msg( "src expected" )
        )?.to_string()
    )
}

pub fn attr_commands( node : Node, attr : &str ) -> anyhow::Result<Vec<String>>
{
    Ok(
        attr_comands_from_str(
            node.attribute( attr ).ok_or(
                anyhow::Error::msg( format!( "{} expected", attr ) )
            )?
        )
    )
}

pub fn attr_comands_from_str( str : &str ) -> Vec<String>
{
    str
    .split( "|" )
    .filter( | s | s.trim() != "" )
    .map(
        | s | s.to_string()
    )
    .collect()
}

pub fn attr_option( node : Node, attr : &str ) -> Option<String>
{
    Some(
        node.attribute( attr )?.to_string()
    )
}

pub fn attr_source( node : Node ) -> anyhow::Result<RTMLSource>
{
    let source = node.attribute( "src" )
    .ok_or(
        anyhow::Error::msg( "src expected" )
    )?;

    Ok( RTMLSource::File( source.to_string() ) )
}

// pub fn container_attrs( node : Node ) -> anyhow::Result<ContainerAttrs>
// {
//     Ok( ContainerAttrs::new( attr_direction( node )?, attr_flex( node )?, container_padding_from_node( node ) ) )
// }

pub fn attr_to_type_kebab<T: FromStr>( node : Node, attr : &str ) -> Option<T>
{
    match node.attribute( attr )
    {
        Some( val ) =>
        {
            match ccase!( pascal, val.trim() ).parse::<T>()
                {
                    Ok( n ) => Some( n ),
                    Err( _ ) => None
                }
        },
        None => None
    }
}

pub fn attr_to_type<T: FromStr>( node : Node, attr : &str ) -> Option<T>
{
    match node.attribute( attr )
    {
        Some( val ) =>
        {
            match val.trim().parse::<T>()
                {
                    Ok( n ) => Some( n ),
                    Err( _ ) => None
                }
        },
        None => None
    }
}