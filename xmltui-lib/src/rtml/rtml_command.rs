use std::{collections::HashMap, time::Duration};


use ratatui::{buffer::Buffer, layout::Rect, style::Style};
use serde_json::Value;

use crate::{rtml::{rtml_attrs::ContainerAttrs, rtml_node::{RTMLNodeCommon, XMLNodeWrapper}, util::rtml_style::{RTMLStyleTemplate, merge_style_with_templates}}, util::draw::clear_area};

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
    pub style_template : RTMLStyleTemplate,
    pub executors : Vec<String>,
    pub refresh : CommandRefresh,
    pub child : Option<XMLNodeWrapper>,
    pub template_name : Option<String>,
    pub template : Option<String>,
    pub output : RTMLCommandOutput,
    pub args : HashMap<String, String>,
    pub envs : HashMap<String, String>
}

impl RTMLCommand
{
    pub fn new( 
        executors : Vec<String>, 
        refresh : CommandRefresh, 
        common : RTMLNodeCommon, 
        container : ContainerAttrs,
        style : Option<Style>,
        style_template : RTMLStyleTemplate,
        child : Option<XMLNodeWrapper>,
        template_name : Option<String>,
        template : Option<String>,
        output : RTMLCommandOutput,
        args : HashMap<String, String>,
        envs : HashMap<String, String>
    ) -> Self
    {
        Self 
        { 
            common, 
            container,
            style,
            style_template,
            executors, 
            refresh,
            child,
            template_name,
            template,
            output,
            args,
            envs
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
    buf : &mut Buffer,
    templates : &HashMap<String, String>,
    context : &Value
)
{
    let style = if let Some( s ) = layout.style
    {
        s
    }
    else
    {
        Style::default()    
    };

    let style = merge_style_with_templates( style, &layout.style_template, context, templates );

    clear_area( area, style, buf );
}