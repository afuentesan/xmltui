
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