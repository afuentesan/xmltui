use roxmltree::Node;

use crate::{rtml::{rtml_layout::RTMLLayout, rtml_node::{RTMLNode, RTMLNodeCommon, RTMLNodeId}}, xml::{attrs::{id_retry_if_exists, parse_common_attrs_2}, xml_doc::XMLDoc, xml_util::container_styles, xml2rtml::process_node}};

pub fn process_body_layout( 
    xml_doc : &mut XMLDoc,
    node : Node, 
    parent_id : Option<RTMLNodeId>, 
    xml : &str 
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    process_container( xml_doc, node, parent_id, xml )
}

pub fn process_layout( 
    xml_doc : &mut XMLDoc,
    node : Node, 
    parent_id : Option<RTMLNodeId>, 
    xml : &str 
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    process_container( xml_doc, node, parent_id, xml )
}

fn process_container( 
    xml_doc : &mut XMLDoc,
    node : Node, 
    parent_id : Option<RTMLNodeId>, 
    xml : &str
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    let layout_id = id_retry_if_exists( node, xml_doc.nodos() );

    let mut childs : Vec<RTMLNodeId> = vec![];

    for c in node.children()
    {
        match process_node( xml_doc, c, Some( layout_id.clone() ), xml )?
        {
            Some( ( n, id ) ) =>
            {
                xml_doc.add_node( n, id.clone() );

                childs.push( id );
            },
            None => {}
        }
    }

    let ( constraint, style, container_attrs ) = container_styles( node, xml_doc.styles_2(), None );

    Ok(
        (
            RTMLNode::Layout(
                RTMLLayout::new( 
                    RTMLNodeCommon::new( 
                        parse_common_attrs_2( node, constraint )?, 
                        childs, 
                        parent_id
                    ),
                    container_attrs,
                    Some( style )
                )
            ),
            layout_id
        )
    )
}

// fn container_styles( node : Node, styles : &HashMap<StyleSelector, XMLStyle>, variant : Option<StyleVariant> ) -> ( Constraint, Style, ContainerAttrs )
// {
//     let styles = style_from_node_2( node, styles, variant );

//     let constraint = if let Some( c ) = styles.constraint.0
//     {
//         c
//     }
//     else
//     {
//         Constraint::default()    
//     };

//     let style = if let Some( s ) = styles.style.0
//     {
//         s
//     }
//     else
//     {
//         Style::default()    
//     };

//     let direction = if let Some( d ) = styles.direction
//     {
//         d
//     }
//     else
//     {
//         Direction::default()    
//     };

//     let flex = if let Some( f ) = styles.flex
//     {
//         f
//     }
//     else
//     {
//         Flex::default()    
//     };

//     let horizontal = if let Some( h ) = styles.inner_padding.0.0
//     {
//         h
//     }
//     else
//     {
//         HorizontalPadding::default()    
//     };

//     let vertical = if let Some( v ) = styles.inner_padding.0.1
//     {
//         v
//     }
//     else
//     {
//         VerticalPadding::default()    
//     };

//     let padding = RTMLPadding::new_parts( horizontal, vertical );

//     let container_attrs = ContainerAttrs::new( direction, flex, padding );

//     ( constraint, style, container_attrs )
// }

