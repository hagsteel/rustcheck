use std::io::{self, BufRead};
use std::process;
use std::thread;

use crate::events::{Event, Tx};
use crate::parser::parse_line;

pub fn cargo_watch(tx: Tx) -> process::Child {
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
    let reader = io::BufReader::new(stderr);
    let mut lines = reader.lines();
    let mut output_lines = Vec::new();
    if let Some(Ok(line)) = lines.next() {
        if line.starts_with("error") {
            let _ = output.kill();
            panic!(line);
        } else {
            output_lines.push(line);
        }
    }


    thread::spawn(move || {
        loop {
            let build_result = output_lines.drain(..).filter_map(parse_line).collect();
            let _ = tx.send(Event::BuildEvent(build_result));

            while let Some(Ok(line)) = lines.next() {
                // Four spaces or blank line means end
                if line.starts_with("    ") || line.trim() == "" {
                    break;
                } else {
                    output_lines.push(line);
                }
            }
        }
    });

    output
}
