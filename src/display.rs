use tui::backend::Backend;
use tui::style::{Color, Style};
use tui::terminal::Frame;
use tui::text::{Span, Spans};
use tui::widgets::Paragraph;

use crate::BuildResult;

fn draw_warnings_and_errors(f: &mut Frame<impl Backend>, result: &BuildResult) {
    let warning_style = {
        if result.warnings.len() > 0 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::DarkGray)
        }
    };

    let warnings = Span::styled(
        format!("{} ", result.warnings.len()),
        warning_style,
    );

    let error_style = {
        if result.errors.len() > 0 {
            Style::default().fg(Color::LightRed)
        } else {
            Style::default().fg(Color::DarkGray)
        }
    };

    let errors = Span::styled(
        format!("{} ", result.errors.len()),
        error_style
    );

    let sep = Span::from("|");

    let mut spans = vec![warnings, sep.clone(), errors, sep];

    if result.errors.len() > 0 {
        // Errors
        let last_line = result.last_line().expect("Errors without last line is not possible");
        spans.push(Span::raw(last_line.to_string()));
        spans.push(Span::styled(format!(" {}", last_line.message), Style::default().fg(Color::LightRed)));
    } else if result.warnings.len() > 0 {
        let last_line = result.last_line().expect("Warnings without last line is not possible");
        // Warnings
        spans.push(Span::raw(last_line.to_string()));
        spans.push(Span::styled(format!(" {}", last_line.message), Style::default().fg(Color::Yellow)));
    } else {
        // Yay everything is okay
        spans.push(Span::styled(format!("[OK]"), Style::default().fg(Color::Green)));
    }

    let p = Paragraph::new(Spans::from(spans));

    f.render_widget(p, f.size());
}

// -----------------------------------------------------------------------------
//     - Render the buld output -
// -----------------------------------------------------------------------------
pub fn render(f: &mut Frame<impl Backend>, result: &BuildResult) {
    draw_warnings_and_errors(f, result);
}
