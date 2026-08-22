
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

    // let mut rest : Option<usize> = None;

    // let mut ret : String = String::from( "" );

    // let mut has_vars = false;

    // let mut index = 0;

    // while let Some( ( ( from, to ), end, var ) ) = next_var( &template[index..] )
    // {
    //     has_vars = true;

    //     ret.push_str( &template[ index..end ] );

    //     match var
    //     {
    //         Some( s ) =>
    //         {
    //             ret.push_str( &var_value( &context, s ) );
    //         },
    //         None => ret.push_str( &template[ from..=to ] ),
    //     }

    //     index = to + 1;

    //     rest = Some( index );
    // }

    // if ! has_vars
    // {
    //     template.to_string()
    // }
    // else
    // {
    //     if let Some( rest ) = rest && rest < template.len()
    //     {
    //         ret.push_str( &template[ rest.. ] );
    //     }

    //     ret    
    // }
}

fn var_value( context : &Value, var : &str ) -> String
{
    let pointer = context.pointer( var );

    if let Some( v ) = pointer
    {
        match v
        {
            Value::String( s ) => s.to_string(),
            Value::Number( n ) => n.to_string(),
            Value::Bool( b ) => b.to_string(),
            Value::Array( a ) => format!( "{:?}", a ),
            Value::Object( a ) => format!( "{:?}", a ),
            Value::Null => "".to_string()
        }
    }
    else
    {
        "".to_string()
    }
}


// ( empieza_siguiente, termina_anterior, )
fn next_var( template : &str ) -> Option<( ( usize, usize ), usize, Option<&str> )>
{
    let first = template.find( "{" )?;

    let last = template[ first.. ].find( "}" )? + first;

    let mut next = first;

    let mut count = 0;

    while next > 0
    {
        if &template[ next .. ( next + 1 ) ] == r#"\"#
        {
            count += 1;

            next -= 1;
        }
        else
        {
            break    
        }
    }

    if count == 0
    {
        Some(
            (
                ( first, last ),
                first,
                Some( &template[ ( first + 1 )..last ] )
            )
        )
    }
    else if ( count % 2 ) == 0
    {
        Some(
            (
                ( first, last ),
                first - 1,
                Some( &template[ ( first + 1 )..last ] )
            )
        )
    }
    else
    {
        Some(
            (
                ( first, last ),
                first - 1,
                None
            )
        )   
    }
}