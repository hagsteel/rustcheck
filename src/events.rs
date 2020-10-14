use std::sync::mpsc;
use std::thread;
use std::process;
use std::io::{self, BufRead};

use crossterm::event::{self as crossterm_event, Event as CTEvent, KeyEvent, KeyCode};

use crate::parser::{parse_line, BuildResult};

type Tx = mpsc::Sender<Event>;

#[derive(Debug)]
pub enum Event {
    BuildEvent(BuildResult),
    Redraw,
    Quit,
}

fn cargo_watch(tx: Tx) -> process::Child {

    let mut output = process::Command::new("cargo")
        .arg("watch")
        .arg("-x")
        .arg("check --message-format short")
        .stderr(process::Stdio::piped())
        .stdout(process::Stdio::null())
        .spawn()
        .unwrap_or_else(|_| {
            eprintln!("Err: Failed to get output");
            process::exit(1);
        });

    let stderr = output.stderr.take().expect("failed to get stderr");
    let br = io::BufReader::new(stderr);
    let mut lines = br.lines();
    let mut output_lines = Vec::new();
    if let Some(Ok(line)) = lines.next() {
        if line.starts_with("error: Not a Cargo project, aborting.") {
            let _ = output.kill();
            panic!("Not a Rust project");
        } else {
            output_lines.push(line);
        }
    }

    thread::spawn(move|| {

        loop {
            while let Some(Ok(line)) = lines.next() {
                // If this is not a Cargo project then bail:

                // Four spaces or blank line means end
                if line.starts_with("    ") || line.trim() == "" {
                    break 
                } else {
                    output_lines.push(line);
                }
            }

            let _ = tx.send(Event::BuildEvent(output_lines.drain(..).filter_map(parse_line).collect()));
        }

    });

    output
}

// -----------------------------------------------------------------------------
//     - Input / resize events -
// -----------------------------------------------------------------------------
fn input_events(tx: Tx) {
    thread::spawn(move || {
        for ev in crossterm_event::read() {
            match ev {
                CTEvent::Key(KeyEvent { code: KeyCode::Esc, .. }) => {
                    let _ = tx.send(Event::Quit);
                }
                CTEvent::Resize(..) => {
                    let _ = tx.send(Event::Redraw);
                }
                CTEvent::Key(_) | CTEvent::Mouse(_) => {}
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
