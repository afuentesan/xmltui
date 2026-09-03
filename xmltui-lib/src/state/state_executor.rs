use std::str::FromStr;

use serde_json::{Number, Value};

use crate::{rtml::rtml_command::RTMLCommandOutput, state::{command_state::CommandState, var_state::VarState}};

#[derive(Debug)]
pub struct CommonState
{
    pub stype : TypeState,
    pub path : String
}

impl CommonState
{
    pub fn new( stype : TypeState, path : String ) -> Self
    {
        Self { stype, path }
    }
}

#[derive(Debug, Clone)]
pub enum TypeState
{
    String,
    Number,
    Bool,
    Json
}

impl TypeState
{
    pub fn from_rtml_command_output( output : &RTMLCommandOutput ) -> TypeState
    {
        match output
        {
            RTMLCommandOutput::Json |
            RTMLCommandOutput::StrVec => TypeState::Json,
            RTMLCommandOutput::String => TypeState::String    
        }
    }

    pub fn str_to_json_value( &self, str : &str ) -> anyhow::Result<Value>
    {
        match self
        {
            TypeState::String =>
            {
                Ok( Value::String( str.to_string() ) )
            },
            TypeState::Bool =>
            {
                Ok(
                    Value::Bool( str.trim().to_lowercase() == "true" )
                )
            },
            TypeState::Json =>
            {
                let json : Value = serde_json::from_str( str )?;

                Ok( json )
            },
            TypeState::Number =>
            {
                Ok(
                    Value::Number( Number::from_str( str )? )
                )
            }
        }
    }
}

impl FromStr for TypeState
{
    type Err = anyhow::Error;

    fn from_str( s : &str ) -> Result<Self, Self::Err> 
    {
        match s.trim().to_lowercase().as_str()
        {
            "number" => Ok( TypeState::Number ),
            "str" => Ok( TypeState::String ),
            "bool" => Ok( TypeState::Bool ),
            "json" => Ok( TypeState::Json ),
            _ => Err( anyhow::Error::msg( format!( "Typestate {s} not found" ) ) )
        }
    }
}

#[derive(Debug)]
pub enum StateExecutor
{
    Var( VarState ),
    Command( CommandState )
}