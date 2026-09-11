use std::collections::HashMap;

use crate::{rtml::{rtml_command::{CommandRefresh, RTMLCommandOutput}, util::rtml_style::RTMLStyleTemplateType}, state::state_executor::CommonState};


#[derive(Debug)]
pub struct CommandState
{
    pub common : CommonState,
    pub executors : Vec<String>,
    pub output : RTMLCommandOutput,
    pub args : HashMap<String, String>,
    pub envs : HashMap<String, String>,
    pub on_init : bool,
    pub template : Option<String>,
    pub refresh : CommandRefresh,
    pub exec_if : Option<RTMLStyleTemplateType>
}

impl CommandState
{
    pub fn new( 
        common : CommonState, 
        executors : Vec<String>,
        output : RTMLCommandOutput, 
        args : HashMap<String, String>, 
        envs : HashMap<String, String>, 
        on_init : bool,
        template : Option<String>,
        refresh : CommandRefresh,
        exec_if : Option<RTMLStyleTemplateType>
    ) -> Self
    {
        Self { common, executors, output, args, envs, on_init, template, refresh, exec_if }
    }
}