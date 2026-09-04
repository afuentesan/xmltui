
use ratatui::widgets::{BorderType, Borders, TitlePosition};
use roxmltree::Node;

use crate::{rtml::{rtml_border::RTMLBorder, rtml_node::{RTMLNode, RTMLNodeCommon, RTMLNodeId}}, xml::{attrs::{attr_alignment_name, attr_option, id_retry_if_exists, parse_common_attrs}, styles::xml_style::StyleVariant, xml_doc::XMLDoc, xml_util::{container_styles, style_from_styles}, xml2rtml::process_node}};


pub fn process_border(
    xml_doc : &mut XMLDoc, 
    node : Node,  
    parent_id : Option<RTMLNodeId>, 
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

    let ( constraint, style, style_template, container_attrs ) = container_styles( node, xml_doc.styles(), None );

    let ( title_style, title_style_template ) = style_from_styles( node, xml_doc.styles(), Some( StyleVariant::Title ), None );
    let ( border_style, border_style_template ) = style_from_styles( node, xml_doc.styles(), Some( StyleVariant::Border ), None );
    
    Ok(
        (
            RTMLNode::Border(
                RTMLBorder::new(
                    borders( node ), 
                    border_type( node ),
                    attr_option( node, "title" ),
                    title_position( node ),
                    attr_alignment_name( node, "title-align" )?,
                    style,
                    style_template,
                    title_style,
                    border_style,
                    container_attrs,
                    RTMLNodeCommon::new( 
                        parse_common_attrs( constraint )?, 
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