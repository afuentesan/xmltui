use unicode_segmentation::UnicodeSegmentation;


pub fn str_len( str : &str ) -> usize
{
    str.chars().count()
}

pub fn substr( str : &str, from : usize, to : usize ) -> &str
{
    let indices = str.grapheme_indices( true )
    .collect::<Vec<(usize, &str)>>();

    if from >= indices.len() { return "" };

    if to >= indices.len()
    {
        &str[indices[from].0..]
    }
    else
    {
        &str[indices[from].0..indices[to].0]
    }
}