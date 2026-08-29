use std::collections::HashMap;

use ratatui::style::Style;

use crate::{rtml::rtml_node::{RTMLNode, RTMLNodeId}, xml::styles::xml_style::StyleSelector};


pub struct XMLDoc<'a>
{
    nodos : &'a mut HashMap<RTMLNodeId, RTMLNode>,
    styles : &'a HashMap<StyleSelector, Style>,
    focus : Option<RTMLNodeId>
}

impl<'a> XMLDoc<'a>
{
    pub fn new( 
        nodos : &'a mut HashMap<RTMLNodeId, RTMLNode>, 
        styles : &'a HashMap<StyleSelector, Style>, 
        focus : Option<RTMLNodeId> 
    ) -> Self
    {
        Self { nodos, styles, focus }
    }

    pub fn replace_focus( &mut self, focus : RTMLNodeId )
    {
        self.focus = Some( focus )
    }

    pub fn add_node( &mut self, nodo : RTMLNode, nodo_id : RTMLNodeId )
    {
        self.nodos.insert( nodo_id, nodo );
    }

    pub fn nodos( &self ) -> &HashMap<RTMLNodeId, RTMLNode>
    {
        self.nodos
    }

    pub fn nodos_mut( &mut self ) -> &mut HashMap<RTMLNodeId, RTMLNode>
    {
        self.nodos
    }

    pub fn styles( &self ) -> &HashMap<StyleSelector, Style>
    {
        self.styles
    }
}