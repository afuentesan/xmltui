
use minijinja::Environment;
use serde_json::{Value, json};

use crate::state::state_executor::TypeState;

pub fn template_and_err_to_xml( 
    data : String, 
    template : Option<impl AsRef<str>>, 
    data_type : TypeState, 
    state : &Value, 
    err : bool 
) -> anyhow::Result<String>
{
    if template.is_none() 
    {
        return Ok( data ) 
    };

    if err
    {
        let context = json!( { "ctx" : data, "st" : state, "err" : true } );

        xml_from_template_context( template.as_ref().unwrap(), &context )
    }
    else
    {
        let context = data_type.str_to_json_value( &data )?;

         let context = json!( { "ctx" : context, "st" : state, "err" : false } );

        xml_from_template_context( template.as_ref().unwrap(), &context )
    }
    
}

pub fn template_to_xml( data : String, template : Option<impl AsRef<str>>, data_type : TypeState, state : &Value ) -> anyhow::Result<String>
{
    template_and_err_to_xml( data, template, data_type, state, false )
}

pub fn xml_from_template_context( template : impl AsRef<str>, context : &Value ) -> anyhow::Result<String>
{
    let mut env = Environment::new();

    env.add_template( "rtml_template", template.as_ref() )?;

    let tmpl = env.get_template( "rtml_template" )?;

    Ok( tmpl.render( context )? )
}

// fn xml_from_template_context_parent_key( template : &str, context : Value, parent : Option<&str> ) -> anyhow::Result<String>
// {
//     let mut env = Environment::new();

//     env.add_template( "rtml_template", template )?;

//     let tmpl = env.get_template( "rtml_template" )?;

//     let context = if let Some( p ) = parent
//     {
//         json!( { p : context } )
//     }
//     else
//     {
//         context    
//     };

//     Ok( tmpl.render( context )? )
// }