use roxmltree::Node;

use crate::{rtml::{rtml_input::RTMLInput, rtml_node::{RTMLNode, RTMLNodeCommon, RTMLNodeId}, util::editable_value::EditableValue}, xml::{attrs::{attr_value, field_attrs_from_node_and_id, id_retry_if_exists, parse_common_attrs}, styles::{default_styles::default_focus_style, xml_style::StyleVariant}, xml_doc::{XMLDoc, replace_xml_doc_focus}, xml_event::parse_event_attrs, xml_util::{input_like_styles, style_from_styles}}};

pub fn process_input( 
    xml_doc : &mut XMLDoc,
    node : Node, 
    parent_id : Option<RTMLNodeId>
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    let id = id_retry_if_exists( node, xml_doc.nodos() );
    
    let ( constraint, style, alignment ) = input_like_styles( node, xml_doc.styles(), None );
    
    let focus_style = style_from_styles( node, xml_doc.styles(), Some( StyleVariant::Focus ), Some( default_focus_style( &style ) ) );

    let common = RTMLNodeCommon::new( 
        parse_common_attrs( node, constraint )?, 
        vec![], 
        parent_id
    );

    let value = parse_input_value( node )?;

    let field = field_attrs_from_node_and_id( node, &id );

    replace_xml_doc_focus( xml_doc, node, &id );

    Ok(
        (
            RTMLNode::Input(
                RTMLInput::new( 
                    alignment, 
                    parse_event_attrs( node, &id )?,
                    value, 
                    style, 
                    focus_style, 
                    common,
                    field
                )
            ),
            id
        )
    )
}

fn parse_input_value( node : Node ) -> anyhow::Result<EditableValue>
{
    let value = attr_value( node );

    Ok( EditableValue::new( value ) )
}