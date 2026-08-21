use crate::{app::event::{AppEvent, send_app_event}, input::event::InputEvent};


pub fn focus_next()
{
    send_app_event( AppEvent::FocusNext );
}

pub fn focus_back()
{
    send_app_event( AppEvent::FocusBack );
}

pub fn send_focus_event( event : InputEvent )
{
    send_app_event( AppEvent::FocusEvent( event ) );
}