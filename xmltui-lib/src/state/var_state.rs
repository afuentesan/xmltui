use serde_json::Value;

use crate::{state::state_executor::CommonState, util::{json::create_or_replace_path, log::log_to_file}};


#[derive(Debug)]
pub struct VarState
{
    pub common : CommonState,
    pub value : String
}

impl VarState
{
    pub fn new( common : CommonState, value : String ) -> Self
    {
        Self { common, value }
    }
}

pub fn change_var_state( params : &VarState, state : &mut Value ) -> bool
{
    match params.common.stype.str_to_json_value( params.value.as_str() )
    {
        Ok( v ) => 
        {
            create_or_replace_path( params.common.path.as_str(), state, v );

            true
        },
        Err( e ) =>
        {
            log_to_file( &format!( "Se ha producido un error al guardar una variable en el estado. Err: {e:?}" ) );

            false
        }
    }
}