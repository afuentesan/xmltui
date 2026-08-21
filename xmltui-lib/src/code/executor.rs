use std::{collections::HashMap, process::Output};

use tokio::process::Command;

#[derive(Debug, Clone)]
pub struct ExecutorEnvVar
{
    name : String,
    value : String
}

impl ExecutorEnvVar
{
    pub fn new( name : String, value : String ) -> Self
    {
        Self { name, value }
    }
}

#[derive(Debug, Clone)]
pub enum ExecutorEnv
{
    Var( ExecutorEnvVar ),
    Data( String )
}

#[derive(Debug, Clone)]
pub enum ExecutorArg
{
    Text( String ),
    Data( String )
}

#[derive(Debug, Clone)]
pub struct Executor 
{
    command : String,

    args : Vec<ExecutorArg>,
    env_vars : Vec<ExecutorEnv>
}

impl Executor
{
    pub fn new( 
        command : String, 
        args : Vec<ExecutorArg>, 
        env_vars : Vec<ExecutorEnv>
    ) -> anyhow::Result<Self>
    {
        if command.trim() == ""
        {
            return Err( anyhow::Error::msg( "command can't be empty" ) );
        }

        Ok( Self { command, args, env_vars } )
    }
}

pub struct ExecutorOutput
{
    output : Output
}

impl ExecutorOutput
{
    pub fn new( output : Output ) -> Self
    {
        Self { output }
    }

    pub fn stdout_str( &self ) -> anyhow::Result<String>
    {
        Ok( String::from_utf8( self.output.stdout.clone() )? )
    }

    // De momento no lo utilizo así que lo comento
    // pub fn stderr_str( &self ) -> anyhow::Result<String>
    // {
    //     Ok( String::from_utf8( self.output.stderr.clone() )? )
    // }

    pub fn success( &self ) -> bool
    {
        self.output.status.success()
    }

    // De momento no lo utilizo así que lo comento
    // pub fn code( &self ) -> Option<i32>
    // {
    //     self.output.status.code()
    // }
}

pub struct ExecutorBuilder
{
    command : String,

    args : Vec<ExecutorArg>,
    env_vars : Vec<ExecutorEnv>
}

impl ExecutorBuilder
{
    pub fn new() -> Self
    {
        Self { command : "".to_string(), args : vec![], env_vars : vec![] }
    }

    pub fn command( mut self, command : String ) -> Self
    {
        self.command = command;

        self
    }

    pub fn arg( mut self, arg : ExecutorArg ) -> Self
    {
        self.args.push( arg );

        self
    }

    pub fn env( mut self, env : ExecutorEnv ) -> Self
    {
        self.env_vars.push( env );

        self
    }

    pub fn build( self ) -> anyhow::Result<Executor>
    {
        Executor::new( self.command, self.args, self.env_vars )
    }

}

pub async fn execute_command( 
    executor: &Executor,
    node_data : &HashMap<String, String>
) -> anyhow::Result<ExecutorOutput> 
{
    let mut command = Command::new( &executor.command );

    command.kill_on_drop( true );

    executor.args.iter().for_each(
        | arg |
        {
            match arg
            {
                ExecutorArg::Data( key ) =>
                {
                    if let Some( val ) = node_data.get( key )
                    {
                        command.arg( val );
                    }
                },
                ExecutorArg::Text( str ) =>
                {
                    command.arg( str );
                }
            }
        }
    );

    executor.env_vars.iter().for_each(
        | env |
        {
            match env
            {
                ExecutorEnv::Var( v ) =>
                {
                    command.env( &v.name, &v.value );
                },
                ExecutorEnv::Data( key ) =>
                {
                    if let Some( val ) = node_data.get( key )
                    {
                        command.env( key, val );
                    }
                }
            }
            
        }
    );

    let output = command.output().await?;

    Ok( ExecutorOutput::new( output ) )
}