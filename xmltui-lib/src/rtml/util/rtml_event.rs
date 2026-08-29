use crate::rtml::{rtml_command::RTMLCommandOutput, rtml_node::RTMLNodeId};


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
    pub value_from : Vec<String>
}

impl RTMLCallbackCommand
{
    pub fn new( name : Vec<String>, data_from : Vec<String>, value_from : Vec<String> ) -> Self
    {
        Self { name, data_from, value_from }
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
pub enum RTMLCallbackAction
{
    ReplaceNode( CallbackReplace ),
    ReplaceChilds( CallbackReplace ),
    ChangeValue( RTMLNodeId ),
    ChangeSrc( CallbackChangeSrcFromCommand ),
    None
}

#[derive(Debug, Clone)]
pub enum RTMLCallback
{
    Command( RTMLCallbackCommand, RTMLCallbackAction ),
    RefreshCommand( Vec<String> ),
    ChangeSrc( RTMLCallbackChangeSrc )
}