use std::time::Duration;

use roxmltree::Node;

use crate::{rtml::{rtml_command::{CommandRefresh, RTMLCommand, RTMLCommandOutput}, rtml_node::{RTMLNode, RTMLNodeCommon, RTMLNodeId, XMLNodeWrapper}}, xml::{attrs::{attr_commands, attr_option, attr_result, container_attrs, id_retry_if_exists, parse_common_attrs}, styles::{default_styles::default_normal_style, xml_style::style_from_container}, xml_doc::XMLDoc, xml_event::nodes_from_attr, xml_util::template_from_inner_node}};

pub fn process_command( 
    xml_doc : &mut XMLDoc,
    node : Node, 
    parent_id : Option<RTMLNodeId>, 
    xml : &str
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    let command_id = id_retry_if_exists( node, xml_doc.nodos() );

    let executors = attr_commands( node, "exec" )?;

    Ok(
        (
            RTMLNode::Command(
                RTMLCommand::new(
                    executors, 
                    refresh_from_node( node ),
                    RTMLNodeCommon::new( 
                        parse_common_attrs( node )?, 
                        vec![], 
                        parent_id
                    ),
                    container_attrs( node )?,
                    style_from_container( node, xml_doc.styles(), default_normal_style() ),
                    wrapper_from_node( node ),
                    attr_option( node, "template" ),
                    template_from_inner_node( node, xml ),
                    output_from_node( node ),
                    nodes_from_attr( node, &command_id, "cdata" ),
                    nodes_from_attr( node, &command_id, "cvalue" )
                )
            ),
            command_id
        )
    )
}

fn output_from_node( node : Node ) -> RTMLCommandOutput
{
    match node.attribute( "output" )
    {
        Some( a ) => output_from_str( a ),
        None => RTMLCommandOutput::String
    }
}

pub fn output_from_str( str : &str ) -> RTMLCommandOutput
{
    match str
    {
        "strvec" => RTMLCommandOutput::StrVec,
        "json" => RTMLCommandOutput::Json,
        _ => RTMLCommandOutput::String
    }
}

fn wrapper_from_node( node : Node ) -> Option<XMLNodeWrapper>
{
    match attr_result( node, "child-type" )
    {
        Ok( v ) if v.trim() != "" =>
        {
            let mut prefix = format!( "<{}", v.trim() );

            prefix.push_str( &child_attrs( node ) );

            prefix.push( '>' );

            let suffix = format!( "</{}>", v.trim() );
            
            Some( XMLNodeWrapper::new( prefix, suffix ) )
        },
        _ => None    
    }
}

fn child_attrs( node : Node ) -> String
{
    node.attributes().fold(
        String::new(),
        | mut acc, attr |
        {
            let name = attr.name().trim();

            if name.starts_with( "child-" ) && name.len() > 6
            {
                let val = attr.value();

                let name = &name[6..];

                acc = format!( r#"{} {}="{}""#, acc, name, val );
            }

            acc
        }
    )
}

fn refresh_from_node( node : Node ) -> CommandRefresh
{
    if let Some( sec ) = node.attribute( "refresh-sec" ) && sec.trim() != ""
    {
        match sec.parse::<u64>()
        {
            Ok( s ) => CommandRefresh::Repeat( Duration::from_secs( s ) ),
            Err( _ ) => CommandRefresh::Once
        }
    }
    else if let Some( ms ) = node.attribute( "refresh-ms" ) && ms.trim() != ""
    {
        match ms.parse::<u64>()
        {
            Ok( s ) => CommandRefresh::Repeat( Duration::from_millis( s ) ),
            Err( _ ) => CommandRefresh::Once
        }
    }
    else
    {
        CommandRefresh::Once    
    }
}