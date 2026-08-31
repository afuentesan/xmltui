use std::{collections::HashMap, str::FromStr};

use ratatui::{layout::{Alignment, Constraint, Direction, Flex}, style::{Color, Modifier, Style}};
use roxmltree::Node;
use serde::Deserialize;

use crate::{app::app_doc::chroot, rtml::rtml_padding::{HorizontalPadding, VerticalPadding}, util::{deserialize::{deserialize_kebab_string_or_type, deserialize_string_or_type}, file::read_file_in_chroot_with_extension}, xml::{attrs::{attr_constraint, attr_to_type, attr_to_type_kebab}, styles::{xml_constraint::XMLConstraint, xml_padding::{XMLPadding, padding_from_str}}}};

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

#[derive(Deserialize, Default)]
pub struct RTStyleBuilder
{
    #[serde( default, deserialize_with = "deserialize_string_or_type" )]
    pub fg : Option<Color>,
    #[serde( default, deserialize_with = "deserialize_string_or_type" )]
    pub bg : Option<Color>,
    #[serde( default, deserialize_with = "deserialize_string_or_type" )]
    pub uc : Option<Color>,
    // normal, bold
    pub font_weight : Option<String>,
    // normal, italic
    pub font_style : Option<String>,
    pub dim : Option<bool>,
    // underline, line-through, none
    pub text_decoration : Option<String>,
    // normal, slow, rapid
    pub blink : Option<String>,
    #[serde( default, deserialize_with = "deserialize_string_or_type" )]
    pub invert : Option<bool>,
    // visible, hidden
    pub visibility : Option<String>
}

impl RTStyleBuilder
{
    pub fn from_node( node : Node ) -> Self
    {
        Self 
        { 
            fg : attr_to_type( node, "fg" ), 
            bg : attr_to_type( node, "bg" ), 
            uc : attr_to_type( node, "uc" ), 
            font_weight : attr_to_type( node, "font-weight" ), 
            font_style : attr_to_type( node, "font-style" ), 
            dim : attr_to_type( node, "dim" ), 
            text_decoration : attr_to_type( node, "text-decoration" ), 
            blink : attr_to_type( node, "blink" ), 
            invert : attr_to_type( node, "invert" ), 
            visibility : attr_to_type( node, "visibility" )
        }
    }
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(from = "RTStyleBuilder")]
pub struct RTStyle( pub Option<Style> );

impl From<RTStyleBuilder> for RTStyle 
{
    fn from( builder : RTStyleBuilder ) -> Self
    {
        let mut style : Option<Style> = None;

        if let Some( c ) = builder.fg
        {
            style = Some( style.unwrap_or_default().fg( c ) );
        }

        if let Some( c ) = builder.bg
        {
            style = Some( style.unwrap_or_default().bg( c ) );
        }

        if let Some( c ) = builder.uc
        {
            style = Some( style.unwrap_or_default().underline_color( c ) );
        }

        if let Some( f ) = builder.font_weight
        {
            match f.trim().to_lowercase().as_str()
            {
                "normal" if style.is_some() =>
                {
                    style = Some( style.unwrap().remove_modifier( Modifier::BOLD ) );
                },
                "bold" =>
                {
                    style = Some( style.unwrap_or_default().add_modifier( Modifier::BOLD ) );
                },
                _ => {}
            }
        }

        if let Some( f ) = builder.font_style
        {
            match f.trim().to_lowercase().as_str()
            {
                "normal" if style.is_some() =>
                {
                    style = Some( style.unwrap().remove_modifier( Modifier::ITALIC ) );
                },
                "italic" =>
                {
                    style = Some( style.unwrap_or_default().add_modifier( Modifier::ITALIC ) );
                },
                _ => {}
            }
        }

        if let Some( d ) = builder.dim
        {
            if d
            {
                style = Some( style.unwrap_or_default().add_modifier( Modifier::DIM ) );
            }
            else if style.is_some()
            {
                style = Some( style.unwrap().remove_modifier( Modifier::DIM ) );
            }
        }

        if let Some( t ) = builder.text_decoration
        {
            match t.trim().to_lowercase().as_str()
            {
                "none" if style.is_some() =>
                {
                    style = Some( style.unwrap().remove_modifier( Modifier::UNDERLINED ).remove_modifier( Modifier::CROSSED_OUT ) );
                },
                "underline" =>
                {
                    style = Some( style.unwrap_or_default().remove_modifier( Modifier::CROSSED_OUT ).add_modifier( Modifier::UNDERLINED ) );
                },
                "line-through" =>
                {
                    style = Some( style.unwrap_or_default().remove_modifier( Modifier::UNDERLINED ).add_modifier( Modifier::CROSSED_OUT ) );
                },
                _ => {}
            }
        }

        if let Some( b ) = builder.blink
        {
            match b.trim().to_lowercase().as_str()
            {
                "normal" if style.is_some() =>
                {
                    style = Some( style.unwrap().remove_modifier( Modifier::SLOW_BLINK ).remove_modifier( Modifier::RAPID_BLINK ) );
                },
                "slow" =>
                {
                    style = Some( style.unwrap_or_default().remove_modifier( Modifier::RAPID_BLINK ).add_modifier( Modifier::SLOW_BLINK ) );
                },
                "rapid" =>
                {
                    style = Some( style.unwrap_or_default().remove_modifier( Modifier::SLOW_BLINK ).add_modifier( Modifier::RAPID_BLINK ) );
                },
                _ => {}
            }
        }

        if let Some( i ) = builder.invert
        {
            if i
            {
                style = Some( style.unwrap_or_default().add_modifier( Modifier::REVERSED ) );
            }
            else if style.is_some()
            {
                style = Some( style.unwrap().remove_modifier( Modifier::REVERSED ) );
            }
        }

        if let Some( f ) = builder.visibility
        {
            match f.trim().to_lowercase().as_str()
            {
                "visible" if style.is_some() =>
                {
                    style = Some( style.unwrap().remove_modifier( Modifier::HIDDEN ) );
                },
                "hidden" =>
                {
                    style = Some( style.unwrap_or_default().add_modifier( Modifier::HIDDEN ) );
                },
                _ => {}
            }
        }

        RTStyle( style )
    }
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct XMLStyle
{
    #[serde(flatten)]
    pub style : RTStyle,

    #[serde(flatten)]
    pub constraint : XMLConstraint,

    #[serde(default, rename = "align", deserialize_with = "deserialize_kebab_string_or_type")]
    pub alignment : Option<Alignment>,

    #[serde(default, rename = "dir", deserialize_with = "deserialize_kebab_string_or_type" )]
    pub direction : Option<Direction>,

    #[serde(default, deserialize_with = "deserialize_kebab_string_or_type")]
    pub flex : Option<Flex>,

    #[serde(flatten)]
    pub inner_padding : XMLPadding
}

pub fn style_from_node( node : Node, styles : &HashMap<StyleSelector, XMLStyle>, variant : Option<StyleVariant> ) -> XMLStyle
{
    let mut style = XMLStyle::default();

    if let Some( s ) = style_from_tagname( node, styles, variant.as_ref() )
    {
        style = merge_xml_styles( style, s.clone() );
    }

    if let Some( s ) = style_from_classes( node, styles, variant.as_ref() )
    {
        style = merge_xml_styles( style, s );
    }

    if let Some( s ) = style_from_id( node, styles, variant.as_ref() )
    {
        style = merge_xml_styles( style, s.clone() );
    }

    overwrite_styles_from_node( node, style )
}

fn overwrite_styles_from_node( node : Node, mut style : XMLStyle ) -> XMLStyle
{
    if let Some( a ) = attr_to_type_kebab::<Alignment>( node, "align" )
    {
        style.alignment = Some( a );
    }

    if let Some( a ) = attr_to_type_kebab::<Direction>( node, "dir" )
    {
        style.direction = Some( a );
    }

    if let Some( f ) = attr_to_type_kebab::<Flex>( node, "flex" )
    {
        style.flex = Some( f );
    }
    
    style = overwrite_constraint_from_node( node, style );

    style = overwrite_padding_from_node( node, style );

    style = overwrite_style_from_node( node, style );

    style
}

fn overwrite_style_from_node( node : Node, mut style : XMLStyle ) -> XMLStyle
{
    let builder = RTStyleBuilder::from_node( node );

    let rt_style : RTStyle = builder.into();

    if let Some( s ) = rt_style.0
    {
        match style.style.0
        {
            Some( current ) =>
            {
                style.style.0 = Some( merge_styles( current, s ) );
            },
            None =>
            {
                style.style.0 = Some( s );
            }
        }
    }

    style
}

fn overwrite_padding_from_node( node : Node, mut style : XMLStyle ) -> XMLStyle
{
    let mut top = None;
    let mut right = None;
    let mut bottom = None;
    let mut left = None;

    if let Some( val ) = node.attribute( "padding" )
    {
        if let Some( ( a, b, c, d ) ) = padding_from_str( val )
        {
            top = Some( a );
            right = Some( b );
            bottom = Some( c );
            left = Some( d );
        }
    }

    if let Some( a ) = attr_to_type::<usize>( node, "padding-top" )
    {
        top = Some( a );
    }

    if let Some( a ) = attr_to_type::<usize>( node, "padding-right" )
    {
        right = Some( a );
    }

    if let Some( a ) = attr_to_type::<usize>( node, "padding-bottom" )
    {
        bottom = Some( a );
    }

    if let Some( a ) = attr_to_type::<usize>( node, "padding-left" )
    {
        left = Some( a );
    }

    if let Some( t ) = top
    {
        match style.inner_padding.0.1
        {
            Some( mut p ) =>
            {
                p.top = t;

                style.inner_padding.0.1 = Some( p );
            },
            None =>
            {
                style.inner_padding.0.1 = Some( VerticalPadding::new( t, 0 ) );
            }
        }
    }

    if let Some( t ) = bottom
    {
        match style.inner_padding.0.1
        {
            Some( mut p ) =>
            {
                p.bottom = t;

                style.inner_padding.0.1 = Some( p );
            },
            None =>
            {
                style.inner_padding.0.1 = Some( VerticalPadding::new( 0, t ) );
            }
        }
    }

    if let Some( t ) = left
    {
        match style.inner_padding.0.0
        {
            Some( mut p ) =>
            {
                p.left = t;

                style.inner_padding.0.0 = Some( p );
            },
            None =>
            {
                style.inner_padding.0.0 = Some( HorizontalPadding::new( t, 0 ) );
            }
        }
    }

    if let Some( t ) = right
    {
        match style.inner_padding.0.0
        {
            Some( mut p ) =>
            {
                p.right = t;

                style.inner_padding.0.0 = Some( p );
            },
            None =>
            {
                style.inner_padding.0.0 = Some( HorizontalPadding::new( 0, t ) );
            }
        }
    }

    style
}

fn overwrite_constraint_from_node( node : Node, mut style : XMLStyle ) -> XMLStyle
{
    if let Some( c ) = attr_constraint( node )
    {
        style.constraint.0 = Some( c );
    }

    style
}

fn style_from_classes<'a, 'input, 'b>( node : Node<'a, 'input>, styles : &'b HashMap<StyleSelector, XMLStyle>, variant : Option<&StyleVariant> ) -> Option<XMLStyle>
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
                        style = Some( s.clone() );
                    }
                    else
                    {
                        let current = style.unwrap();

                        style = Some( merge_xml_styles( current, s.clone() ) );
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

fn style_from_classname<'a, 'b>( classname : &'a str, styles : &'b HashMap<StyleSelector, XMLStyle>, variant : Option<&StyleVariant> ) -> Option<&'b XMLStyle>
{
    let key = StyleSelector::Class( calc_variant( classname, variant ) );

    styles.get( &key )
}

fn style_from_id<'a, 'input, 'b>( node : Node<'a, 'input>, styles : &'b HashMap<StyleSelector, XMLStyle>, variant : Option<&StyleVariant> ) -> Option<&'b XMLStyle>
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

fn style_from_tagname<'a, 'input, 'b>( node : Node<'a, 'input>, styles : &'b HashMap<StyleSelector, XMLStyle>, variant : Option<&StyleVariant> ) -> Option<&'b XMLStyle>
{
    let key = StyleSelector::TagName( 
        calc_variant(
            node.tag_name().name(), 
            variant
        )
    );

    styles.get( &key )
}

pub fn styles_from_head( node : Option<Node> ) -> anyhow::Result<HashMap<StyleSelector, XMLStyle>>
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

fn add_styles( node : Node, styles : &mut HashMap<StyleSelector, XMLStyle> ) -> anyhow::Result<()>
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

fn add_styles_from_file( path : &str, styles : &mut HashMap<StyleSelector, XMLStyle> ) -> anyhow::Result<()>
{
    let str_styles = read_file_in_chroot_with_extension( path, chroot(), "json" )?;

    if str_styles.trim() == "" { return Ok( () ) };

    add_styles_from_str( str_styles.as_str(), styles )
}

fn add_styles_from_content( node : Node, styles : &mut HashMap<StyleSelector, XMLStyle> ) -> anyhow::Result<()>
{
    match node.text()
    {
        Some( s ) if s.trim() != "" => add_styles_from_str( s, styles ),
        _ => Ok( () )
    }
}

fn add_styles_from_str( str_styles: &str, styles : &mut HashMap<StyleSelector, XMLStyle> ) -> anyhow::Result<()>
{
    let map_styles : HashMap<String, XMLStyle> = serde_json::from_str( str_styles )?;

    add_styles_from_map( map_styles, styles )
}

fn add_styles_from_map( map_styles : HashMap<String, XMLStyle>, styles : &mut HashMap<StyleSelector, XMLStyle> ) -> anyhow::Result<()>
{
    for ( selectors, style ) in map_styles
    {
        let selectors = style_selectors( &selectors )?;

        insert_style_selectors( selectors, style, styles );
    }

    Ok( () )
}

fn insert_style_selectors( selectors : Vec<StyleSelector>, style : XMLStyle, styles : &mut HashMap<StyleSelector, XMLStyle> )
{
    selectors.into_iter()
    .for_each(
        | s |
        {
            insert_style_selector( s, style.clone(), styles );
        }
    );
}

fn insert_style_selector( selector : StyleSelector, mut style : XMLStyle, styles : &mut HashMap<StyleSelector, XMLStyle> )
{
    if styles.contains_key( &selector )
    {
        let current = styles.remove( &selector ).unwrap();

        style = merge_xml_styles( current, style );
    }
    
    styles.insert( selector, style ); 
}

pub fn merge_xml_styles( mut current : XMLStyle, new : XMLStyle ) -> XMLStyle
{
    current = merge_xml_style( current, new.style.0 );
    current = merge_xml_constraint( current, new.constraint.0 );
    current = merge_xml_alignment( current, new.alignment );
    current = merge_xml_direction( current, new.direction );
    current = merge_xml_flex( current, new.flex );
    current = merge_xml_padding( current, new.inner_padding.0 );

    current
}

pub fn merge_xml_padding( mut current : XMLStyle, new : ( Option<HorizontalPadding>, Option<VerticalPadding> ) ) -> XMLStyle
{
    match new.0
    {
        Some( p ) =>
        {
            current.inner_padding.0.0 = Some( p );
        },
        None => {}
    }

    match new.1
    {
        Some( p ) =>
        {
            current.inner_padding.0.1 = Some( p );
        },
        None => {}
    }

    current
}

pub fn merge_xml_flex( mut current : XMLStyle, new : Option<Flex> ) -> XMLStyle
{
    match new
    {
        Some( f ) =>
        {
            current.flex = Some( f );

            current
        },
        None => current
    }
}

pub fn merge_xml_direction( mut current : XMLStyle, new : Option<Direction> ) -> XMLStyle
{
    match new
    {
        Some( d ) =>
        {
            current.direction = Some( d );

            current
        },
        None => current
    }
}

pub fn merge_xml_alignment( mut current : XMLStyle, new : Option<Alignment> ) -> XMLStyle
{
    match new
    {
        Some( a ) =>
        {
            current.alignment = Some( a );

            current
        },
        None => current
    }
}

pub fn merge_xml_constraint( mut current : XMLStyle, new : Option<Constraint> ) -> XMLStyle
{
    match new
    {
        Some( c ) =>
        {
            current.constraint.0 = Some( c );
            current
        },
        None => current
    }
}

pub fn merge_xml_style( mut current : XMLStyle, new : Option<Style> ) -> XMLStyle
{
    match new
    {
        Some( s ) =>
        {
            match current.style.0
            {
                Some( cs ) =>
                {
                    current.style.0 = Some( merge_styles( cs, s ) );

                    current
                },
                None =>
                {
                    current.style.0 = Some( s );

                    current
                }
            }
        },
        None => current
    }
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