use std::collections::HashMap;

use crate::{rtml::{rtml_command::RTMLCommandOutput, rtml_node::RTMLNodeId}, state::state_executor::TypeState};


#[derive(Debug)]
pub enum RTMLEvent 
{
    Enter( RTMLCallback )
}

#[derive(Debug, Clone)]
pub struct RTMLCallbackCommand
{
    pub name : Vec<String>,
    pub data_from : Vec<String>,
    pub value_from : Vec<String>,
    pub args : HashMap<String, String>,
    pub envs : HashMap<String, String>
}

impl RTMLCallbackCommand
{
    pub fn new( 
        name : Vec<String>, 
        data_from : Vec<String>, 
        value_from : Vec<String>, 
        args : HashMap<String, String>, 
        envs : HashMap<String, String> 
    ) -> Self
    {
        Self { name, data_from, value_from, args, envs }
    }
}

#[derive(Debug, Clone)]
pub struct CallbackReplace
{
    pub node_id : RTMLNodeId,
    pub template : Option<String>,
    pub output : RTMLCommandOutput
}

impl CallbackReplace
{
    pub fn new( node_id : RTMLNodeId, template : Option<String>, output : RTMLCommandOutput ) -> Self
    {
        Self { node_id, template, output }
    }
}

#[derive(Debug, Clone)]
pub struct CallbackChangeSrcFromCommand
{
    pub url : Option<String>,
    pub output : RTMLCommandOutput
}

impl CallbackChangeSrcFromCommand
{
    pub fn new( url : Option<String>, output : RTMLCommandOutput ) -> Self
    {
        Self { url, output }
    }
}

#[derive(Debug, Clone)]
pub struct RTMLCallbackChangeSrc 
{
    pub url : String,
    pub data_from : Vec<String>,
    pub value_from : Vec<String>
}

impl RTMLCallbackChangeSrc
{
    pub fn new( url : String, data_from : Vec<String>, value_from : Vec<String> ) -> Self
    {
        Self { url, data_from, value_from }
    }
}

#[derive(Debug, Clone)]
pub struct CallbackChangeState
{
    pub path : String,
    pub stype : TypeState,
    pub template : Option<String>,
    pub output : RTMLCommandOutput
}

impl CallbackChangeState
{
    pub fn new( path : String, stype : TypeState, template : Option<String>, output : RTMLCommandOutput ) -> Self
    {
        Self { path, stype, template, output }
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
    ChangeSrc( RTMLCallbackChangeSrc )
}