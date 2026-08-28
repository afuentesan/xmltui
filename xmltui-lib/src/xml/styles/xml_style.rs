use std::{collections::HashMap, str::FromStr};

use ratatui::style::Style;
use roxmltree::Node;

use crate::{app::app_doc::chroot, util::file::read_file_in_chroot_with_extension};

#[derive(Debug, PartialEq, Hash, Eq)]
pub enum StyleSelector
{
    Class( String ),
    TagName( String ),
    Id( String )
}

impl ToString for StyleSelector
{
    fn to_string( &self ) -> String 
    {
        match self
        {
            StyleSelector::Class( s ) |
            StyleSelector::Id( s ) |
            StyleSelector::TagName( s ) => s.clone()    
        }
    }
}

impl FromStr for StyleSelector
{
    type Err = anyhow::Error;

    fn from_str( s : &str ) -> Result<Self, Self::Err> 
    {
        let s = s.trim();

        if ( s.starts_with( "." ) || s.starts_with( "#" ) ) && s.len() < 2
        {
            return Err( anyhow::Error::msg( format!( "Invalid selector. Selector: {}", s ) ) )
        }

        if s.starts_with( "." )
        {
            Ok( StyleSelector::Class( s[ 1.. ].to_string() ) )
        }
        else if s.starts_with( "#" )
        {
            Ok( StyleSelector::Id( s[ 1.. ].to_string() ) )
        }
        else
        {
            Ok( StyleSelector::TagName( s.to_string() ) )   
        }
    }
}

pub enum StyleVariant
{
    Focus,
    Border,
    Title,
    Selected
}

impl ToString for StyleVariant
{
    fn to_string( &self ) -> String 
    {
        match self
        {
            StyleVariant::Focus => String::from( "focus" ),
            StyleVariant::Border => "border".to_string(),
            StyleVariant::Title => "title".to_string(),
            StyleVariant::Selected => "select".to_string()
        }
    }
}

fn calc_variant( prefix : &str, variant : Option<&StyleVariant> ) -> String
{
    match variant
    {
        Some( v ) => format!( "{}:{}", prefix, v.to_string() ),
        None => prefix.to_string()    
    }
}

pub fn style_from_container( node : Node, styles : &HashMap<StyleSelector, Style>, default_style : Style ) -> Option<Style>
{
    let style = style_from_node( node, styles, default_style, None );

    if style == Style::default()
    {
        None
    }
    else
    {
        Some( style )    
    }
}

pub fn style_from_node( node : Node, styles : &HashMap<StyleSelector, Style>, default_style : Style, variant : Option<StyleVariant> ) -> Style
{
    let mut style = default_style;

    if let Some( s ) = style_from_tagname( node, styles, variant.as_ref() )
    {
        style = merge_styles( style, *s );
    }

    if let Some( s ) = style_from_classes( node, styles, variant.as_ref() )
    {
        style = merge_styles( style, s );
    }

    if let Some( s ) = style_from_id( node, styles, variant.as_ref() )
    {
        style = merge_styles( style, *s );
    }

    style
}

// pub fn classes_from_node( node : Node ) -> Vec<String>
// {
//     node.attribute( "class" )
//     .unwrap_or( "" )
//     .split( " " )
//     .filter( | s | s.trim() != "" )
//     .map( | s | s.to_string() )
//     .collect()
// }

fn style_from_classes<'a, 'input, 'b>( node : Node<'a, 'input>, styles : &'b HashMap<StyleSelector, Style>, variant : Option<&StyleVariant> ) -> Option<Style>
{
    if let Some( cls ) = node.attribute( "class" ) && cls.trim() != ""
    {
        let mut style = None;

        for cls in cls.split( " " )
        {
            match style_from_classname( cls, styles, variant )
            {
                Some( s ) =>
                {
                    if style.is_none()
                    {
                        style = Some( *s );
                    }
                    else
                    {
                        let current = style.unwrap();

                        style = Some( merge_styles( current, *s ) );
                    }
                },
                None => continue    
            }
        }

        style
    }
    else
    {
        None
    }
}

fn style_from_classname<'a, 'b>( classname : &'a str, styles : &'b HashMap<StyleSelector, Style>, variant : Option<&StyleVariant> ) -> Option<&'b Style>
{
    let key = StyleSelector::Class( calc_variant( classname, variant ) );

    styles.get( &key )
}

fn style_from_id<'a, 'input, 'b>( node : Node<'a, 'input>, styles : &'b HashMap<StyleSelector, Style>, variant : Option<&StyleVariant> ) -> Option<&'b Style>
{
    if let Some( id ) = node.attribute( "id" ) && id.trim() != ""
    {
        let key = StyleSelector::Id( 
            calc_variant( id, variant )
        );

        styles.get( &key )
    }
    else
    {
        None    
    }
}

fn style_from_tagname<'a, 'input, 'b>( node : Node<'a, 'input>, styles : &'b HashMap<StyleSelector, Style>, variant : Option<&StyleVariant> ) -> Option<&'b Style>
{
    let key = StyleSelector::TagName( 
        calc_variant(
            node.tag_name().name(), 
            variant
        )
    );

    styles.get( &key )
}

pub fn styles_from_head( node : Option<Node> ) -> anyhow::Result<HashMap<StyleSelector, Style>>
{
    let mut ret = HashMap::new();

    if node.is_none() { return Ok( ret ) };

    let node = node.unwrap();

    for child in node.children()
    {
        add_styles( child, &mut ret )?;
    }

    Ok( ret )
}

fn add_styles( node : Node, styles : &mut HashMap<StyleSelector, Style> ) -> anyhow::Result<()>
{
    if node.tag_name().name() != "style" { return Ok( () ) };

    if node.has_attribute( "src" )
    {
        add_styles_from_file( node.attribute( "src" ).unwrap(), styles )
    }
    else
    {
        add_styles_from_content( node, styles )
    }
}

fn add_styles_from_file( path : &str, styles : &mut HashMap<StyleSelector, Style> ) -> anyhow::Result<()>
{
    let str_styles = read_file_in_chroot_with_extension( path, chroot(), "json" )?;

    if str_styles.trim() == "" { return Ok( () ) };

    add_styles_from_str( str_styles.as_str(), styles )
}

fn add_styles_from_content( node : Node, styles : &mut HashMap<StyleSelector, Style> ) -> anyhow::Result<()>
{
    match node.text()
    {
        Some( s ) if s.trim() != "" => add_styles_from_str( s, styles ),
        _ => Ok( () )
    }
}

fn add_styles_from_str( str_styles: &str, styles : &mut HashMap<StyleSelector, Style> ) -> anyhow::Result<()>
{
    let map_styles : HashMap<String, Style> = serde_json::from_str( str_styles )?;

    add_styles_from_map( map_styles, styles )
}

fn add_styles_from_map( map_styles : HashMap<String, Style>, styles : &mut HashMap<StyleSelector, Style> ) -> anyhow::Result<()>
{
    for ( selectors, style ) in map_styles
    {
        let selectors = style_selectors( &selectors )?;

        insert_style_selectors( selectors, style, styles );
    }

    Ok( () )
}

fn insert_style_selectors( selectors : Vec<StyleSelector>, style : Style, styles : &mut HashMap<StyleSelector, Style> )
{
    selectors.into_iter()
    .for_each(
        | s |
        {
            insert_style_selector( s, style, styles );
        }
    );
}

fn insert_style_selector( selector : StyleSelector, mut style : Style, styles : &mut HashMap<StyleSelector, Style> )
{
    if styles.contains_key( &selector )
    {
        let current = styles.remove( &selector ).unwrap();

        style = merge_styles( current, style );
    }
    
    styles.insert( selector, style ); 
}

pub fn merge_styles( mut current : Style, new : Style ) -> Style
{
    if let Some( fg ) = new.fg
    {
        current = current.fg( fg );
    }

    if let Some( bg ) = new.bg
    {
        current = current.bg( bg );
    }

    if let Some( uc ) = new.underline_color
    {
        current = current.underline_color( uc );
    }

    current.add_modifier = current.add_modifier.union( new.add_modifier );

    let sub_modifiers = current.sub_modifier.difference( new.add_modifier );
    
    current.sub_modifier = sub_modifiers.union( new.sub_modifier );

    current
}

fn style_selectors( selectors : &str ) -> anyhow::Result<Vec<StyleSelector>>
{
    let selectors = selectors.trim().split( "," );

    let mut ret = vec![];

    for selector in selectors
    {
        ret.push( StyleSelector::from_str( selector )? );
    }

    Ok( ret )
}