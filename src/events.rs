use std::process;
use std::sync::mpsc;
use std::thread;

use crossterm::event::{
    self as crossterm_event, Event as CTEvent, KeyCode, KeyEvent, KeyModifiers,
};

use crate::parser::BuildResult;
use crate::watch::cargo_watch;

pub type Tx = mpsc::Sender<Event>;

#[derive(Debug)]
pub enum Event {
    BuildEvent(BuildResult),
    Redraw,
    Quit,
}


// -----------------------------------------------------------------------------
//     - Input / resize events -
// -----------------------------------------------------------------------------
fn input_events(tx: Tx) {
    thread::spawn(move || {
        loop {
            if let Ok(ev) = crossterm_event::read() {
                match ev {
                    CTEvent::Key(KeyEvent {
                        code: KeyCode::Esc, ..
                    }) => {
                        let _ = tx.send(Event::Quit);
                    }
                    CTEvent::Key(KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                    }) => {
                        let _ = tx.send(Event::Quit);
                    }
                    CTEvent::Resize(..) => {
                        let _ = tx.send(Event::Redraw);
                    }
                    CTEvent::Key(_) | CTEvent::Mouse(_) => {}
                }
            }
        }
    });
}

pub fn events() -> (mpsc::Receiver<Event>, process::Child) {
    let (tx, rx) = mpsc::channel();
    input_events(tx.clone());
    let child = cargo_watch(tx);
    (rx, child)
}
