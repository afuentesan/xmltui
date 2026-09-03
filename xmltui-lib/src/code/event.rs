
use std::{collections::HashMap, time::Duration};

use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

use crate::{app::event::{AppEvent, CallbackResponse, HidrateCommand, send_app_event}, code::executor::{Executor, ExecutorOutput, execute_commands}, rtml::{rtml_command::CommandRefresh, util::rtml_event::RTMLCallbackAction}, util::log::log_to_file};

pub enum ExecutorEventType
{
    CommandChild,
    Callback( RTMLCallbackAction )
}

pub struct CommandExecutorParams
{
    doc_id : String,
    node_id : String,
    node_data : HashMap<String, String>,
    node_value : HashMap<String, String>,
    args : HashMap<String, String>,
    envs : HashMap<String, String>,
    refresh : CommandRefresh,
    executors : Vec<Executor>,
    event_type : ExecutorEventType,
    global_cancellation_token : Option<CancellationToken>,
    local_cancellation_token : Option<CancellationToken>
}

impl CommandExecutorParams
{
    pub fn new(
        doc_id : String,
        node_id : String,
        node_data : HashMap<String, String>,
        node_value : HashMap<String, String>,
        args : HashMap<String, String>,
        envs : HashMap<String, String>,
        refresh : CommandRefresh,
        executors : Vec<Executor>,
        event_type : ExecutorEventType,
        global_cancellation_token : Option<CancellationToken>,
        local_cancellation_token : Option<CancellationToken>
    ) -> Self
    {
        Self { doc_id, node_id, node_data, node_value, args, envs, refresh, executors, event_type, global_cancellation_token, local_cancellation_token }
    }
}

pub async fn new_command_executor( 
    params : CommandExecutorParams
)
{
    match params.refresh
    {
        CommandRefresh::Repeat( d ) =>
        {
            new_repeat_command_executor( params, d ).await;
        },
        CommandRefresh::Once =>
        {
            execute_once( &params.doc_id, &params.node_id, &params.node_data, &params.node_value, &params.args, &params.envs, &params.executors, &params.event_type ).await;
        }
    }
}

async fn new_repeat_command_executor( 
    params : CommandExecutorParams,
    duration : Duration
)
{
    loop
    {
        execute_once( &params.doc_id, &params.node_id, &params.node_data, &params.node_value, &params.args, &params.envs, &params.executors, &params.event_type ).await;

        if let Some( g ) = params.global_cancellation_token.as_ref() &&
           let Some( l ) = params.local_cancellation_token.as_ref()
        {
            tokio::select! {
                _ = sleep(duration) => {}
                _ = g.cancelled() => { break; }
                _ = l.cancelled() => { break; }
            }
        }
        else if let Some( g ) = params.global_cancellation_token.as_ref()
        {
            tokio::select! {
                _ = sleep(duration) => {}
                _ = g.cancelled() => { break; }
            }
        }
        else if let Some( l ) = params.local_cancellation_token.as_ref()
        {
            tokio::select! {
                _ = sleep(duration) => {}
                _ = l.cancelled() => { break; }
            }
        }
        else
        {
            sleep( duration ).await
        }
        
    }
}

async fn execute_once( 
    doc_id : &str,
    node_id : &str,
    node_data : &HashMap<String, String>,
    node_value : &HashMap<String, String>,
    args : &HashMap<String, String>,
    envs : &HashMap<String, String>,
    executors : &Vec<Executor>,
    event_type : &ExecutorEventType,
)
{
    match execute_commands( executors, node_data, node_value, args, envs ).await
    {
        Ok( output ) =>
        {
            send_command_output( doc_id, node_id, event_type, output );
        },
        Err( e ) =>
        {
            log_to_file( &format!( "execute_once. Se ha producido un error al ejecutar el comando. Error: {:?}", e ) );
        }
    }
}

fn send_command_output( doc_id : &str, node_id : &str, event_type : &ExecutorEventType, output : ExecutorOutput )
{
    if output.success()
    {
        let response = match output.stdout_str()
        {
            Ok( s ) => s,
            Err( e ) =>
            {
                log_to_file( &format!( "Error execute command: {:?}", e ) );
                
                return;
            }    
        };

        match event_type
        {
            ExecutorEventType::CommandChild =>
            {
                send_app_event(
                    AppEvent::HidrateCommand(
                        HidrateCommand::new( doc_id.to_string(), node_id.to_string(), response )
                    )
                );
            },
            ExecutorEventType::Callback( action ) =>
            {
                send_app_event(
                    AppEvent::CallbackResponse( 
                        CallbackResponse::new( action.clone(), response ) 
                    )
                );
            }   
        }
        
    }
    else
    {
        log_to_file( &format!( "Se ha producido un error al ejecutar el comando del nodo {node_id}. Stderr: {}", output.stderr_str().unwrap_or( "".to_string() ) ) );
    }
}