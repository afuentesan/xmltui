use std::{fs::OpenOptions, io::Write};


const DEFAULT_FILE : &str = "/tmp/rtml.log";

pub fn log_to_file( msg : &str )
{
    match OpenOptions::new()
    .write( true )
    .append( true )
    .create( true )
    .open( DEFAULT_FILE )
    {
        Ok( mut f ) =>
        {
            match f.write_all( msg.as_bytes() )
            {
                Ok( _ ) => {
                    let _ = f.write_all( "\n".as_bytes() );
                },
                Err( e ) => eprintln!( "Error en log to file: {:?}", e )
            }
        },
        Err( e ) => eprintln!( "Error en log to file: {:?}", e )
    }
}