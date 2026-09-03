
#[derive(Debug)]
pub struct FieldAttrs
{
    pub path : String
}

impl FieldAttrs
{
    pub fn new( path : String ) -> Self
    {
        Self { path }
    }
}