use std::collections::HashMap;

use serde_json::Value;

use crate::{code::executor::{ExecutorArg, ExecutorEnv}, rtml::rtml_command::RTMLCommandOutput, state::state_executor::CommonState};


#[derive(Debug)]
pub struct CommandState
{
    pub common : CommonState,
    pub executors : Vec<String>,
    pub output : RTMLCommandOutput,
    pub args : HashMap<String, String>,
    pub envs : HashMap<String, String>,
    pub on_init : bool,
    pub template : Option<String>
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
        template : Option<String>
    ) -> Self
    {
        Self { common, executors, output, args, envs, on_init, template }
    }
}

fn args_values( map : &HashMap<String, String>, state : &Value ) -> Vec<ExecutorArg>
{
    let mut ret = vec![];

    ret
}

fn envs_values( map : &HashMap<String, String>, state : &Value ) -> Vec<ExecutorEnv>
{
    let mut ret = vec![];

    ret
}