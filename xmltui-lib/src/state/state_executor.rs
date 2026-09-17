use std::str::FromStr;

use serde_json::{Number, Value};

use crate::state::{command_state::CommandState, var_state::VarState};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Copy)]
pub enum TypeState
{
    String,
    Number,
    Bool,
    Json,
    StrVec
}

impl TypeState
{
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
                    Value::Number( Number::from_str( str.trim() )? )
                )
            },
            TypeState::StrVec =>
            {
                let vec : Result<Vec<String>, _> = serde_json::from_str( str );

                let context = match vec
                {
                    Ok( v ) =>
                    {
                        v.into_iter().map( | s | serde_json::Value::String( s ) ).collect()
                    },
                    Err( _ ) =>
                    {
                        str.split( "\n" ).map( | s | serde_json::Value::String( s.to_string() ) ).collect::<Vec<_>>()
                    }
                };

                Ok( Value::Array( context ) )
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
            "strvec" => Ok( TypeState::StrVec ),
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