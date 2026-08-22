use std::thread;

use ratatui::{DefaultTerminal, style::Style, widgets::Block};
use tokio_util::sync::CancellationToken;

use crate::{app::{app_callback::{execute_callback, execute_callback_response}, app_doc::load_file, event::{AppEvent, HidrateCommand, init_app_event_channels, send_app_event}}, rtml::rtml_doc::{RTMLDoc, render_rtml_doc}, util::{log::log_to_file, template::template_to_xml}, xml::xml2rtml::{replace_node_childs_with_xml, xml2rtml_doc}};

#[derive(Debug)]
pub struct App
{
    doc : RTMLDoc
}

impl App
{
    pub fn new( doc : RTMLDoc ) -> Self
    {
        Self { doc }
    }

    pub fn change_doc( &mut self, doc : RTMLDoc )
    {
        self.doc = doc;
    }
}

pub fn init_app( initial_path : &str ) -> anyhow::Result<()>
{
    let initial_doc = xml2rtml_doc( initial_path )?;

    let mut app = App::new( RTMLDoc::empty() );

    let rx = init_app_event_channels();

    let mut terminal = ratatui::init();

    let mut cancellation_token : Option<CancellationToken> = None;

    let th = thread::spawn(
        move ||
        {
            loop
            {
                match rx.recv()
                {
                    Ok( event ) =>
                    {
                        match event
                        {
                            AppEvent::Render( doc ) =>
                            {
                                if let Some( cancellation ) = cancellation_token
                                {
                                    cancellation.cancel();
                                }

                                cancellation_token = Some( CancellationToken::new() );
                                
                                app.change_doc( doc );

                                app.doc.init_commands( cancellation_token.as_ref().unwrap().clone() );

                                rtml_to_terminal( &mut terminal, &mut app.doc );
                            },
                            AppEvent::FocusNext =>
                            {
                                app.doc.focus_next();

                                rtml_to_terminal( &mut terminal, &mut app.doc );
                            },
                            AppEvent::FocusBack =>
                            {
                                app.doc.focus_back();

                                rtml_to_terminal( &mut terminal, &mut app.doc );
                            },
                            AppEvent::FocusEvent( input_event ) =>
                            {
                                if app.doc.focus_event( &input_event )
                                {
                                    rtml_to_terminal( &mut terminal, &mut app.doc );
                                }
                            },
                            AppEvent::LoadFile( path ) =>
                            {
                                load_file( &path );
                            },
                            AppEvent::HidrateCommand( h ) =>
                            {
                                hidrate_command( 
                                    &mut terminal, 
                                    &mut app.doc, 
                                    h, 
                                    cancellation_token.as_ref().unwrap().clone() 
                                );
                            },
                            AppEvent::Callback( c ) =>
                            {
                                execute_callback( &app.doc, c );
                            },
                            AppEvent::CallbackResponse( r ) =>
                            {
                                if execute_callback_response(
                                    &mut app.doc, 
                                    r,
                                    cancellation_token.as_ref().unwrap().clone() 
                                )
                                {
                                    rtml_to_terminal( &mut terminal, &mut app.doc );
                                }
                            },
                            AppEvent::Exit =>
                            {
                                if let Some( cancellation ) = cancellation_token
                                {
                                    cancellation.cancel();
                                }
                                
                                break    
                            }
                        }
                    },
                    Err( _ ) => break
                }    
            }
        }
    );

    send_app_event( AppEvent::Render( initial_doc ) );

    let _ = th.join();

    ratatui::restore();

    Ok( () )
}



fn hidrate_command(
    terminal : &mut DefaultTerminal,
    rtml_doc : &mut RTMLDoc,
    hidrate : HidrateCommand,
    cancellation_token : CancellationToken
)
{
    if hidrate.doc_id != rtml_doc.doc_id { return };

    let mut response = template_to_xml( hidrate.response, rtml_doc.node_template( &hidrate.node_id ), rtml_doc.command_output( &hidrate.node_id ) );

    if let Some( wrapper ) = rtml_doc.node_wrapper( &hidrate.node_id )
    {
        response = format!( "{}{}{}", wrapper.prefix, response, wrapper.suffix );
    }
   
    match replace_node_childs_with_xml( rtml_doc, hidrate.node_id.clone(), &response ) 
    {
        Ok( _ ) =>
        {
            rtml_doc.init_commands_for_childs( cancellation_token, &hidrate.node_id );

            rtml_to_terminal( terminal, rtml_doc );
        },
        Err( e ) =>
        {
            // TODO: Mostrar algún tipo de error
            log_to_file( &format!( "append_xml_to_node. XML: {}\n Error: {:?}", response, e ) );
        }
    }
}

fn rtml_to_terminal(
    terminal : &mut DefaultTerminal,
    rtml : &mut RTMLDoc
)
{
    match terminal.draw(
            | frame |
        {
            frame.render_widget( default_background( &rtml.style ), frame.area() );
            
            match render_rtml_doc(
                frame.area(),
                frame.buffer_mut(),
                rtml
            )
            {
                Ok( _ ) => {},
                Err( e ) => 
                {
                    // TODO: Si hay un error modificar el rtml para mostrar una página de error.
                    // Habrá que ver si se puede utilizar el mismo frame para repintar la página
                    log_to_file( &format!( "Terminal draw error. {:?}", e ) );
                }
            }
        }
    )
    {
        Ok( _ ) => {},
        Err( e ) =>
        {
            // TODO: Si hay un error modificar el rtml para mostrar una página de error.
            // Habrá que ver si tiene sentido llamar otra vez a terminal.draw o si simplemente modificamos el rtml
            log_to_file( &format!( "Terminal draw error. {:?}", e ) );
        }
    };
}

fn default_background( style : &Style ) -> Block<'_>
{
    Block::default().style( *style )
}