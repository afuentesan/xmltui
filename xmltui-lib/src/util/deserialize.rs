use std::str::FromStr;

use convert_case::ccase;
use serde::{Deserialize, Deserializer};


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