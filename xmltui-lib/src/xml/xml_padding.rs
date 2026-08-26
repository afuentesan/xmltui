use roxmltree::Node;
use unicode_width::UnicodeWidthStr;

use crate::rtml::rtml_padding::{HorizontalPadding, RTMLPadding};

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
        let p = p.split( "," )
        .flat_map(
            | s |
            {
                if s.trim() == ""
                {
                    None
                }
                else if let Ok( n ) = s.parse::<usize>()
                {
                    Some( n )
                }
                else
                {
                    None
                }
            }
        )
        .collect::<Vec<_>>();

        if p.len() > 0
        {
            left = p[ 0 ];
        }

        if p.len() > 1
        {
            right = p[ 1 ];
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
        let mut padding = [ 0, 0, 0, 0 ];

        for ( i, s ) in p.split( "," ).enumerate()
        {
            padding[ i ] = parse_padding( s );
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
        Some( parse_padding( p ) )
    }
    else
    {
        None
    }
}

fn parse_padding( padding : &str ) -> usize
{
    match padding.trim().parse::<usize>()
    {
        Ok( n ) => n,
        _ => 0
    }
}