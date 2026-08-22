use std::time::Duration;

use crate::rtml::rtml_node::{RTMLNodeCommon, XMLNodeWrapper};

#[derive(Debug, Clone, Copy)]
pub enum RTMLCommandOutput
{
    String,
    StrVec
}

#[derive(Debug)]
pub struct RTMLCommand
{
    pub common : RTMLNodeCommon,
    pub executor_id : String,
    pub refresh : CommandRefresh,
    pub child : Option<XMLNodeWrapper>,
    pub template : Option<String>,
    pub output : RTMLCommandOutput
}

impl RTMLCommand
{
    pub fn new( 
        executor_id : String, 
        refresh : CommandRefresh, 
        common : RTMLNodeCommon, 
        child : Option<XMLNodeWrapper>,
        template : Option<String>,
        output : RTMLCommandOutput
    ) -> Self
    {
        Self 
        { 
            common, 
            executor_id, 
            refresh,
            child,
            template,
            output
        }
    }
}

#[derive(Debug, Clone)]
pub enum CommandRefresh
{
    Once,
    Repeat( Duration )
}