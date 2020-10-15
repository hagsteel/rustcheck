use std::fmt::{self, Display};
use std::iter::FromIterator;
use std::path::PathBuf;
use std::str;

const WARNING: &'static str = "warning:";
const ERROR: &'static str = "error";

// -----------------------------------------------------------------------------
//     - Location -
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct Location {
    pub path: PathBuf,
    pub line: usize,
    pub col: usize,
    pub message: String,
}

impl Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path = match self.path.to_str() {
            Some(p) => p,
            None => "[err]",
        };
        write!(f, "{}:{}:{}", path, self.line, self.col)
    }
}

impl Location {
    fn from_parts(parts: Vec<&str>, message: &str) -> Self {
        Self {
            path: parts[0].into(),
            line: parts[1].parse().expect("not a valid line number"),
            col: parts[2].parse().expect(""),
            message: message.into(),
        }
    }
}

// -----------------------------------------------------------------------------
//     - Build result -
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct BuildResult {
    pub warnings: Vec<Location>,
    pub errors: Vec<Location>,
}

impl BuildResult {
    pub fn ok() -> Self {
        Self {
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn zero_messages(&self) -> bool {
        self.warnings.len() + self.errors.len() == 0
    }

    pub fn last_line(&self) -> Option<&Location> {
        match (self.errors.last(), self.warnings.last()) {
            (Some(e), _) => Some(e),
            (None, Some(w)) => Some(w),
            (None, None) => None,
        }
    }
}

impl FromIterator<Line> for BuildResult {
    fn from_iter<T: IntoIterator<Item = Line>>(iter: T) -> Self {
        let mut warnings = Vec::new();
        let mut errors = Vec::new();
        iter.into_iter().for_each(|line| match line {
            Line::Warning(loc) => warnings.push(loc),
            Line::Error(loc) => errors.push(loc),
        });

        Self { warnings, errors }
    }
}

// -----------------------------------------------------------------------------
//     - Line -
// -----------------------------------------------------------------------------
pub enum Line {
    Warning(Location),
    Error(Location),
}

pub fn parse_line(line: String) -> Option<Line> {
    line_from_parts(line.splitn(3, ' ').collect())
}

fn line_from_parts(p: Vec<&str>) -> Option<Line> {
    if p.len() < 2 {
        return None;
    }

    match p[1] {
        msg if msg == WARNING || msg.starts_with(ERROR) => {
            let line_msg = {
                if p.len() == 3 {
                    p[2]
                } else {
                    ""
                }
            };
            let path_parts = p[0].split(':').collect::<Vec<_>>();
            let loc = Location::from_parts(path_parts, line_msg);
            if msg == WARNING {
                Some(Line::Warning(loc))
            } else {
                Some(Line::Error(loc))
            }
        }
        _ => None,
    }
}
