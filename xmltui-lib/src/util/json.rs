use std::str::FromStr;

use convert_case::ccase;
use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value};

use crate::util::str::is_uint;


pub fn deserialize_string_or_type<'de, D, T>( deserializer: D ) -> Result<Option<T>, D::Error>
where D: Deserializer<'de>, T: FromStr + Deserialize<'de>
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum TypeOrStr<T: FromStr> 
    {
        T( T ),
        Str( String )
    }

    if let Ok( val ) = TypeOrStr::deserialize( deserializer )
    {
        match val
        {
            TypeOrStr::T( i ) => Ok( Some( i ) ),
            TypeOrStr::Str( s ) => 
            {
                match s.trim().parse::<T>()
                {
                    Ok( n ) => Ok( Some( n ) ),
                    Err( _ ) => Ok( None )
                }
            }
        }
    }
    else
    {
        Ok( None )    
    }
}

pub fn deserialize_kebab_string_or_type<'de, D, T>( deserializer: D ) -> Result<Option<T>, D::Error>
where D: Deserializer<'de>, T: FromStr + Deserialize<'de>
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum TypeOrKebab<T: FromStr> 
    {
        T( T ),
        Str( String )
    }

    if let Ok( val ) = TypeOrKebab::deserialize( deserializer )
    {
        match val
        {
            TypeOrKebab::T( i ) => Ok( Some( i ) ),
            TypeOrKebab::Str( s ) => 
            {
                match ccase!( pascal, s.trim() ).parse::<T>()
                {
                    Ok( n ) => Ok( Some( n ) ),
                    Err( _ ) => Ok( None )
                }
            }
        }
    }
    else
    {
        Ok( None )    
    }
}

pub fn create_or_replace_path( path : &str, mut current : &mut Value, value : Value )
{
    let parts = parts_from_path( path );

    if parts.is_empty() { return };

    let last = parts.len() - 1;

    for ( idx, part ) in parts.iter().enumerate()
    {
        if idx == last
        {
            create_part_if_not_exists( idx, part, current, Some( value ) );

            break;
        }
        
        current = create_part_if_not_exists( idx, part, current, None );
    }
}

fn create_part_if_not_exists<'a, 'b>( idx : usize, part : &'a str, value : &'b mut Value, insert_value : Option<Value> ) -> &'b mut Value
{
    if value.is_object()
    {
        insert_path_in_object( part, value, insert_value )
    }
    else if value.is_array() && idx > 0 && is_uint( part )
    {
        insert_path_in_array( part, value, insert_value )
    }
    else if idx > 0 && is_uint( part )
    {
        replace_path_with_array( part, value, insert_value )
    }
    else
    {
        replace_path_with_object( part, value, insert_value )
    }
}

fn replace_path_with_array<'a, 'b>( part : &'a str, value : &'b mut Value, insert_value : Option<Value> ) -> &'b mut Value
{
    match part.parse::<usize>()
    {
        Ok( n ) =>
        {
            let mut new_array = vec![ Value::Null; n ];

            new_array.push( insert_value.unwrap_or( Value::Null ) );
            
            *value = Value::Array( new_array );

            value.as_array_mut().unwrap().get_mut( n ).unwrap()
        },
        Err( _ ) => replace_path_with_object( part, value, insert_value )
    }
}

fn replace_path_with_object<'a, 'b>( part : &'a str, value : &'b mut Value, insert_value : Option<Value> ) -> &'b mut Value
{
    let mut new_object = Map::new();

    new_object.insert( part.to_string(), insert_value.unwrap_or( Value::Null ) );

    *value = Value::Object( new_object );

    value.as_object_mut().unwrap().get_mut( part ).unwrap()
}

fn insert_path_in_array<'a, 'b>( part : &'a str, value : &'b mut Value, insert_value : Option<Value> ) -> &'b mut Value
{
    match part.parse::<usize>() 
    {
        Ok( n ) =>
        {
            let arr = value.as_array_mut().unwrap();
            let len = arr.len();

            if n >= len
            {
                arr.resize( n, Value::Null );

                arr.push( insert_value.unwrap_or( Value::Null ) );
            }
            else if let Some( v ) = insert_value
            {
                arr[ n ] = v;
            }

            value.as_array_mut().unwrap().get_mut( n ).unwrap()
        },
        Err( _ ) => replace_path_with_object( part, value, insert_value )
    }
}

fn insert_path_in_object<'a, 'b>( part : &'a str, value : &'b mut Value, insert_value : Option<Value> ) -> &'b mut Value
{
    if value.as_object().unwrap().contains_key( part )
    {
        if let Some( v ) = insert_value
        {
            *value.as_object_mut().unwrap().get_mut( part ).unwrap() = v;
        }

        return value.as_object_mut().unwrap().get_mut( part ).unwrap();
    }
    
    value.as_object_mut().unwrap().insert( part.to_string(), insert_value.unwrap_or( Value::Null ) );

    value.as_object_mut().unwrap().get_mut( part ).unwrap()
}

fn parts_from_path( path : &str ) -> Vec<&str>
{
    path.split( "/" )
    .filter( | p | p.trim() != "" )
    .collect()
}

pub fn json_value_to_string( val : &Value ) -> String
{
    match val
    {
        Value::String( s ) => s.clone(),
        Value::Null => "".into(),
        Value::Bool( b ) => b.to_string(),
        Value::Number( n ) => n.to_string(),
        v => v.to_string() 
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_create_path_on_empty_map()
    {
        let mut value = Value::Object( Map::new() );

        create_or_replace_path( "/hola/0/periquito", &mut value, Value::String( "bien".into() ) );

        test_hola_0_periquito_bien( &mut value );
    }

    #[test]
    fn test_create_path_on_fill_map()
    {
        let mut value = Value::Object( Map::new() );

        create_or_replace_path( "/hola/0/periquito", &mut value, Value::String( "bien".into() ) );

        test_hola_0_periquito_bien( &mut value );

        create_or_replace_path( "/hola/3/periquito", &mut value, Value::String( "bien_2".into() ) );

        let hola = value.pointer( "/hola" );

        assert_eq!( hola.unwrap().as_array().unwrap().len(), 4 );
    }

    #[test]
    fn test_create_path_on_fill_map_replace_value_in_map()
    {
        let mut value = Value::Object( Map::new() );

        create_or_replace_path( "/hola/0/periquito", &mut value, Value::String( "bien".into() ) );

        test_hola_0_periquito_bien( &mut value );

        create_or_replace_path( "/hola/0/periquito", &mut value, Value::String( "bien_2".into() ) );

        let hola = value.pointer( "/hola" );

        assert_eq!( hola.unwrap().as_array().unwrap().len(), 1 );

        let periquito = value.pointer( "/hola/0/periquito" ).unwrap();

        assert_eq!( periquito.as_str().unwrap(), "bien_2" );
    }

    #[test]
    fn test_create_path_on_fill_map_replace_value_in_array()
    {
        let mut value = Value::Object( Map::new() );

        create_or_replace_path( "/hola/0/periquito", &mut value, Value::String( "bien".into() ) );

        test_hola_0_periquito_bien( &mut value );

        create_or_replace_path( "/hola/3/periquito", &mut value, Value::String( "bien_2".into() ) );

        create_or_replace_path( "/hola/3", &mut value, Value::String( "bien_3".into() ) );

        let pointer = value.pointer( "/hola/3" );

        assert_eq!( pointer.unwrap().as_str().unwrap(), "bien_3" );
    }

    #[test]
    fn test_create_path_on_empty_map_array_index_1()
    {
        let mut value = Value::Object( Map::new() );

        create_or_replace_path( "/hola/1/periquito", &mut value, Value::String( "bien".into() ) );

        let pointer = value.pointer( "/hola/1/periquito" );

        assert_eq!( pointer.unwrap().as_str().unwrap(), "bien" );
    }

    #[test]
    fn test_create_path_empty_path()
    {
        let mut value = Value::Object( Map::new() );

        create_or_replace_path( "", &mut value, Value::String( "bien".into() ) );

        assert_eq!( value.as_object().unwrap().len(), 0 );

        create_or_replace_path( "///", &mut value, Value::String( "bien".into() ) );

        assert_eq!( value.as_object().unwrap().len(), 0 );

        create_or_replace_path( "/", &mut value, Value::String( "bien".into() ) );

        assert_eq!( value.as_object().unwrap().len(), 0 );
    }

    fn test_hola_0_periquito_bien( value : &mut Value )
    {
        let pointer = value.pointer( "/hola/0/periquito" );

        assert!( pointer.is_some() );

        let pointer = pointer.unwrap();

        assert!( pointer.is_string() );

        assert_eq!( pointer.as_str().unwrap(), "bien" );

        let pointer = value.pointer( "/hola/0" );

        assert!( pointer.is_some() && pointer.unwrap().is_object() );

        let pointer = value.pointer( "/hola" );

        assert!( pointer.is_some() && pointer.unwrap().is_array() );
    }
}