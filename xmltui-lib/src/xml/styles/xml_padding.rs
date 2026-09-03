use serde::Deserialize;

use crate::{rtml::rtml_padding::{HorizontalPadding, VerticalPadding}, util::json::deserialize_string_or_type};

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

fn parse_padding( padding : &str ) -> Option<usize>
{
    match padding.trim().parse::<usize>()
    {
        Ok( n ) => Some( n ),
        _ => None
    }
}