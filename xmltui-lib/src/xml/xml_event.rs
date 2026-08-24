use roxmltree::Node;

use crate::{rtml::{rtml_command::RTMLCommandOutput, util::rtml_event::{CallbackReplace, RTMLCallback, RTMLCallbackAction, RTMLCallbackCommand, RTMLEvent}}, xml::{attrs::attr_comands_from_str, xml_command::output_from_str}};


pub fn parse_event_attrs( node : Node, id : &str  ) -> anyhow::Result<Vec<RTMLEvent>>
{
    let mut ret = vec![];

    if let Some( enter ) = node.attribute( "enter" ) && enter.trim() != ""
    {
        ret.push( parse_enter_event( node, enter, id )? );
    }
    
    Ok( ret )
}

fn parse_enter_event( node : Node, value : &str, id : &str ) -> anyhow::Result<RTMLEvent>
{
    let executors = attr_comands_from_str( value );

    let mut data_from = vec![];

    if let Some( from ) = node.attribute( "enter-data" )
    {
        from.split( "," ).for_each(
            | s |
            {
                let s = s.trim();

                if s != ""
                {
                    data_from.push( s.to_string() );
                }
            }
        );
    }
    else
    {
        data_from.push( id.to_string() );    
    }

    Ok(
        RTMLEvent::Enter( RTMLCallback::Command(
                RTMLCallbackCommand::new( executors, data_from ),
                parse_callback_action( node, "enter" )?
            )
        )
    )
}

fn parse_callback_action( node : Node, prefix : &str ) -> anyhow::Result<RTMLCallbackAction>
{
    if let Some( parent_id ) = node.attribute( format!( "{prefix}-replace-childs" ).as_str() ) && parent_id.trim() != ""
    {
        Ok( RTMLCallbackAction::ReplaceChilds( callback_replace_from_node( node, parent_id.to_string(), prefix ) ) )
    }
    else if let Some( node_id ) = node.attribute( format!( "{prefix}-replace-node" ).as_str() ) && node_id.trim() != ""
    {
        Ok( RTMLCallbackAction::ReplaceNode( callback_replace_from_node( node, node_id.to_string(), prefix ) ) )
    }
    else if let Some( node_id ) = node.attribute( format!( "{prefix}-change-value" ).as_str() ) && node_id.trim() != ""
    {
        Ok( RTMLCallbackAction::ChangeValue( node_id.to_string() ) )
    }
    else
    {
        Ok( RTMLCallbackAction::None )    
    }
}

fn callback_replace_from_node( node : Node, node_id : String, prefix : &str ) -> CallbackReplace
{
    let template = if let Some( template ) = node.attribute( format!( "{prefix}-template" ).as_str() ) && template.trim() != ""
    {
        Some( template.to_string() )
    }
    else
    {
        None    
    };

    let output = if let Some( output ) = node.attribute( format!( "{prefix}-output" ).as_str() ) && output.trim() != ""
    {
        output_from_str( output )
    }
    else
    {
        RTMLCommandOutput::String
    };

    CallbackReplace::new( node_id, template, output )
}