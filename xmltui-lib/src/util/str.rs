use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;


pub fn str_len( str : &str ) -> usize
{
    str.graphemes( true ).count()
}

pub fn take_width<'a>( text: &'a str, max_width: usize ) -> &'a str
{
    let mut current_width = 0;
    let mut end_byte = 0;

    for( idx, g ) in text.grapheme_indices( true )
    {
        let w = g.width();
        
        if current_width + w > max_width 
        {
            break;
        }
        
        current_width += w;
        end_byte = idx + g.len();
    }

    &text[ ..end_byte ]
}

pub fn substr( text : &str, from : usize, to : usize ) -> &str
{
    if from >= to || text.is_empty() { return "" };

    let mut iter = text.grapheme_indices( true );

    let start_byte = match iter.nth( from )
    {
        Some( ( idx, _ ) ) => idx,
        None => return "",
    };

    let end_byte = match iter.nth( to - from - 1 )
    {
        Some( ( idx, _ ) ) => idx,
        None => text.len(),
    };

    &text[ start_byte..end_byte ]
}

pub fn is_uint( s: &str ) -> bool 
{
    ! s.is_empty() && s.chars().all( | c | c.is_ascii_digit() )
}