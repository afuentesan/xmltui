
use minijinja::Environment;
use serde_json::{Value, json};

use crate::rtml::rtml_command::RTMLCommandOutput;


pub fn template_to_xml( data : String, template : Option<&String>, data_type : RTMLCommandOutput ) -> anyhow::Result<String>
{
    if template.is_none() { return Ok( data ) };

    match data_type
    {
        RTMLCommandOutput::String =>
        {
            let context = serde_json::Value::String( data );

            xml_from_template_context( template.as_ref().unwrap(), context )
        },
        RTMLCommandOutput::StrVec =>
        {
            let vec : Result<Vec<String>, _> = serde_json::from_str( &data );

            let context = match vec
            {
                Ok( v ) =>
                {
                    v.into_iter().map( | s | serde_json::Value::String( s ) ).collect()
                },
                Err( _ ) =>
                {
                    data.split( "\n" ).map( | s | serde_json::Value::String( s.to_string() ) ).collect::<Vec<_>>()
                }
            };

            xml_from_template_context( template.as_ref().unwrap(), serde_json::Value::Array( context ) )
        },
        RTMLCommandOutput::Json =>
        {
            let json : Result<serde_json::Value, _> = serde_json::from_str( &data );

            let context = match json
            {
                Ok( v ) => v,
                Err( _ ) => serde_json::Value::String( data )
            };

            xml_from_template_context( template.as_ref().unwrap(), context )
        }
    }
}

fn xml_from_template_context( template : &str, context : Value ) -> anyhow::Result<String>
{
    let mut env = Environment::new();

    env.add_template( "rtml_template", template )?;

    let tmpl = env.get_template( "rtml_template" )?;

    let context = json!( { "ctx" : context } );

    Ok( tmpl.render( context )? )
}