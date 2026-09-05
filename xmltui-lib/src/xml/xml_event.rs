use roxmltree::Node;

use crate::{rtml::{rtml_command::RTMLCommandOutput, util::rtml_event::{CallbackChangeSrcFromCommand, CallbackReplace, RTMLCallback, RTMLCallbackAction::{self}, RTMLCallbackChangeSrc, RTMLCallbackCommand, RTMLEvent}}, xml::{attrs::attr_comands_from_str, xml_command::output_from_str, xml_state::parse_args_envs_from_node}};

pub fn parse_event_attrs( node : Node  ) -> anyhow::Result<Vec<RTMLEvent>>
{
    let ret = parse_enter_event( node )?;
    
    Ok( ret )
}

fn parse_enter_event( node : Node ) -> anyhow::Result<Vec<RTMLEvent>>
{
    let callbacks = parse_callback_event_attrs( node, "enter" )?;

    Ok( callbacks.into_iter().map( | c | RTMLEvent::Enter( c ) ).collect() )
}

fn parse_callback_event_attrs( node : Node, prefix : &str ) -> anyhow::Result<Vec<RTMLCallback>>
{
    let mut ret = vec![];

    if let Some( ev ) = node.attribute( prefix ) && ev.trim() != ""
    {
        ret.push( parse_callback_event( node, ev, prefix )? );
    }
    
    if let Some( commands ) = node.attribute( format!( "{prefix}-refresh-command" ).as_str() ) && commands.trim() != ""
    {
        if let Some( callback ) = parse_refresh_commands_event( commands )
        {
            ret.push( callback );
        }
    }

    if let Some( states ) = node.attribute( format!( "{prefix}-refresh-st" ).as_str() ) && states.trim() != ""
    {
        if let Some( callback ) = parse_refresh_states_event( states )
        {
            ret.push( callback );
        }
    }

    if let Some( src ) = node.attribute( format!( "{prefix}-src" ).as_str() ) && src.trim() != ""
    {
        ret.push( parse_change_src( src ) );
    }
    
    Ok( ret )
}

fn parse_callback_event( node : Node, value : &str, prefix : &str ) -> anyhow::Result<RTMLCallback>
{
    let executors = attr_comands_from_str( value );

    let args = parse_args_envs_from_node( node, format!( "{prefix}-args" ).as_str() );
    let envs = parse_args_envs_from_node( node, format!( "{prefix}-envs" ).as_str() );

    Ok(
        RTMLCallback::Command(
            RTMLCallbackCommand::new( executors, args, envs ),
            parse_callback_action( node, prefix )?
        )
        
    )
}

fn parse_change_src( src : &str )-> RTMLCallback
{
    RTMLCallback::ChangeSrc(
        RTMLCallbackChangeSrc::new(
            src.to_string()
        )
    )
}

fn parse_refresh_commands_event( commands : &str ) -> Option<RTMLCallback>
{
    let commands = commands
    .split( ", " )
    .filter( | s | s.trim() != "" )
    .map( | s | s.trim().to_string() )
    .collect::<Vec<_>>();

    if commands.len() > 0
    {
        Some( RTMLCallback::RefreshCommand( commands ) )
    }
    else
    {
        None
    }
}

fn parse_refresh_states_event( states : &str ) -> Option<RTMLCallback>
{
    let commands = states
    .split( ", " )
    .filter( | s | s.trim() != "" )
    .map( | s | s.trim().to_string() )
    .collect::<Vec<_>>();

    if commands.len() > 0
    {
        Some( RTMLCallback::RefreshState( commands ) )
    }
    else
    {
        None
    }
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
    else if let Some( url ) = node.attribute( format!( "{prefix}-command-src" ).as_str() )
    {
        Ok( RTMLCallbackAction::ChangeSrc( callback_change_src( node, url, prefix ) ) )
    }
    else
    {
        Ok( RTMLCallbackAction::None )    
    }
}

fn callback_change_src( node : Node, url : &str, prefix : &str ) -> CallbackChangeSrcFromCommand
{
    let url = if url.trim() != ""
    {
        Some( url.trim().to_string() )
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

    CallbackChangeSrcFromCommand::new( url, output )
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