use serde_json::json;
use tokio_util::sync::CancellationToken;

use crate::{app::event::{AppEvent, CallbackResponse, send_app_event}, async_app::async_app::spawn_async_task, code::event::{CommandExecutorParams, ExecutorEventType, new_command_executor}, rtml::{rtml_command::{CommandRefresh, RTMLCommandOutput}, rtml_doc::RTMLDoc, util::rtml_event::{CallbackChangeSrcFromCommand, CallbackChangeState, CallbackReplace, RTMLCallback, RTMLCallbackAction, RTMLCallbackChangeSrc, RTMLCallbackCommand}}, state::{state_executor::CommonState, var_state::{VarState, change_var_state}}, util::{log::log_to_file, template::{template_to_xml, xml_from_template_context}}, xml::xml2rtml::{replace_node_childs_with_xml, replace_node_with_xml}};


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
            );
        },
        RTMLCallback::RefreshCommand( commands ) =>
        {
            doc.refresh_commands( commands );
        },
        RTMLCallback::ChangeSrc( params ) =>
        {
            change_src( doc, params );
        }
    }
}

fn change_src( doc : &RTMLDoc, params : RTMLCallbackChangeSrc )
{
    let node_data = doc.data_from_nodes_id( params.data_from.as_ref() );
    let node_value = doc.value_from_nodes_id( params.value_from.as_ref() );

    let context = json!( { "data" : node_data, "value" : node_value } );

    match xml_from_template_context( &params.url, context )
    {
        Ok( s ) =>
        {
            send_app_event( AppEvent::LoadFile( s ) );
        },
        Err( e ) =>
        {
            log_to_file( &format!( "change_src. No se ha podido parsear el src. Url: {}, Err: {e:?}", params.url ) );
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
        let args = doc.state_from_key_path( &command.args );
        let envs = doc.state_from_key_path( &command.envs );

        let params = CommandExecutorParams::new(
            doc_id, 
            node_id, 
            node_data, 
            node_value,
            args,
            envs,
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
        },
        RTMLCallbackAction::ChangeSrc( c ) =>
        {
            parse_change_src( c, response.response )
        },
        RTMLCallbackAction::ChangeState( s ) =>
        {
            parse_change_state( doc, s, response.response )
        }
    }
}

fn parse_change_state(
    doc : &mut RTMLDoc,
    change_state : CallbackChangeState,
    response : String
) -> bool
{
    let response = match parse_response_from_template_and_output( 
        change_state.template.as_ref(), 
        change_state.output, 
        response, 
        doc
    )
    {
        Some( s ) => s,
        None =>
        {
            return false;
        }
    };

    let params = VarState::new(
        CommonState::new(
            change_state.stype, 
            change_state.path
        ), 
        response
    );

    change_var_state( &params, &mut doc.state )
}

fn parse_change_src( change_data : CallbackChangeSrcFromCommand, response : String ) -> bool
{
    if let Some( url ) = change_data.url && url.trim() != ""
    {
        match template_to_xml( response.clone(), Some( &url ), change_data.output )
        {
            Ok( f ) =>
            {
                send_app_event( AppEvent::LoadFile( f ) );
            },
            Err( e ) =>
            {
                log_to_file( &format!( "parse_response. Fail to parse template: {:?}", e ) );

                return false;
            }
        }
    }
    
    send_app_event( AppEvent::LoadFile( response ) );

    false
}

fn parse_response( replace_data : &CallbackReplace, response : String, doc : &RTMLDoc ) -> Option<String>
{
    // let response_xml = if let Some( template ) = replace_data.template.as_ref() && doc.templates.contains_key( template )
    // {
    //     match template_to_xml( response, doc.templates.get( template ), replace_data.output )
    //     {
    //         Ok( r ) => r,
    //         Err( e ) =>
    //         {
    //             log_to_file( &format!( "parse_response. Fail to parse template: {:?}", e ) );

    //             return None;
    //         }
    //     }
    // }
    // else
    // {
    //     response    
    // };

    // Some( response_xml )

    parse_response_from_template_and_output( replace_data.template.as_ref(), replace_data.output, response, doc )
}

fn parse_response_from_template_and_output( template : Option<&String>, output : RTMLCommandOutput, response : String, doc : &RTMLDoc ) -> Option<String>
{
    let response_xml = if let Some( template ) = template && doc.templates.contains_key( template )
    {
        match template_to_xml( response, doc.templates.get( template ), output )
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