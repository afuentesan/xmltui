use std::{path::{Path, PathBuf}, sync::OnceLock};

use anyhow::Context;

use crate::{app::event::{AppEvent, send_app_event}, util::log::log_to_file, xml::xml2rtml::xml2rtml_doc};

static CHROOT : OnceLock<PathBuf> = OnceLock::new();

pub fn init_chroot( path : &str ) -> anyhow::Result<()>
{
    if ! Path::new( path ).is_dir() { return Err( anyhow::Error::msg( format!( "init_chroot. El path {path} no existe o no es un directorio." ) ) ) };

    let root_path = Path::new( path )
    .canonicalize()
    .context( format!( "init_chroot. El path {path} no existe o no es un directorio." ) )?;

    CHROOT.set( root_path ).expect( "init_chroot solo se puede llamar una vez" );

    Ok( () )
}

pub fn chroot() -> &'static PathBuf
{
    CHROOT.get().expect( "Se esperaba haber llamado a la función init_chroot" )
}

pub fn load_file( path : &str )
{
    match xml2rtml_doc( path )
    {
        Ok( d ) =>
        {
            send_app_event( AppEvent::Render( d ) );
        },
        Err( e ) =>
        {
            log_to_file( &format!( "Error: {:?}", e ) );
            // TODO: Mostrar algún mensaje de error
        }
    }
}