use roxmltree::Node;

use crate::{rtml::{rtml_input::RTMLInput, rtml_node::{RTMLNode, RTMLNodeCommon, RTMLNodeId}, util::editable_value::EditableValue}, xml::{attrs::{attr_alignment, attr_value, id_retry_if_exists, parse_common_attrs}, styles::{default_styles::{default_focus_style, default_normal_style}, xml_style::{StyleVariant, style_from_node}}, xml_doc::{XMLDoc, replace_xml_doc_focus}, xml_event::parse_event_attrs}};

pub fn process_input( 
    xml_doc : &mut XMLDoc,
    node : Node, 
    parent_id : Option<RTMLNodeId>
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    let id = id_retry_if_exists( node, xml_doc.nodos() );
    
    let alignment = attr_alignment( node )?;

    let value = parse_input_value( node )?;

    replace_xml_doc_focus( xml_doc, node, &id );

    Ok(
        (
            RTMLNode::Input(
                RTMLInput::new( 
                    alignment, 
                    parse_event_attrs( node, &id )?,
                    value, 
                    style_from_node( node, xml_doc.styles(), default_normal_style(), None ), 
                    style_from_node( node, xml_doc.styles(), default_focus_style(), Some( StyleVariant::Focus ) ), 
                    RTMLNodeCommon::new( 
                        parse_common_attrs( node )?, 
                        vec![], 
                        parent_id
                    )
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