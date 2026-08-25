use roxmltree::Node;

use crate::{rtml::rtml_padding::HorizontalPadding, util::str::str_len};

pub fn node_text_len_y_horizontal_padding( node : &Node ) -> usize
{
    let padding = horizontal_padding_from_node( node );

    let mut len = padding.left + padding.right;

    for child in node.children()
    {
        if child.is_text()
        {
            if let Some( t ) = child.text()
            {
                len += str_len( &t.replace( "\n", "" ).trim() );
            }
        }
        else if child.tag_name().name() == "span"
        {
            let padding = horizontal_padding_from_node( &child );

            len = len + padding.left + padding.right;

            if let Some( t ) = child.text()
            {
                len += str_len( &t.replace( "\n", "" ).trim() );
            }
        }
    }

    len
}

pub fn horizontal_padding_from_node( node : &Node ) -> HorizontalPadding
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