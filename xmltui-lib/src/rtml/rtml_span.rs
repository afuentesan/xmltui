use ratatui::{style::Style, text::Span};

use crate::rtml::{rtml_doc::RTMLDoc, rtml_node::{RTMLNode, RTMLNodeCommon}};


#[derive(Debug)]
pub struct RTMLSpan 
{
    pub common : RTMLNodeCommon,
    pub text : String,
    pub style : Style
}

impl RTMLSpan
{
    pub fn new( text : String, common : RTMLNodeCommon, style : Style ) -> Self
    {
        Self { text, common, style }
    }

    pub fn replace_value( &mut self, new_value : String ) -> bool
    {
        self.text = new_value;

        true
    }
}

pub fn spans_from_childs<'a>(
    childs : &Vec<String>,
    doc : &'a RTMLDoc
) -> anyhow::Result<Vec<Span<'a>>>
{
    let mut spans : Vec<Span> = vec![];

    for child in childs
    {
        let child_node = doc.node_ref_by_id( &child ).ok_or(
            anyhow::Error::msg( format!( "No se encontró el nodo con id {child} en spans_from_childs" ) )
        )?;

        match &child_node
        {
            RTMLNode::Span( s ) =>
            {
                if s.text.len() > 0
                {
                    let span = Span::from( s.text.as_str() ).style( s.style );

                    spans.push( span );
                }
            },
            RTMLNode::Command( c ) =>
            {
                let mut command_spans = spans_from_childs( &c.common.childs, doc )?;

                spans.append( &mut command_spans );
            },
            _ => return Err( anyhow::Error::msg( "Se esperaba un Span o un Command" ) )
        }
    }
    
    if spans.len() == 0
    {
        spans.push( Span::from( " " ) );
    }
    
    Ok( spans )
}