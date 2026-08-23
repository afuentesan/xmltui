use std::{collections::HashMap, time::Duration};

use ratatui::layout::{Direction, Flex};

use crate::rtml::rtml_node::{RTMLNodeCommon, XMLNodeWrapper};

#[derive(Debug, Clone, Copy)]
pub enum RTMLCommandOutput
{
    String,
    StrVec,
    Json
}

#[derive(Debug)]
pub struct RTMLCommand
{
    pub common : RTMLNodeCommon,
    pub direction : Direction,
    pub flex : Flex,
    pub executors : Vec<String>,
    pub refresh : CommandRefresh,
    pub child : Option<XMLNodeWrapper>,
    pub template_name : Option<String>,
    pub template : Option<String>,
    pub output : RTMLCommandOutput
}

impl RTMLCommand
{
    pub fn new( 
        executors : Vec<String>, 
        refresh : CommandRefresh, 
        common : RTMLNodeCommon, 
        direction : Direction,
        flex : Flex,
        child : Option<XMLNodeWrapper>,
        template_name : Option<String>,
        template : Option<String>,
        output : RTMLCommandOutput
    ) -> Self
    {
        Self 
        { 
            common, 
            direction,
            flex,
            executors, 
            refresh,
            child,
            template_name,
            template,
            output
        }
    }

    pub fn node_template<'a>( &'a self, templates : &'a HashMap<String, String> ) -> Option<&'a String>
    {
        if let Some( n ) = self.template_name.as_ref() && templates.contains_key( n )
        {
            return templates.get( n )
        }

        self.template.as_ref()
    }
}

#[derive(Debug, Clone)]
pub enum CommandRefresh
{
    Once,
    Repeat( Duration )
}