use std::collections::HashMap;

use crate::{rtml::{rtml_command::CommandRefresh, rtml_toast::ToastParams, util::rtml_style::RTMLStyleTemplateType}, state::state_executor::CommonState};


#[derive(Debug)]
pub struct CommandState
{
    pub common : CommonState,
    pub executors : Vec<String>,
    pub args : HashMap<String, String>,
    pub envs : HashMap<String, String>,
    pub on_init : bool,
    pub template : Option<String>,
    pub refresh : CommandRefresh,
    pub exec_if : Option<RTMLStyleTemplateType>,
    pub message_success : Option<ToastParams>,
    pub message_error : Option<ToastParams>
}

impl CommandState
{
    pub fn new( 
        common : CommonState, 
        executors : Vec<String>,
        args : HashMap<String, String>, 
        envs : HashMap<String, String>, 
        on_init : bool,
        template : Option<String>,
        refresh : CommandRefresh,
        exec_if : Option<RTMLStyleTemplateType>,
        message_success : Option<ToastParams>,
        message_error : Option<ToastParams>
    ) -> Self
    {
        Self { common, executors, args, envs, on_init, template, refresh, exec_if, message_success, message_error }
    }
}