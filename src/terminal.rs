use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use std::io::Write;

pub struct Terminal {
    lines: Vec<String>,
    prev_lines: Vec<String>,
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            lines: Vec::default(),
            prev_lines: Vec::default(),
        }
    }

    pub fn set_line(&mut self, index: usize, content: impl Into<String>) {
        if index >= self.lines.len() {
            self.lines.resize(index + 1, String::new());
        }
        self.lines[index] = content.into();
    }

    pub fn render(&mut self) -> std::io::Result<()> {
        let mut stdout = std::io::stdout();

        for (i, line) in self.lines.iter().enumerate() {
            if self.prev_lines.get(i) != Some(line) {
                execute!(stdout, MoveTo(0, i as u16), Clear(ClearType::CurrentLine))?;
                print!("{}", line);
            }
        }
        self.prev_lines = self.lines.clone();

        stdout.flush()?;
        Ok(())
    }

    pub fn run() {}
}
