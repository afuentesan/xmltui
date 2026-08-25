
#[derive(Debug)]
pub struct HorizontalPadding
{
    pub left : usize,
    pub right : usize
}

impl HorizontalPadding
{
    pub fn new( left : usize, right : usize ) -> Self
    {
        Self { left, right }
    }
}

#[derive(Debug)]
pub struct VerticalPadding
{
    pub top : usize,
    pub bottom : usize
}

impl VerticalPadding
{
    pub fn new( top : usize, bottom : usize ) -> Self
    {
        Self { top, bottom }
    }
}

#[derive(Debug)]
pub struct RTMLPadding 
{
    pub horizontal : HorizontalPadding,
    pub vertical : VerticalPadding
}

impl RTMLPadding
{
    pub fn new( top : usize, right : usize, bottom : usize, left : usize ) -> Self
    {
        Self 
        { 
            horizontal : HorizontalPadding::new( left, right ), 
            vertical : VerticalPadding::new( top, bottom )
        }
    }
}