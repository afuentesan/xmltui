
use std::{collections::HashMap, time::Duration};

use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

use crate::{app::event::{AppEvent, CallbackResponse, HidrateCommand, ShowMessage, send_app_event}, code::executor::{Executor, ExecutorOutput, execute_commands}, rtml::{rtml_command::CommandRefresh, rtml_toast::ToastParams, util::rtml_event::RTMLCallbackAction}, util::log::log_to_file};

pub enum ExecutorEventType
{
    CommandChild,
    Callback( RTMLCallbackAction )
}

#[derive(Debug)]
pub enum CommandExecutorType
{
    Command( String ),
    State( String )
}

pub struct CommandExecutorParams
{
    pub doc_id : String,
    pub node_id : CommandExecutorType,
    pub args : HashMap<String, String>,
    pub envs : HashMap<String, String>,
    refresh : CommandRefresh,
    executors : Vec<Executor>,
    event_type : ExecutorEventType,
    global_cancellation_token : Option<CancellationToken>,
    local_cancellation_token : Option<CancellationToken>,
    pub exec : bool,
    pub success_msg : Option<ToastParams>,
    pub err_msg : Option<ToastParams>
}

impl CommandExecutorParams
{
    pub fn new(
        doc_id : String,
        node_id : CommandExecutorType,
        args : HashMap<String, String>,
        envs : HashMap<String, String>,
        refresh : CommandRefresh,
        executors : Vec<Executor>,
        event_type : ExecutorEventType,
        global_cancellation_token : Option<CancellationToken>,
        local_cancellation_token : Option<CancellationToken>,
        exec : bool,
        success_msg : Option<ToastParams>,
        err_msg : Option<ToastParams>
    ) -> Self
    {
        Self { doc_id, node_id, args, envs, refresh, executors, event_type, global_cancellation_token, local_cancellation_token, exec, success_msg, err_msg }
    }

    pub fn node_id( &self ) -> &str
    {
        match &self.node_id
        {
            CommandExecutorType::Command( n ) |
            CommandExecutorType::State( n ) => n
        }
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
            if params.exec
            {
                execute_once( 
                    &params.doc_id, 
                    params.node_id(), 
                    &params.args, 
                    &params.envs, 
                    &params.executors, 
                    &params.event_type,
                    params.success_msg.as_ref(),
                    params.err_msg.as_ref()
                ).await;
            }
        }
    }
}

async fn new_repeat_command_executor( 
    params : CommandExecutorParams,
    duration : Duration
)
{
    if params.exec
    {
        execute_once( 
            &params.doc_id, 
            params.node_id(), 
            &params.args, 
            &params.envs, 
            &params.executors, 
            &params.event_type,
            params.success_msg.as_ref(),
            params.err_msg.as_ref()
        ).await;
    }

    if let Some( g ) = params.global_cancellation_token.as_ref() &&
        let Some( l ) = params.local_cancellation_token.as_ref()
    {
        tokio::select! {
            _ = sleep( duration ) => {}
            _ = g.cancelled() => { return ; }
            _ = l.cancelled() => { return; }
        }
    }
    else if let Some( g ) = params.global_cancellation_token.as_ref()
    {
        tokio::select! {
            _ = sleep( duration ) => {}
            _ = g.cancelled() => { return; }
        }
    }
    else if let Some( l ) = params.local_cancellation_token.as_ref()
    {
        tokio::select! {
            _ = sleep( duration ) => {}
            _ = l.cancelled() => { return; }
        }
    }
    else
    {
        sleep( duration ).await
    }
        
    send_app_event( AppEvent::RefreshCommand( params ) );
}

async fn execute_once( 
    doc_id : &str,
    node_id : &str,
    args : &HashMap<String, String>,
    envs : &HashMap<String, String>,
    executors : &Vec<Executor>,
    event_type : &ExecutorEventType,
    msg_success : Option<&ToastParams>,
    msg_err : Option<&ToastParams>
)
{
    match execute_commands( executors, args, envs ).await
    {
        Ok( output ) =>
        {
            send_command_output( doc_id, node_id, event_type, output, msg_success, msg_err );
        },
        Err( e ) =>
        {
            send_message( doc_id, msg_err, format!( "Error: {e:?}" ) );

            log_to_file( &format!( "execute_once. Se ha producido un error al ejecutar el comando. Error: {:?}", e ) );
        }
    }
}

fn send_message( doc_id : &str, msg : Option<&ToastParams>, response : String )
{
    if let Some( msg ) = msg
    {
        send_app_event(
            AppEvent::ShowMessage(
                ShowMessage::new(
                    doc_id.to_string(), 
                    msg.clone(), 
                    response
                )
            )
        );
    }
}

fn send_command_output( 
    doc_id : &str, 
    node_id : &str, 
    event_type : &ExecutorEventType, 
    output : ExecutorOutput,
    msg_success : Option<&ToastParams>,
    msg_err : Option<&ToastParams>
)
{
    if output.success()
    {
        let response = output.stdout_str();

        match event_type
        {
            ExecutorEventType::CommandChild =>
            {
                send_app_event(
                    AppEvent::HidrateCommand(
                        HidrateCommand::new( doc_id.to_string(), node_id.to_string(), response.clone() )
                    )
                );
            },
            ExecutorEventType::Callback( action ) =>
            {
                send_app_event(
                    AppEvent::CallbackResponse( 
                        CallbackResponse::new( action.clone(), response.clone() ) 
                    )
                );
            }
        };

        send_message( doc_id, msg_success, response );
        
    }
    else
    {
        send_message( doc_id, msg_err, output.stderr_str() );

        log_to_file( &format!( "Se ha producido un error al ejecutar el comando del nodo {node_id}. Stderr: {}", output.stderr_str() ) );
    }
}