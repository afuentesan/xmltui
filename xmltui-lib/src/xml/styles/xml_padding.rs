use roxmltree::Node;
use serde::Deserialize;
use unicode_width::UnicodeWidthStr;

use crate::{rtml::rtml_padding::{HorizontalPadding, RTMLPadding, VerticalPadding}, util::deserialize::deserialize_string_or_type};

#[derive(Deserialize, Default)]
pub struct PaddingBuilder 
{
    #[serde(default, rename = "padding-top", deserialize_with = "deserialize_string_or_type" )]
    pub top : Option<usize>,
    #[serde(default, rename = "padding-bottom", deserialize_with = "deserialize_string_or_type" )]
    pub bottom : Option<usize>,
    #[serde(default, rename = "padding-left", deserialize_with = "deserialize_string_or_type" )]
    pub left : Option<usize>,
    #[serde(default, rename = "padding-right", deserialize_with = "deserialize_string_or_type" )]
    pub right : Option<usize>,
    #[serde(default)]
    pub padding : Option<String>
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(from = "PaddingBuilder")]
pub struct XMLPadding( pub ( Option<HorizontalPadding>, Option<VerticalPadding> ) );

impl From<PaddingBuilder> for XMLPadding 
{
    fn from( builder : PaddingBuilder ) -> Self 
    {
        let mut top = None;
        let mut right = None;
        let mut bottom = None;
        let mut left = None;

        if let Some( p ) = builder.padding
        {
            if let Some( p ) = padding_from_str( &p )
            {
                top = Some( p.0 );
                right = Some( p.1 );
                bottom = Some( p.2 );
                left = Some( p.3 );
            }
        }

        if let Some( t ) = builder.top
        {
            top = Some( t );
        }

        if let Some( r ) = builder.right
        {
            right = Some( r );
        }

        if let Some( b ) = builder.bottom
        {
            bottom = Some( b );
        }

        if let Some( l ) = builder.left
        {
            left = Some( l );
        }

        let vertical = match ( top, bottom )
        {
            ( Some( t ), Some( b ) ) => Some( VerticalPadding::new( t, b ) ),
            ( Some( t ), None ) => Some( VerticalPadding::new( t, 0 ) ),
            ( None, Some( b ) ) => Some( VerticalPadding::new( 0, b ) ),
            ( None, None ) => None
        };

        let horizontal = match ( left, right )
        {
            ( Some( l ), Some( r ) ) => Some( HorizontalPadding::new( l, r ) ),
            ( Some( l ), None ) => Some( HorizontalPadding::new( l, 0 ) ),
            ( None, Some( r ) ) => Some( HorizontalPadding::new( 0, r ) ),
            ( None, None ) => None
        };

        XMLPadding( ( horizontal, vertical ) )
    }
}

pub fn padding_from_str( p : &str ) -> Option<( usize, usize, usize, usize )>
{
    if p.trim() == "" { return None };

    let mut idx = 0;

    let mut padding = ( 0, 0, 0, 0 );

    let mut tiene_alguno = false;

    for s in p.split( "," )
    {
        if let Some( n ) = parse_padding( s )
        {
            tiene_alguno = true;

            if idx == 0
            {
                padding = ( n, n, n, n );
            }
            else if idx == 1
            {
                padding.1 = n;
                padding.3 = n;
            }
            else if idx == 2
            {
                padding.2 = n;
            }
            else
            {
                padding.3 = n;

                break;
            }

            idx += 1;
        }
    }

    if tiene_alguno
    {
        Some( padding )
    }
    else
    {
        None
    }
}

pub fn node_text_len_y_horizontal_padding( node : Node ) -> usize
{
    let padding = horizontal_padding_from_node( node );

    let mut len = padding.left + padding.right;

    for child in node.children()
    {
        if child.is_text()
        {
            if let Some( t ) = child.text()
            {
                len += &t.replace( "\n", "" ).trim().width();
            }
        }
        else if child.tag_name().name() == "span"
        {
            let padding = horizontal_padding_from_node( child );

            len = len + padding.left + padding.right;

            if let Some( t ) = child.text()
            {
                len += &t.replace( "\n", "" ).trim().width();
            }
        }
    }

    len
}

pub fn horizontal_padding_from_node( node : Node ) -> HorizontalPadding
{
    let mut left = 0;
    let mut right = 0;

    if let Some( p ) = node.attribute( "padding" )
    {
        let mut idx = 0;

        for s in p.split( "," )
        {
            if s.trim() == "" { continue };

            if let Ok( s ) = s.trim().parse::<usize>()
            {
                if idx == 0
                {
                    left = s;
                    right = s;
                }
                else if idx == 1
                {
                    right = s;
                }
                else
                {
                    break;    
                }

                idx += 1;
            }
        }
    }

    if let Some( p ) = node.attribute( "padding-left" )
    {
        if let Ok( n ) = p.parse::<usize>()
        {
            left = n;
        }
    }

    if let Some( p ) = node.attribute( "padding-right" )
    {
        if let Ok( n ) = p.parse::<usize>()
        {
            right = n;
        }
    }

    HorizontalPadding::new( left, right )
}

pub fn container_padding_from_node( node : Node ) -> RTMLPadding
{
    let mut padding = padding_all( node, "padding" );

    if let Some( p ) = padding_attr( node, "padding-top" )
    {
        padding[ 0 ] = p;
    }

    if let Some( p ) = padding_attr( node, "padding-right" )
    {
        padding[ 1 ] = p;
    }

    if let Some( p ) = padding_attr( node, "padding-bottom" )
    {
        padding[ 2 ] = p;
    }

    if let Some( p ) = padding_attr( node, "padding-left" )
    {
        padding[ 3 ] = p;
    }

    RTMLPadding::new( padding[ 0 ], padding[ 1 ], padding[ 2 ], padding[ 3 ] )
}

fn padding_all( node : Node, attr : &str ) -> [ usize; 4 ]
{
    if let Some( p ) = node.attribute( attr )
    {
        let mut idx = 0;

        let mut padding = [ 0, 0, 0, 0 ];

        for s in p.split( "," )
        {
            if let Some( n ) = parse_padding( s )
            {
                if idx == 0
                {
                    padding = [ n, n, n, n ];
                }
                else if idx == 1
                {
                    padding[ 1 ] = n;
                    padding[ 3 ] = n;
                }
                else if idx == 2
                {
                    padding[ 2 ] = n;
                }
                else
                {
                    padding[ 3 ] = n;

                    break;
                }

                idx += 1;
            }
        }

        padding
    }
    else
    {
        [ 0, 0, 0, 0 ]
    }
}

fn padding_attr( node : Node, attr : &str ) -> Option<usize>
{
    if let Some( p ) = node.attribute( attr ) && p.trim() != ""
    {
        parse_padding( p )
    }
    else
    {
        None
    }
}

fn parse_padding( padding : &str ) -> Option<usize>
{
    match padding.trim().parse::<usize>()
    {
        Ok( n ) => Some( n ),
        _ => None
    }
}