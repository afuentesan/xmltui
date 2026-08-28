use std::collections::HashMap;

use ratatui::{buffer::Buffer, layout::{Constraint, Rect}};

use crate::{input::event::InputEvent, rtml::{rtml_attrs::CommonAttrs, rtml_border::RTMLBorder, rtml_button::{RTMLButton, render_rtml_button_focus}, rtml_command::{RTMLCommand, RTMLCommandOutput}, rtml_input::{RTMLInput, render_input_cursor}, rtml_layout::RTMLLayout, rtml_line::RTMLLine, rtml_link::{RTMLLink, render_rtml_link_focus}, rtml_paragraph::RTMLParagraph, rtml_select::{RTMLSelect, render_rtml_select_focus}, rtml_span::RTMLSpan}};

pub type RTMLNodeId = String;

#[derive(Debug)]
pub struct XMLNodeWrapper
{
    pub prefix : String,
    pub suffix : String
}

impl XMLNodeWrapper
{
    pub fn new( prefix : String, suffix : String ) -> Self
    {
        Self { prefix, suffix }
    }
}

#[derive(Debug)]
pub struct RTMLNodeCommon
{
    pub attrs : CommonAttrs,
    pub childs : Vec<RTMLNodeId>,
    pub parent_id : Option<RTMLNodeId>
}

impl RTMLNodeCommon
{
    pub fn new( attrs : CommonAttrs, childs : Vec<RTMLNodeId>, parent_id : Option<RTMLNodeId> ) -> Self
    {
        Self { attrs, childs, parent_id }
    }
}

#[derive(Debug)]
pub enum RTMLNode
{
    Layout( RTMLLayout ),
    Line( RTMLLine ),
    Span( RTMLSpan ),
    Input( RTMLInput ),
    Link( RTMLLink ),
    Command( RTMLCommand ),
    Button( RTMLButton ),
    Border( RTMLBorder ),
    Paragraph( RTMLParagraph ),
    Select( RTMLSelect )
}

impl RTMLNode
{
    pub fn is_focusable( &self ) -> bool
    {
        match self
        {
            RTMLNode::Input( _ ) |
            RTMLNode::Select( _ ) |
            RTMLNode::Button( _ ) |
            RTMLNode::Paragraph( _ ) |
            RTMLNode::Link( _ ) => true,
            RTMLNode::Layout( _ ) |
            RTMLNode::Line( _ ) |
            RTMLNode::Span( _ ) |
            RTMLNode::Border( _ ) |
            RTMLNode::Command( _ ) => false
        }
    }

    pub fn focus_event( &mut self, event : &InputEvent ) -> bool
    {
        match self
        {
            RTMLNode::Input( input ) =>
            {
                input.focus_event( event )
            },
            RTMLNode::Link( link ) =>
            {
                link.focus_event( event )
            },
            RTMLNode::Button( button ) =>
            {
                button.focus_event( event )
            },
            RTMLNode::Paragraph( paragraph ) =>
            {
                paragraph.focus_event( event )
            },
            RTMLNode::Select( select ) =>
            {
                select.focus_event( event )
            },
            RTMLNode::Layout( _ ) |
            RTMLNode::Line( _ ) |
            RTMLNode::Span( _ ) |
            RTMLNode::Border( _ ) |
            RTMLNode::Command( _ ) => false
        }
    }

    pub fn parent_id( &self ) -> Option<&String>
    {
        match self
        {
            RTMLNode::Layout( n ) => n.common.parent_id.as_ref(),
            RTMLNode::Line( n ) => n.common.parent_id.as_ref(),
            RTMLNode::Input( n ) => n.common.parent_id.as_ref(),
            RTMLNode::Span( n ) => n.common.parent_id.as_ref(),
            RTMLNode::Link( n ) => n.common.parent_id.as_ref(),
            RTMLNode::Command( n ) => n.common.parent_id.as_ref(),
            RTMLNode::Button( n ) => n.common.parent_id.as_ref(),
            RTMLNode::Border( n ) => n.common.parent_id.as_ref(),
            RTMLNode::Paragraph( n ) => n.common.parent_id.as_ref(),
            RTMLNode::Select( n ) => n.common.parent_id.as_ref()
        }
    }

    pub fn childs( &self ) -> &Vec<String>
    {
        match self
        {
            RTMLNode::Layout( n ) => &n.common.childs,
            RTMLNode::Line( n ) => &n.common.childs,
            RTMLNode::Input( n ) => &n.common.childs,
            RTMLNode::Span( n ) => &n.common.childs,
            RTMLNode::Link( n ) => &n.common.childs,
            RTMLNode::Command( n ) => &n.common.childs,
            RTMLNode::Button( n ) => &n.common.childs,
            RTMLNode::Border( n ) => &n.common.childs,
            RTMLNode::Paragraph( n ) => &n.common.childs,
            RTMLNode::Select( n ) => &n.common.childs
        }
    }

    pub fn childs_mut( &mut self ) -> &mut Vec<String>
    {
        match self
        {
            RTMLNode::Layout( n ) => &mut n.common.childs,
            RTMLNode::Line( n ) => &mut n.common.childs,
            RTMLNode::Input( n ) => &mut n.common.childs,
            RTMLNode::Span( n ) => &mut n.common.childs,
            RTMLNode::Link( n ) => &mut n.common.childs,
            RTMLNode::Command( n ) => &mut n.common.childs,
            RTMLNode::Button( n ) => &mut n.common.childs,
            RTMLNode::Border( n ) => &mut n.common.childs,
            RTMLNode::Paragraph( n ) => &mut n.common.childs,
            RTMLNode::Select( n ) => &mut n.common.childs
        }
    }

    pub fn set_area( &mut self, area : Rect )
    {
        match self
        {
            RTMLNode::Layout( n ) => n.common.attrs.area = area,
            RTMLNode::Line( n ) => n.common.attrs.area = area,
            RTMLNode::Input( n ) => n.common.attrs.area = area,
            RTMLNode::Span( n ) => n.common.attrs.area = area,
            RTMLNode::Link( n ) => n.common.attrs.area = area,
            RTMLNode::Command( n ) => n.common.attrs.area = area,
            RTMLNode::Button( n ) => n.common.attrs.area = area,
            RTMLNode::Border( n ) => n.common.attrs.area = area,
            RTMLNode::Paragraph( n ) => n.common.attrs.area = area,
            RTMLNode::Select( n ) => n.common.attrs.area = area
        }
    }

    pub fn constraint( &self ) -> &Constraint
    {
        match self
        {
            RTMLNode::Layout( n ) => &n.common.attrs.constraint,
            RTMLNode::Line( n ) => &n.common.attrs.constraint,
            RTMLNode::Input( n ) => &n.common.attrs.constraint,
            RTMLNode::Span( n ) => &n.common.attrs.constraint,
            RTMLNode::Link( n ) => &n.common.attrs.constraint,
            RTMLNode::Command( n ) => &n.common.attrs.constraint,
            RTMLNode::Button( n ) => &n.common.attrs.constraint,
            RTMLNode::Border( n ) => &n.common.attrs.constraint,
            RTMLNode::Paragraph( n ) => &n.common.attrs.constraint,
            RTMLNode::Select( n ) => &n.common.attrs.constraint
        }
    }

    pub fn node_wrapper( &self ) -> Option<&XMLNodeWrapper>
    {
        match self
        {
            RTMLNode::Layout( _ ) |
            RTMLNode::Line( _ ) |
            RTMLNode::Input( _ ) |
            RTMLNode::Span( _ ) |
            RTMLNode::Button( _ ) |
            RTMLNode::Border( _ ) |
            RTMLNode::Paragraph( _ ) |
            RTMLNode::Select( _ ) |
            RTMLNode::Link( _ ) => None,
            RTMLNode::Command( n ) => n.child.as_ref()
        }
    }

    pub fn node_template<'a>( &'a self, templates : &'a HashMap<String, String> ) -> Option<&'a String>
    {
        match self
        {
            RTMLNode::Layout( _ ) |
            RTMLNode::Line( _ ) |
            RTMLNode::Input( _ ) |
            RTMLNode::Span( _ ) |
            RTMLNode::Button( _ ) |
            RTMLNode::Border( _ ) |
            RTMLNode::Paragraph( _ ) |
            RTMLNode::Select( _ ) |
            RTMLNode::Link( _ ) => None,
            RTMLNode::Command( n ) => n.node_template( templates )
        }
    }

    pub fn command_output( &self ) -> RTMLCommandOutput
    {
        match self
        {
            RTMLNode::Layout( _ ) |
            RTMLNode::Line( _ ) |
            RTMLNode::Input( _ ) |
            RTMLNode::Span( _ ) |
            RTMLNode::Button( _ ) |
            RTMLNode::Border( _ ) |
            RTMLNode::Paragraph( _ ) |
            RTMLNode::Select( _ ) |
            RTMLNode::Link( _ ) => RTMLCommandOutput::String,
            RTMLNode::Command( n ) => n.output
        }
    }

    pub fn data( &self ) -> &HashMap<String, String>
    {
        match self
        {
            RTMLNode::Layout( n ) => &n.common.attrs.data,
            RTMLNode::Line( n ) => &n.common.attrs.data,
            RTMLNode::Input( n ) => &n.common.attrs.data,
            RTMLNode::Span( n ) => &n.common.attrs.data,
            RTMLNode::Link( n ) => &n.common.attrs.data,
            RTMLNode::Command( n ) => &n.common.attrs.data,
            RTMLNode::Button( n ) => &n.common.attrs.data,
            RTMLNode::Border( n ) => &n.common.attrs.data,
            RTMLNode::Paragraph( n ) => &n.common.attrs.data,
            RTMLNode::Select( n ) => &n.common.attrs.data
        }
    }

    pub fn replace_value( &mut self, new_value : String ) -> bool
    {
        match self
        {
            RTMLNode::Input( n ) => n.replace_value( new_value ),
            RTMLNode::Button( n ) => n.replace_value( new_value ),
            RTMLNode::Link( n ) => n.replace_value( new_value ),
            RTMLNode::Span( n ) => n.replace_value( new_value ),
            RTMLNode::Select( n ) => n.replace_value( new_value ),
            RTMLNode::Line( _ ) |
            RTMLNode::Command( _ ) |
            RTMLNode::Border( _ ) |
            RTMLNode::Paragraph( _ ) |
            RTMLNode::Layout( _ ) => false
        }
    }

    pub fn value( &self ) -> Option<&str>
    {
        match self
        {
            RTMLNode::Input( n ) => Some( n.value() ),
            RTMLNode::Button( n ) => Some( n.value() ),
            RTMLNode::Link( n ) => Some( n.value() ),
            RTMLNode::Span( n ) => Some( n.value() ),
            RTMLNode::Select( n ) => Some( n.value() ),
            RTMLNode::Line( _ ) |
            RTMLNode::Command( _ ) |
            RTMLNode::Border( _ ) |
            RTMLNode::Paragraph( _ ) |
            RTMLNode::Layout( _ ) => None
        }
    }
}

pub fn render_focus_node( 
    buf : &mut Buffer,
    node : &RTMLNode 
) -> anyhow::Result<()>
{
    match &node
    {
        RTMLNode::Input( input ) => render_input_cursor( input, buf ),
        RTMLNode::Link( link ) => render_rtml_link_focus( link, buf ),
        RTMLNode::Button( button ) => render_rtml_button_focus( button, buf ),
        RTMLNode::Select( select ) => render_rtml_select_focus( select, buf ),
        RTMLNode::Command( _ ) |
        RTMLNode::Layout( _ ) |
        RTMLNode::Line( _ ) |
        RTMLNode::Border( _ ) |
        RTMLNode::Paragraph( _ ) |
        RTMLNode::Span( _ ) => Ok( () )
    }
}