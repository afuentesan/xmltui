use ratatui::layout::Constraint;
use serde::Deserialize;

use crate::util::json::deserialize_string_or_type;


#[derive(Deserialize, Default)]
pub struct ConstraintBuilder 
{
    #[serde( default, deserialize_with = "deserialize_string_or_type" )]
    pub min : Option<u16>,
    #[serde( default, deserialize_with = "deserialize_string_or_type" )]
    pub max : Option<u16>,
    #[serde( default, deserialize_with = "deserialize_string_or_type" )]
    pub length : Option<u16>,
    #[serde( default, deserialize_with = "deserialize_string_or_type" )]
    pub percent : Option<u16>,
    #[serde( default, deserialize_with = "deserialize_string_or_type" )]
    pub fill : Option<u16>,
    pub ratio : Option<String>
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(from = "ConstraintBuilder")]
pub struct XMLConstraint( pub Option<Constraint> );

impl From<ConstraintBuilder> for XMLConstraint 
{
    fn from( builder: ConstraintBuilder ) -> Self 
    {
        let constraint = if let Some( v ) = builder.fill 
        {
            Some( Constraint::Fill( v ) )
        } 
        else if let Some( v ) = builder.min 
        {
            Some( Constraint::Min( v ) )
        } 
        else if let Some( v ) = builder.max 
        {
            Some( Constraint::Max( v ) )
        } 
        else if let Some( v ) = builder.length 
        {
            Some( Constraint::Length( v ) )
        } 
        else if let Some( v ) = builder.percent 
        {
            Some( Constraint::Percentage( v ) )
        } 
        else if let Some( r ) = builder.ratio 
        {
            let parts: Vec<&str> = r.split(',').collect();

            if parts.len() == 2 
            {
                let num = parts[ 0 ].trim().parse::<u32>().unwrap_or( 1 );
                let den = parts[ 1 ].trim().parse::<u32>().unwrap_or( 1 );

                Some( Constraint::Ratio( num, den ) )
            } 
            else 
            {
                None
            }
        } 
        else 
        {
            None
        };

        XMLConstraint( constraint )
    }
}