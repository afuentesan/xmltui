use std::sync::OnceLock;
use tokio::runtime::Runtime;

static ASYNC_RUNTIME: OnceLock<Runtime> = OnceLock::new();

pub fn init_async_app()
{
    let rt = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .expect( "Failed building the Tokio Runtime" );

    ASYNC_RUNTIME.set( rt ).expect( "init_async_app only can run once" );
}

pub fn spawn_async_task<F>( future : F )
where
    F: std::future::Future<Output = ()> + Send + 'static,
{
    if let Some( rt ) = ASYNC_RUNTIME.get() 
    {
        rt.spawn( future );
    } 
    else 
    {
        eprintln!( "Error: Se intentó lanzar una tarea pero el runtime no está inicializado" );
    }
}

// De momento no lo utilizo así que lo comento
// Si en algún momento necesito lanzar tareas asíncronas y no avanzar hasta que terminen utilizaría esto
// pub fn block_on_async_task<F>( future : F ) -> F::Output
// where
//     F: std::future::Future + Send,
// {
//     if let Some( rt ) = ASYNC_RUNTIME.get() 
//     {
//         rt.block_on( future )
//     } 
//     else 
//     {
//         panic!( "El runtime no está inicializado" );
//     }
// }