
use ratatui::{style::Style, widgets::{BorderType, Borders, TitlePosition}};
use roxmltree::Node;

use crate::{rtml::{rtml_border::RTMLBorder, rtml_node::{RTMLNode, RTMLNodeCommon, RTMLNodeId}}, xml::{attrs::{attr_alignment_name, attr_option, container_attrs, id_retry_if_exists, parse_common_attrs}, styles::{default_styles::default_normal_style, xml_style::{StyleVariant, style_from_node}}, xml_doc::XMLDoc, xml2rtml::process_node}};


pub fn process_border(
    xml_doc : &mut XMLDoc, 
    node : Node, 
    // nodos : &mut HashMap<String, RTMLNode>, 
    parent_id : Option<RTMLNodeId>, 
    // styles : &HashMap<StyleSelector, Style>,
    xml : &str
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    let border_id = id_retry_if_exists( node, xml_doc.nodos() );

    let mut childs : Vec<RTMLNodeId> = vec![];

    for c in node.children()
    {
        match process_node( xml_doc, c, Some( border_id.clone() ), xml )?
        {
            Some( ( n, id ) ) =>
            {
                xml_doc.add_node( n, id.clone() );

                childs.push( id );
            },
            None => {}
        }
    }

    Ok(
        (
            RTMLNode::Border(
                RTMLBorder::new(
                    borders( node ), 
                    border_type( node ),
                    attr_option( node, "title" ),
                    title_position( node ),
                    attr_alignment_name( node, "title-align" )?,
                    style_from_node( node, xml_doc.styles(), default_normal_style(), None ),
                    style_from_node( node, xml_doc.styles(), Style::default(), Some( StyleVariant::Title ) ),
                    style_from_node( node, xml_doc.styles(), Style::default(), Some( StyleVariant::Border ) ),
                    container_attrs( node )?,
                    RTMLNodeCommon::new( 
                        parse_common_attrs( node )?, 
                        childs, 
                        parent_id
                    )
                )
            ),
            border_id
        )
    )
}

fn title_position( node : Node ) -> TitlePosition
{
    match node.attribute( "title-position" )
    {
        Some( t ) =>
        {
            match t.trim()
            {
                "" | "top" => TitlePosition::Top,
                "bottom" => TitlePosition::Bottom,
                _ => TitlePosition::Top    
            }
        },
        None => TitlePosition::Top
    }
}

fn borders( node : Node ) -> Borders
{
    match node.attribute( "borders" )
    {
        Some( b ) if b.trim() != "" =>
        {
            let mut borders = Borders::empty();

            for b in b.split( "," )
            {
                match b.trim()
                {
                    "top" => borders = borders.union( Borders::TOP ),
                    "left" => borders = borders.union( Borders::LEFT ),
                    "right" => borders = borders.union( Borders::RIGHT ),
                    "bottom" => borders = borders.union( Borders::BOTTOM ),
                    _ => continue
                }
            }

            borders
        },
        _ => Borders::all()
    }
}

// TODO: Permitir diferentes tipos de borde
fn border_type( _node : Node ) -> BorderType
{
    BorderType::default()
}