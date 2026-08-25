use std::collections::HashMap;

use ratatui::layout::{Constraint, Direction, Flex, Rect};

use crate::rtml::rtml_padding::RTMLPadding;


#[derive(Debug)]
pub struct CommonAttrs
{
    pub area : Rect,
    pub constraint : Constraint,
    pub data : HashMap<String, String>
}

#[derive(Debug)]
pub struct ContainerAttrs
{
    pub direction : Direction,
    pub flex : Flex,
    pub padding : RTMLPadding
}

impl ContainerAttrs
{
    pub fn new( direction : Direction, flex : Flex, padding : RTMLPadding ) -> Self
    {
        Self { direction, flex, padding }
    }
}