use std::{fs, path::{Path, PathBuf}};

use anyhow::{Context, bail};

// De momento no lo estoy usando así que lo comento
// pub fn read_file_in_chroot( path : &str, root_path : &PathBuf ) -> anyhow::Result<String>
// {
//     read_file_in_chroot_with_option_extension( path, root_path, None )
// }

pub fn read_file_in_chroot_with_extension( path : &str, root_path : &PathBuf, extension : &str ) -> anyhow::Result<String>
{
    read_file_in_chroot_with_option_extension( path, root_path, Some( extension ) )
}

fn read_file_in_chroot_with_option_extension( path : &str, root_path : &PathBuf, extension : Option<&str> ) -> anyhow::Result<String>
{
    let requested_path = Path::new( path );

    let safe_requested_path = requested_path.strip_prefix( "/" ).unwrap_or( requested_path );

    let mut combined_path = root_path.join( safe_requested_path );

    if let Some( e ) = extension && combined_path.is_dir()
    {
        combined_path = combined_path.join( format!( "index{e}" ) );
    }

    if let Some( extension ) = extension
    {
        if let Some( e ) = combined_path.extension() && e == extension
        {
            // Tiene la extensión correcta
        }
        else
        {
            combined_path.add_extension( extension );
        }
    }

    let final_path = combined_path
        .canonicalize()
        .context( "El archivo solicitado no existe o es inaccesible" )?;

    if ! final_path.starts_with( &root_path ) 
    {
        bail!( "Acceso denegado: Intento de Path Traversal fuera de chroot." );
    }

    Ok( fs::read_to_string( final_path )? )
}

#[cfg(test)]
mod tests
{
    use super::*;

    // Los comento porque he comentado la función
    // #[test]
    // fn test_file()
    // {
    //     let content = read_file_in_chroot( 
    //         "examples/example1.xml", 
    //         &Path::new( "./" ).canonicalize().unwrap().to_path_buf() 
    //     ).expect( "Se esperaba que existiese el fichero examples/example1.xml en ./" );

    //     assert!( content.trim() != "" );
    // }

    // #[test]
    // fn test_dir()
    // {
    //     let content = read_file_in_chroot( 
    //         "examples", 
    //         &Path::new( "./" ).canonicalize().unwrap().to_path_buf() 
    //     );

    //     assert!( content.is_err() );
    // }

    #[test]
    fn test_file_with_extension()
    {
        let content = read_file_in_chroot_with_extension( 
            "examples/example1", 
            &Path::new( "./" ).canonicalize().unwrap().to_path_buf(),
            "xml"
        ).expect( "Se esperaba que existiese el fichero examples/example1 con la extensión xml en ./" );

        assert!( content.trim() != "" );
    }

     #[test]
    fn test_file_with_extension_added()
    {
        let content = read_file_in_chroot_with_extension( 
            "examples/example1.xml", 
            &Path::new( "./" ).canonicalize().unwrap().to_path_buf(),
            "xml"
        ).expect( "Se esperaba que existiese el fichero examples/example1 con la extensión xml en ./" );

        assert!( content.trim() != "" );
    }
}