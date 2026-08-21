use crate::{app::{app::init_app, app_doc::init_chroot}, async_app::async_app::init_async_app, input::input::init_input};

mod rtml;
mod xml;
mod app;
mod input;
mod util;
mod code;
mod async_app;

pub fn run_app( 
    chroot : &str,
    initial_path : &str
) -> anyhow::Result<()>
{
    init_chroot( chroot )?;
    
    init_async_app();

    init_input();

    init_app( initial_path )
}