use std::collections::HashMap;

use ratatui::style::Style;
use roxmltree::Node;

use crate::{rtml::rtml_node::{RTMLNode, RTMLNodeId}, xml::styles::xml_style::{StyleSelector, XMLStyle}};


pub struct XMLDoc<'a>
{
    nodos : &'a mut HashMap<RTMLNodeId, RTMLNode>,
    styles : &'a HashMap<StyleSelector, Style>,
    styles_2 : &'a HashMap<StyleSelector, XMLStyle>,
    focus : Option<RTMLNodeId>
}

impl<'a> XMLDoc<'a>
{
    pub fn new( 
        nodos : &'a mut HashMap<RTMLNodeId, RTMLNode>, 
        styles : &'a HashMap<StyleSelector, Style>, 
        styles_2 : &'a HashMap<StyleSelector, XMLStyle>, 
        focus : Option<RTMLNodeId> 
    ) -> Self
    {
        Self { nodos, styles, styles_2, focus }
    }

    fn replace_focus( &mut self, focus : RTMLNodeId )
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

    pub fn styles_2( &self ) -> &HashMap<StyleSelector, XMLStyle>
    {
        self.styles_2
    }

    pub fn consume_focus( self ) -> Option<RTMLNodeId>
    {
        self.focus
    }
}

pub fn replace_xml_doc_focus( xml_doc : &mut XMLDoc, node : Node, id : &str )
{
    if let Some( f ) = node.attribute( "set-focus" ) && f.trim() == "true"
    {
        xml_doc.replace_focus( id.to_string() );
    }
}

pub struct XMLDocResult
{
    root : Option<( RTMLNode, RTMLNodeId )>,
    focus : Option<RTMLNodeId>
}

impl XMLDocResult
{
    pub fn new( root : Option<( RTMLNode, RTMLNodeId )>, focus : Option<RTMLNodeId> ) -> Self
    {
        Self { root, focus }
    }

    pub fn consume_with_err_if_no_root( self ) -> anyhow::Result<( RTMLNode, RTMLNodeId, Option<RTMLNodeId> )>
    {
        let ( root, root_id ) = self.root.ok_or( anyhow::Error::msg( "No root element" ) )?;

        Ok( ( root, root_id, self.focus ) )
    }

    pub fn consume_focus( self ) -> Option<RTMLNodeId>
    {
        self.focus
    }
}