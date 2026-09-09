use std::{collections::HashMap, str::FromStr};

use roxmltree::Node;

use crate::{rtml::{rtml_command::RTMLCommandOutput, rtml_node::{RTMLNode, RTMLNodeCommon, RTMLNodeId}, rtml_state::RTMLState}, state::{command_state::CommandState, state_executor::{CommonState, StateExecutor, TypeState}, var_state::VarState}, util::log::log_to_file, xml::{attrs::{attr_commands, attr_option, attr_option_str, default_id, id_retry_if_exists, parse_common_attrs, parse_path}, xml_command::output_from_node, xml_doc::XMLDoc, xml_util::{container_styles, template_from_inner_node}}};


pub fn states_map( node : Node ) -> HashMap<String, StateExecutor>
{
    let mut ret = HashMap::new();

    for child in node.children()
    {
        state_from_node( child, &mut ret );
    }

    ret
}

fn state_from_node( node : Node, states : &mut HashMap<String, StateExecutor> )
{
    match node.tag_name().name()
    {
        "st-var" => state_var_from_node( node, states ),
        "st-command" => state_command_from_node( node, states ),
        _ => {}
    }
}

fn state_var_from_node( node : Node, states : &mut HashMap<String, StateExecutor> )
{
    if let Some( ( common, id ) ) = state_common_from_node( node, None )
    {
        if let Some( v ) = attr_option_str( node, "value" )
        {
            let id = id_from_option( id );

            states.insert(
                id, 
                StateExecutor::Var(
                    VarState::new( common, v.to_string() )
                )
            );
        }
    }
}

fn id_from_option( id : Option<String> ) -> String
{
    id.unwrap_or( default_id() )
}

fn state_command_from_node( node : Node, states : &mut HashMap<String, StateExecutor> )
{
    let output = output_from_node( node );

    if let Some( ( common, id ) ) = state_common_from_node( node, Some( &output ) )
    {
        state_command_from_output_common_and_option_id( node, output, common, id, states );
    }
}

fn state_command_from_output_common_and_option_id(
    node : Node,
    output : RTMLCommandOutput,
    common : CommonState,
    id : Option<String>,
    states : &mut HashMap<String, StateExecutor>
)
{
    let executors = match attr_commands( node, "exec" )
    {
        Ok( e ) => e,
        Err( e ) =>
        {
            log_to_file( &format!( "No se han podido encontrar executors. Err: {e:?}" ) );

            return;
        }
    };

    let on_init = if let Some( i ) = attr_option_str( node, "on-init" ) && i.trim().to_lowercase() == "true"
    {
        true
    }
    else { false };

    if id.is_none() && ! on_init { return };

    let id = id_from_option( id );

    let args = parse_args_envs_from_node( node, "args" );
    let envs = parse_args_envs_from_node( node, "envs" );

    let template = if let Some( t ) = attr_option_str( node, "template" ) && t.trim() != ""
    {
        Some( t.to_string() )
    }
    else
    {
        None    
    };

    states.insert(
        id, 
        StateExecutor::Command(
            CommandState::new( common, executors, output, args, envs, on_init, template )
        )
    );
}

pub fn parse_args_envs_from_node( node : Node, attr : &str ) -> HashMap<String, String>
{
    if let Some( v ) = attr_option_str( node, attr ) && v.trim() != ""
    {
        parse_args_envs_from_str( v )
    }
    else
    {
        HashMap::new()    
    }
}

pub fn parse_args_envs_from_str( value : &str ) -> HashMap<String, String>
{
    value.split( "," )
    .fold(
        HashMap::new(), 
        | mut acc, val |
        {
            arg_env_value( val, &mut acc );

            acc
        }
    )
}

fn arg_env_value( value : &str, acc : &mut HashMap<String, String> )
{
    if value.trim() == "" { return };

    let mut iter = value.trim().split( ":" );

    let key = iter.next().unwrap().trim();

    let path = if let Some( n ) = iter.next() && n.trim() != ""
    {
        n.trim()
    }
    else
    {
        key    
    };

    let path = parse_path( path );

    acc.insert( 
        key.to_string(), 
        path
    );
}

fn state_common_from_node( node : Node, output : Option<&RTMLCommandOutput> ) -> Option<( CommonState, Option<String> )>
{
    let path = parse_path( attr_option_str( node, "path" )? );
    let stype = attr_option_str( node, "type" );
    
    let stype = if let Some( t ) = stype
    {
        type_from_str( &t, output )
    }
    else
    {
        type_from_output( output )
    };

    let id = if let Some( id ) = attr_option_str( node, "id" ) && id.trim() != ""
    {
        Some( id.to_string() )
    }
    else
    {
        None    
    };

    Some( ( CommonState::new( stype, path ), id ) )
}

fn type_from_output( output : Option<&RTMLCommandOutput> ) -> TypeState
{
    match output
    {
        Some( o ) =>
        {
            TypeState::from_rtml_command_output( o )
        },
        None => TypeState::String
    }
}

fn type_from_str( str : &str, output : Option<&RTMLCommandOutput> ) -> TypeState
{
    match TypeState::from_str( &str )
    {
        Ok( t ) => t,
        Err( _ ) =>
        {
            if let Some( o ) = output
            {
                TypeState::from_rtml_command_output( o )
            }
            else
            {
                TypeState::String
            }
        }
    }
}

pub fn process_state( 
    xml_doc : &mut XMLDoc,
    node : Node, 
    parent_id : Option<RTMLNodeId>, 
    xml : &str
) -> anyhow::Result<( RTMLNode, RTMLNodeId )>
{
    let command_id = id_retry_if_exists( node, xml_doc.nodos() );

    let ( constraint, style, style_template, container_attrs ) = container_styles( node, xml_doc.styles(), None );

    let common = RTMLNodeCommon::new( 
        parse_common_attrs( constraint )?, 
        vec![], 
        parent_id
    );

    let ( reload_with_state, reload_with_state_path ) = reload_with_state( node );
    
    Ok(
        (
            RTMLNode::State(
                RTMLState::new(
                    common,
                    container_attrs,
                    Some( style ),
                    style_template,
                    attr_option( node, "template" ),
                    template_from_inner_node( node, xml ),
                    reload_with_state,
                    reload_with_state_path,
                )
            ),
            command_id
        )
    )
}

pub fn process_state_from_parent(
    xml_doc : &mut XMLDoc,
    parent_node : Node, 
    parent_id : Option<&RTMLNodeId>, 
    xml : &str
) -> anyhow::Result<()>
{
    for child in parent_node.children()
    {
        if child.tag_name().name() != "st" { continue; };

        let ( node, id ) = process_state( xml_doc, child, parent_id.cloned(), xml )?;

        xml_doc.add_node( node, id );

        break;
    }

    Ok( () )
}

pub fn reload_with_state( node : Node ) -> ( bool, Vec<String> )
{
    if let Some( a ) = node.attribute( "reload-with-st-path" ) && a.trim() != ""
    {
        let paths = a.split( "," )
        .filter_map(
            | s |
            {
                if s.trim() != ""
                {
                    Some( parse_path( s ) )
                }
                else
                {
                    None    
                }
            }
        )
        .collect();

        ( 
            false, 
            paths
        )
    }
    else if let Some( a ) = node.attribute( "reload-with-st" ) && a.trim().to_lowercase() == "true"
    {
        ( true, vec![] )
    }
    else { ( false, vec![] ) }
}