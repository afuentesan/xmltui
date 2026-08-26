use tokio_util::sync::CancellationToken;

use crate::{app::event::CallbackResponse, async_app::async_app::spawn_async_task, code::event::{CommandExecutorParams, ExecutorEventType, new_command_executor}, rtml::{rtml_command::CommandRefresh, rtml_doc::RTMLDoc, util::rtml_event::{CallbackReplace, RTMLCallback, RTMLCallbackAction, RTMLCallbackCommand}}, util::{log::log_to_file, template::template_to_xml}, xml::xml2rtml::{replace_node_childs_with_xml, replace_node_with_xml}};


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
    if let Some( executors ) = doc.executors_from_ids( &command.name )
    {
        let doc_id = doc.doc_id.clone();
        let node_id = doc.root_id.clone();
        let node_data = doc.data_from_nodes_id( command.data_from.as_ref() );
        let node_value = doc.value_from_nodes_id( command.value_from.as_ref() );

        let params = CommandExecutorParams::new(
            doc_id, 
            node_id, 
            node_data, 
            node_value,
            CommandRefresh::Once, 
            executors, 
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
    }
    else
    {
        log_to_file( &format!( "No se encontro un executor. Command: {:?}", command ) );    
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
        RTMLCallbackAction::ReplaceChilds( replace_data ) =>
        {
            let response_xml = parse_response( &replace_data, response.response, doc );

            if response_xml.is_none() { return false };

            let response_xml = response_xml.unwrap();

            match replace_node_childs_with_xml(
                doc, 
                replace_data.node_id.clone(), 
                &response_xml
            )
            {
                Ok( _ ) =>
                {
                    doc.init_commands_for_childs( cancellation_token, &replace_data.node_id );

                    true
                },
                Err( e ) =>
                {
                    log_to_file( &format!( "Error en RTMLCallbackAction::ReplaceChilds. Error: {:?}", e ) );
                    
                    false
                }
            }
        },
        RTMLCallbackAction::ReplaceNode( replace_data ) =>
        {
            if doc.root_id == replace_data.node_id
            {
                log_to_file( "No se puede reemplazar el nodo root" );

                return false;
            }

            log_to_file( &format!( "Templates: {:?}", doc.templates ) );

            let response_xml = parse_response( &replace_data, response.response, doc );

            if response_xml.is_none() { return false };

            match replace_node_with_xml(
                doc, 
                replace_data.node_id, 
                response_xml.unwrap()
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

fn parse_response( replace_data : &CallbackReplace, response : String, doc : &RTMLDoc ) -> Option<String>
{
    let response_xml = if let Some( template ) = replace_data.template.as_ref() && doc.templates.contains_key( template )
    {
        match template_to_xml( response, doc.templates.get( template ), replace_data.output )
        {
            Ok( r ) => r,
            Err( e ) =>
            {
                log_to_file( &format!( "parse_response. Fail to parse template: {:?}", e ) );

                return None;
            }
        }
    }
    else
    {
        response    
    };

    Some( response_xml )
}