use std::{collections::HashMap, time::Duration};


use ratatui::{buffer::Buffer, layout::Rect, style::Style};

use crate::{rtml::{rtml_attrs::ContainerAttrs, rtml_node::{RTMLNodeCommon, XMLNodeWrapper}}, util::draw::clear_area};

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
    pub container : ContainerAttrs,
    pub style : Option<Style>,
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
        container : ContainerAttrs,
        style : Option<Style>,
        child : Option<XMLNodeWrapper>,
        template_name : Option<String>,
        template : Option<String>,
        output : RTMLCommandOutput
    ) -> Self
    {
        Self 
        { 
            common, 
            container,
            style,
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

pub fn render_rtml_command(
    layout : &RTMLCommand,
    area : Rect,
    buf : &mut Buffer
)
{
    match layout.style
    {
        Some( s ) =>
        {
            clear_area( area, s, buf );
        },
        None => {}
    }
}