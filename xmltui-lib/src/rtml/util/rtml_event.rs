use std::collections::HashMap;

use crate::{rtml::{rtml_node::RTMLNodeId, rtml_toast::ToastParams}, state::state_executor::TypeState};


#[derive(Debug)]
pub enum RTMLEvent 
{
    Enter( RTMLCallback )
}

#[derive(Debug, Clone)]
pub struct RTMLCallbackCommand
{
    pub name : Vec<String>,
    pub args : HashMap<String, String>,
    pub envs : HashMap<String, String>,
    pub message_success : Option<ToastParams>,
    pub message_error : Option<ToastParams>
}

impl RTMLCallbackCommand
{
    pub fn new( 
        name : Vec<String>, 
        args : HashMap<String, String>, 
        envs : HashMap<String, String>,
        message_success : Option<ToastParams>,
        message_error : Option<ToastParams>
    ) -> Self
    {
        Self { name, args, envs, message_success, message_error }
    }
}

#[derive(Debug, Clone)]
pub struct CallbackReplace
{
    pub node_id : RTMLNodeId,
    pub template : Option<String>,
    pub output : TypeState
}

impl CallbackReplace
{
    pub fn new( node_id : RTMLNodeId, template : Option<String>, output : TypeState ) -> Self
    {
        Self { node_id, template, output }
    }
}

#[derive(Debug, Clone)]
pub struct CallbackChangeSrcFromCommand
{
    pub url : Option<String>,
    pub output : TypeState
}

impl CallbackChangeSrcFromCommand
{
    pub fn new( url : Option<String>, output : TypeState ) -> Self
    {
        Self { url, output }
    }
}

#[derive(Debug, Clone)]
pub struct RTMLCallbackChangeSrc 
{
    pub url : String
}

impl RTMLCallbackChangeSrc
{
    pub fn new( url : String ) -> Self
    {
        Self { url }
    }
}

#[derive(Debug, Clone)]
pub struct CallbackChangeState
{
    pub path : String,
    pub stype : TypeState,
    pub template : Option<String>
}

impl CallbackChangeState
{
    pub fn new( path : String, stype : TypeState, template : Option<String> ) -> Self
    {
        Self { path, stype, template }
    }
}

#[derive(Debug, Clone)]
pub enum RTMLCallbackAction
{
    ReplaceNode( CallbackReplace ),
    ReplaceChilds( CallbackReplace ),
    ChangeValue( RTMLNodeId ),
    ChangeSrc( CallbackChangeSrcFromCommand ),
    ChangeState( CallbackChangeState ),
    None
}

#[derive(Debug, Clone)]
pub enum RTMLCallback
{
    Command( RTMLCallbackCommand, RTMLCallbackAction ),
    RefreshCommand( Vec<String> ),
    ChangeSrc( RTMLCallbackChangeSrc ),
    RefreshState( Vec<String> )
}