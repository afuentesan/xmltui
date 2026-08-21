use std::{sync::{OnceLock, mpsc::{self, Receiver, Sender}}, thread};

use crossterm::event::{Event, KeyCode, KeyModifiers};

static TX_INPUT_EVENT_CHANNEL : OnceLock<Sender<InputEvent>> = OnceLock::new();

pub fn init_input_event_channels() -> Receiver<InputEvent>
{
    let ( tx, rx ) = mpsc::channel();

    TX_INPUT_EVENT_CHANNEL.set( tx ).expect( "init_input_event_channels only can run once" );

    handle_input_events();

    rx
}

fn send_input_event( event : InputEvent )
{
    if let Some( c ) = TX_INPUT_EVENT_CHANNEL.get()
    {
        let _ = c.send( event );
    }
}

pub enum InputEvent
{
    Char( ( KeyCode, char ) ),
    Alt( KeyCode ),
    AltShift( KeyCode ),
    Ctrl( KeyCode ),
    CtrlShift( KeyCode ),
    CtrlAlt( KeyCode ),
    CtrlAltShift( KeyCode ),
    ShiftEnter,
    ShiftTab,
    Enter,
    Up,
    Down,
    Left,
    Right,
    Tab,
    Esc,
    Delete,
    Backspace,
    End,
    Home,
    Other( KeyCode )
}

fn handle_input_events()
{
    thread::spawn(
        move ||
        {
            loop
            {
                match crossterm::event::read()
                {
                    Ok( ev ) =>
                    {
                        if let Some( ev ) = event_to_input_event( ev )
                        {
                            send_input_event( ev );
                        }
                    },
                    Err( _ ) => {}
                };
            }
        }
    );
}

fn event_to_input_event( event : Event ) -> Option<InputEvent>
{
    match event
    {
        Event::Key( k ) if ( k.is_press() || k.is_repeat() ) =>
        {
            if is_ctrl_alt_shift( k.modifiers )
            {
                Some( InputEvent::CtrlAltShift( k.code ) )
            }
            else if is_ctrl_alt( k.modifiers )
            {
                Some( InputEvent::CtrlAlt( k.code ) )
            }
            else if is_ctrl_shift( k.modifiers )
            {
                Some( InputEvent::CtrlShift( k.code ) )
            }
            else if is_alt_shift( k.modifiers )
            {
                Some( InputEvent::AltShift( k.code ) )
            }
            else if is_ctrl( k.modifiers )
            {
                Some( InputEvent::Ctrl( k.code ) )
            }
            else if is_alt( k.modifiers )
            {
                Some( InputEvent::Alt( k.code ) )
            }
            else if is_shift( k.modifiers ) && k.code.is_enter()
            {
                Some( InputEvent::ShiftEnter )
            }
            else if k.code.is_back_tab()
            {
                Some( InputEvent::ShiftTab )
            }
            else if k.code.is_enter()
            {
                Some( InputEvent::Enter )
            }
            else if k.code.is_tab()
            {
                Some( InputEvent::Tab )
            }
            else if k.code.is_up()
            {
                Some( InputEvent::Up )
            }
            else if k.code.is_down()
            {
                Some( InputEvent::Down )
            }
            else if k.code.is_left()
            {
                Some( InputEvent::Left )
            }
            else if k.code.is_right()
            {
                Some( InputEvent::Right )
            }
            else if k.code.is_esc()
            {
                Some( InputEvent::Esc )
            }
            else if k.code.is_delete()
            {
                Some( InputEvent::Delete )
            }
            else if k.code.is_backspace()
            {
                Some( InputEvent::Backspace )
            }
            else if k.code.is_end()
            {
                Some( InputEvent::End )
            }
            else if k.code.is_home()
            {
                Some( InputEvent::Home )
            }
            else if let Some( c ) = k.code.as_char()
            {
                Some( InputEvent::Char( ( k.code, c ) ) )
            }
            else
            {
                Some( InputEvent::Other( k.code ) )
            }
        },
        _ => None
    }
}

fn is_ctrl( modifiers : KeyModifiers ) -> bool
{
    modifiers.contains( KeyModifiers::CONTROL )
}

fn is_alt( modifiers : KeyModifiers ) -> bool
{
    modifiers.contains( KeyModifiers::ALT )
}

fn is_shift( modifiers : KeyModifiers ) -> bool
{
    modifiers.contains( KeyModifiers::SHIFT )
}

fn is_alt_shift( modifiers : KeyModifiers ) -> bool
{
    is_alt( modifiers ) && is_shift( modifiers )
}

fn is_ctrl_alt( modifiers : KeyModifiers ) -> bool
{
    is_ctrl( modifiers ) && is_alt( modifiers )
}

fn is_ctrl_shift( modifiers : KeyModifiers ) -> bool
{
    is_ctrl( modifiers ) && is_shift( modifiers )
}

fn is_ctrl_alt_shift( modifiers : KeyModifiers ) -> bool
{
    is_ctrl( modifiers ) && is_alt( modifiers ) && is_shift( modifiers )
}