use std::sync::{OnceLock, mpsc::{self, Receiver, Sender}};

use crate::{code::event::CommandExecutorParams, input::event::InputEvent, rtml::{rtml_doc::RTMLDoc, rtml_node::RTMLNodeId, rtml_toast::ToastParams, util::rtml_event::{RTMLCallback, RTMLCallbackAction}}};


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
    pub response : String,
    pub has_err : bool
}

impl HidrateCommand
{
    pub fn new( doc_id : String, node_id : RTMLNodeId, response : String, has_err : bool ) -> Self
    {
        Self { doc_id, node_id, response, has_err }
    }
}

#[derive(Debug)]
pub struct HidrateState
{
    pub doc_id : String,
    pub node_id : RTMLNodeId
}

impl HidrateState
{
    pub fn new( doc_id : String, node_id : RTMLNodeId ) -> Self
    {
        Self { doc_id, node_id }
    }
}

#[derive(Debug)]
pub struct CallbackResponse
{
    pub callback_action : RTMLCallbackAction,
    pub response : String,
    pub has_err : bool
}

impl CallbackResponse
{
    pub fn new( callback_action : RTMLCallbackAction, response : String, has_err : bool ) -> Self
    {
        Self { callback_action, response, has_err }
    }
}

pub struct ShowMessage
{
    pub doc_id : String,
    pub toast : ToastParams,
    pub response : String
}

impl ShowMessage
{
    pub fn new( doc_id : String, toast : ToastParams, response : String ) -> Self
    {
        Self { doc_id, toast, response }
    }
}

pub enum AppEvent
{
    Render( RTMLDoc ),
    ReRender,
    FocusNext,
    FocusBack,
    FocusEvent( InputEvent ),
    LoadFile( String ),
    HidrateCommand( HidrateCommand ),
    HidrateState( HidrateState ),
    Callback( RTMLCallback ),
    CallbackResponse( CallbackResponse ),
    RefreshCommand( CommandExecutorParams ),
    ShowMessage( ShowMessage ),
    CloseMessage( String ),
    Esc,
    Exit
}

