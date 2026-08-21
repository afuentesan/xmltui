
#[derive(Debug)]
pub enum RTMLSource
{
    File( String )
}

impl RTMLSource
{
    pub fn source( &self ) -> &str
    {
        match self
        {
            RTMLSource::File( f ) =>
            {
                f
            }    
        }
    }
}