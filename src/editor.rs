use anyhow::Result;
use log::debug;
use std::io::stdout;
use std::io::{stdin, Error, Write};
use std::process;
use termion::event::Key;
use termion::input::TermRead;

use crate::terminal::Terminal;

#[derive(Debug)]
struct CursorPosition {
    x: u16,
    y: u16,
}

pub struct Editor {
    terminal: Terminal,
    text: String,
    cursor_position: CursorPosition,
    save: bool,
    quit: bool,
}

impl Editor {
    pub fn default() -> Result<Self> {
        Ok(Self {
            terminal: Terminal::default()?,
            text: "".into(),
            cursor_position: CursorPosition { x: 1, y: 1 },
            save: false,
            quit: false,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            if self.quit {
                Self::quit()?;
            }
            if let Err(err) = self.refresh_screen() {
                Self::die(&err);
            }
            if let Err(err) = self.process_keypress() {
                Self::die(&err);
            }
        }
    }

    fn quit() -> Result<()> {
        // FIX: This removes the previous command history from the screen. Also messes up the
        // output somehow. Any command run after quiting will have wonky output
        Self::clear_screen()?;
        process::exit(0);
    }

    fn clear_screen() -> Result<()> {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        stdout().flush()?;
        Ok(())
    }

    fn process_keypress(&mut self) -> Result<()> {
        let key = Self::read_key()?;
        match key {
            Key::Ctrl('q') => self.quit = true,
            Key::Char(a) => {
                if a == '\n' {
                    self.text.push('\r');
                    self.cursor_position = CursorPosition {
                        x: 1,
                        y: self.cursor_position.y + 1,
                    };
                } else {
                    self.cursor_position = CursorPosition {
                        x: self.cursor_position.x + 1,
                        y: self.cursor_position.y,
                    };
                }
                self.text.push(a);
            }
            Key::Backspace => {
                if let Some(ch) = self.text.pop() {
                    if ch == '\n' {
                        self.text.pop();
                        self.cursor_position = CursorPosition {
                            x: 1 + self.text.split('\n').last().unwrap().len() as u16,
                            y: self.cursor_position.y - 1,
                        };
                    } else {
                        self.cursor_position = CursorPosition {
                            x: self.cursor_position.x - 1,
                            y: self.cursor_position.y,
                        };
                    }
                }
            }

            _ => {}
        }
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<()> {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        stdout().flush()?;

        debug!("CURSOR : {:?}", self.cursor_position);
        debug!("TEXT : {}", self.text);
        print!("{}", self.text);
        print!(
            "{}",
            termion::cursor::Goto(self.cursor_position.x, self.cursor_position.y)
        );
        stdout().flush()?;

        Ok(())
    }

    fn die(e: &anyhow::Error) {
        Self::clear_screen();
        panic!("{}", e);
    }

    fn read_key() -> Result<Key, Error> {
        let mut it = stdin().keys();
        loop {
            if let Some(key) = it.next() {
                return key;
            }
        }
    }
}
