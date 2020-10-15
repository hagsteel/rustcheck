use crossterm::terminal::enable_raw_mode;
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

mod display;
mod events;
mod parser;
mod watch;

use crate::events::{events, Event};
pub use parser::BuildResult;

fn main() -> io::Result<()> {

    enable_raw_mode().expect("Could not put terminal into raw mode");
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    let (rx, mut child_proc) = events();

    let mut build_res: Option<BuildResult> = None;
    loop {
        let event = match rx.recv() {
            Ok(e) => e,
            Err(_e) => continue,
        };


        match event {
            Event::BuildEvent(br) => {
                let _ = terminal.draw(|f| display::render(f, &br));
                build_res = Some(br);
            }
            Event::Redraw => {
                build_res.as_ref().map(|br| {
                    let _ = terminal.draw(|f| display::render(f, br));
                });
            }
            Event::Quit => {
                child_proc
                    .kill()
                    .expect("Failed to terminate the child process");
                break Ok(());
            }
        }
    }
}
