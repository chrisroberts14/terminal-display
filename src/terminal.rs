use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use std::io::{stdout, Write};
use std::sync::mpsc;
use crate::command::Command;
use std::thread;

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

    fn set_line(&mut self, index: usize, content: impl Into<String>) {
        if index >= self.lines.len() {
            self.lines.resize(index + 1, String::new());
        }
        self.lines[index] = content.into();
    }

    fn clear(&mut self) {
        self.lines.clear();
    }

    fn render(&mut self) -> std::io::Result<()> {
        let mut stdout = stdout();

        for (i, line) in self.lines.iter().enumerate() {
            if self.prev_lines.get(i) != Some(line) {
                execute!(
                    stdout,
                    MoveTo(0, i as u16),
                    Clear(ClearType::CurrentLine)
                )?;
                print!("{}", line);
            }
        }

        // If we previously had MORE lines, clear the leftovers
        if self.prev_lines.len() > self.lines.len() {
            for i in self.lines.len()..self.prev_lines.len() {
                execute!(
                    stdout,
                    MoveTo(0, i as u16),
                    Clear(ClearType::CurrentLine)
                )?;
            }
        }

        self.prev_lines = self.lines.clone();
        stdout.flush()?;

        Ok(())
    }

    pub fn run(self) -> TerminalHandle {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let mut terminal = self;

            while let Ok(cmd) = rx.recv() {
                match cmd {
                    Command::SetLine(i, content) => {
                        terminal.set_line(i, content);
                    }
                    Command::Clear => {
                        terminal.clear();
                    }
                    Command::Shutdown => break,
                }

                // Render AFTER handling updates
                terminal.render().unwrap();
            }
        });

        TerminalHandle { tx }
    }
}

pub struct TerminalHandle {
    tx: mpsc::Sender<Command>,
}

impl TerminalHandle {
    pub fn set_line(&self, index: usize, content: impl Into<String>) {
        let _ = self.tx.send(Command::SetLine(index, content.into()));
    }

    pub fn clear(&self) {
        let _ = self.tx.send(Command::Clear);
    }

    pub fn shutdown(&self) {
        let _ = self.tx.send(Command::Shutdown);
    }
}
