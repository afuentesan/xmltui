use std::sync::{OnceLock, mpsc::{self, Receiver, Sender}};

use crate::{input::event::InputEvent, rtml::{rtml_doc::RTMLDoc, rtml_node::RTMLNodeId, util::rtml_event::{RTMLCallback, RTMLCallbackAction}}};


static TX_EVENT_CHANNEL : OnceLock<Sender<AppEvent>> = OnceLock::new();

pub fn init_app_event_channels() -> Receiver<AppEvent>
{
    let ( tx, rx ) = mpsc::channel();

    TX_EVENT_CHANNEL.set( tx ).expect( "init_app_event_channels only can run once" );

    rx
}

pub fn send_app_event( event : AppEvent )
{
    if let Some( c ) = TX_EVENT_CHANNEL.get()
    {
        let _ = c.send( event );
    }
}

#[derive(Debug)]
pub struct HidrateCommand
{
    pub doc_id : String,
    pub node_id : RTMLNodeId,
    pub xml : String
}

impl HidrateCommand
{
    pub fn new( doc_id : String, node_id : RTMLNodeId, xml : String ) -> Self
    {
        Self { doc_id, node_id, xml }
    }
}

#[derive(Debug)]
pub struct CallbackResponse
{
    pub callback_action : RTMLCallbackAction,
    pub response : String
}

impl CallbackResponse
{
    pub fn new( callback_action : RTMLCallbackAction, response : String ) -> Self
    {
        Self { callback_action, response }
    }
}

pub enum AppEvent
{
    Render( RTMLDoc ),
    FocusNext,
    FocusBack,
    FocusEvent( InputEvent ),
    LoadFile( String ),
    HidrateCommand( HidrateCommand ),
    Callback( RTMLCallback ),
    CallbackResponse( CallbackResponse ),
    Exit
}

