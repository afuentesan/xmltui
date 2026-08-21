use crate::rtml::rtml_node::RTMLNodeId;


#[derive(Debug)]
pub enum RTMLEvent 
{
    Enter( RTMLCallback )
}

#[derive(Debug, Clone)]
pub struct RTMLCallbackCommand
{
    pub name : String,
    pub data_from : Vec<String>
}

impl RTMLCallbackCommand
{
    pub fn new( name : String, data_from : Vec<String> ) -> Self
    {
        Self { name, data_from }
    }
}

#[derive(Debug, Clone)]
pub enum RTMLCallbackAction
{
    ReplaceNode( RTMLNodeId ),
    ReplaceChilds( RTMLNodeId ),
    ChangeValue( RTMLNodeId ),
    None
}

#[derive(Debug, Clone)]
pub enum RTMLCallback
{
    Command( RTMLCallbackCommand, RTMLCallbackAction )
}