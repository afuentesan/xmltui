use tokio_util::sync::CancellationToken;

use crate::{app::event::CallbackResponse, async_app::async_app::spawn_async_task, code::event::{CommandExecutorParams, ExecutorEventType, new_command_executor}, rtml::{rtml_command::CommandRefresh, rtml_doc::RTMLDoc, util::rtml_event::{RTMLCallback, RTMLCallbackAction, RTMLCallbackCommand}}, util::log::log_to_file, xml::xml2rtml::{replace_node_childs_with_xml, replace_node_with_xml}};


pub fn execute_callback(
    doc : &RTMLDoc,
    callback : RTMLCallback
)
{
    match callback
    {
        RTMLCallback::Command( command, action ) =>
        {
            execute_callback_command(
                doc,
                command,
                action
            )
        }
    }
}

fn execute_callback_command(
    doc : &RTMLDoc,
    command : RTMLCallbackCommand,
    action : RTMLCallbackAction
)
{
    match doc.executors.get( &command.name )
    {
        Some( executor ) =>
        {
            let doc_id = doc.doc_id.clone();
            let node_id = doc.root_id.clone();
            let node_data = doc.data_from_nodes_id( command.data_from.as_ref() );
            let executor = executor.clone();

            let params = CommandExecutorParams::new(
                doc_id, 
                node_id, 
                node_data, 
                CommandRefresh::Once, 
                vec![ executor ], 
                ExecutorEventType::Callback( action ), 
                None, 
                None
            );

            spawn_async_task(
                async move 
                {
                    new_command_executor( params ).await
                }
            );
        },
        None =>
        {
            log_to_file( &format!( "No se encontro un executor. Command: {:?}", command ) );
        }
    }
    
}

pub fn execute_callback_response(
    doc : &mut RTMLDoc,
    response : CallbackResponse,
    cancellation_token : CancellationToken
) -> bool
{
    match response.callback_action
    {
        RTMLCallbackAction::None => false,
        RTMLCallbackAction::ReplaceChilds( parent_id ) =>
        {
            match replace_node_childs_with_xml(
                doc, 
                parent_id.clone(), 
                &response.response
            )
            {
                Ok( _ ) =>
                {
                    doc.init_commands_for_childs( cancellation_token, &parent_id );

                    true
                },
                Err( e ) =>
                {
                    log_to_file( &format!( "Error en RTMLCallbackAction::ReplaceChilds. Error: {:?}", e ) );
                    
                    false
                }
            }
        },
        RTMLCallbackAction::ReplaceNode( node_id ) =>
        {
            if doc.root_id == node_id
            {
                log_to_file( "No se puede reemplazar el nodo root" );

                return false;
            }

            match replace_node_with_xml(
                doc, 
                node_id, 
                response.response
            )
            {
                Ok( new_node_id ) =>
                {
                    doc.init_commands_for_node_and_childs( cancellation_token, &new_node_id );

                    true
                },
                Err( e ) =>
                {
                    log_to_file( &format!( "Error en RTMLCallbackAction::ReplaceNode. Error: {:?}", e ) );
                    
                    false
                }
            }
        },
        RTMLCallbackAction::ChangeValue( node_id ) =>
        {
            doc.replace_node_value( &node_id, response.response )
        }
    }
}