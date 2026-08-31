use serde::Deserialize;


#[derive(Deserialize, Debug, Clone)]
pub struct HorizontalPadding
{
    #[serde(default, rename = "padding-left" )]
    pub left : usize,
    #[serde(default, rename = "padding-right" )]
    pub right : usize
}

impl HorizontalPadding
{
    pub fn new( left : usize, right : usize ) -> Self
    {
        Self { left, right }
    }
}

impl Default for HorizontalPadding
{
    fn default() -> Self 
    {
        Self { left : Default::default(), right : Default::default() }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct VerticalPadding
{
    #[serde(default, rename = "padding-top" )]
    pub top : usize,
    #[serde(default, rename = "padding-bottom" )]
    pub bottom : usize
}

impl VerticalPadding
{
    pub fn new( top : usize, bottom : usize ) -> Self
    {
        Self { top, bottom }
    }
}

impl Default for VerticalPadding
{
    fn default() -> Self 
    {
        Self { top : Default::default(), bottom : Default::default() }
    }
}

#[derive(Deserialize, Debug)]
pub struct RTMLPadding 
{
    #[serde(flatten, default)]
    pub horizontal : HorizontalPadding,
    #[serde(flatten, default)]
    pub vertical : VerticalPadding
}

impl RTMLPadding
{
    pub fn new( horizontal : HorizontalPadding, vertical : VerticalPadding ) -> Self
    {
        Self { horizontal, vertical }
    }
}

impl Default for RTMLPadding
{
    fn default() -> Self 
    {
        Self { horizontal : Default::default(), vertical : Default::default() }
    }
}