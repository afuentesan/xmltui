use std::time::Duration;

use crate::rtml::rtml_node::{RTMLNodeCommon, XMLNodeWrapper};

#[derive(Debug)]
pub struct RTMLCommand
{
    pub common : RTMLNodeCommon,
    pub executor_id : String,
    pub refresh : CommandRefresh,
    pub child : Option<XMLNodeWrapper>
}

impl RTMLCommand
{
    pub fn new( executor_id : String, refresh : CommandRefresh, common : RTMLNodeCommon, child : Option<XMLNodeWrapper> ) -> Self
    {
        Self 
        { 
            common, 
            executor_id, 
            refresh,
            child
        }
    }
}

#[derive(Debug, Clone)]
pub enum CommandRefresh
{
    Once,
    Repeat( Duration )
}