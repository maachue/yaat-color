use std::time::Duration;

use owo_colors::{Style, Styled};

pub const WARN_MSG: Styled<&'static str> = Style::new().yellow().bold().style("warning");
pub const DOING_WORK_MSG: Style = Style::new().bright_green().bold();
// pub const ERR_MSG: Styled<&'static str> = Style::new().red().bold().style("error");
#[allow(dead_code)]
pub const NOTE_MSG: Styled<&'static str> = Style::new().bold().green().style("note");

pub fn format_duration(d: Duration) -> String {
    match d {
        d if d.as_secs() >= 10 => format!("{}s", d.as_secs()),
        d if d.as_secs() >= 1 => format!("{:.2}s", d.as_secs_f64()),
        d if d.as_millis() >= 1 => format!("{}ms", d.as_millis()),
        d if d.as_micros() >= 1 => format!("{}µs", d.as_micros()),
        _ => format!("{}ns", d.as_nanos()),
    }
}

pub fn read_stdin() -> Option<String> {
    use std::io::{self, IsTerminal, Read};

    if !io::stdin().is_terminal() {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).ok()?;
        Some(buf)
    } else {
        None
    }
}
