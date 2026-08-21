use std::thread;

use crate::{app::event::{AppEvent, send_app_event}, input::{event::{InputEvent, init_input_event_channels}, focus_events::{focus_back, focus_next, send_focus_event}}};


pub fn init_input()
{
    let rx = init_input_event_channels();

    thread::spawn(
        move ||
        {
            loop
            {
                match rx.recv()
                {
                    Ok( event ) =>
                    {
                        process_input_event( event );
                    },
                    Err( _ ) => break
                }
            }
        }
    );
}

fn process_input_event( event : InputEvent )
{
    match event
    {
        InputEvent::Ctrl( k ) if let Some( c ) = k.as_char() && c == 'q' =>
        {
            send_app_event( AppEvent::Exit );
        },
        InputEvent::Tab =>
        {
            focus_next();
        },
        InputEvent::ShiftTab =>
        {
            focus_back();
        },
        InputEvent::Left |
        InputEvent::Right |
        InputEvent::Up |
        InputEvent::Down |
        InputEvent::Char( _ ) |
        InputEvent::Backspace |
        InputEvent::Delete |
        InputEvent::End |
        InputEvent::Home |
        InputEvent::Enter =>
        {
            send_focus_event( event );
        },
        InputEvent::Alt( _k ) => {},
        InputEvent::AltShift( _k ) => {},
        InputEvent::CtrlAlt( _k ) => {},
        InputEvent::CtrlAltShift( _k ) => {},
        InputEvent::CtrlShift( _k ) => {},
        InputEvent::Esc => {},
        InputEvent::Other( _k ) => {},
        InputEvent::ShiftEnter => {},
        _ => {}
    }
}