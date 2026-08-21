use std::collections::HashMap;

use ratatui::layout::{Constraint, Rect};


#[derive(Debug)]
pub struct CommonAttrs
{
    pub area : Rect,
    pub constraint : Constraint,
    pub data : HashMap<String, String>
}