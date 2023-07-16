use anyhow::Result;
use log::debug;
use std::io::{self, stdout, Write};
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
            text: String::new(),
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
            self.refresh_screen()?;
            self.process_keypress()?;
        }
    }

    fn quit() -> Result<()> {
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
                    self.cursor_position.x = 1;
                    self.cursor_position.y += 1;
                } else {
                    self.cursor_position.x += 1;
                }
                self.text.push(a);
            }
            Key::Backspace => {
                if let Some(ch) = self.text.pop() {
                    if ch == '\n' {
                        self.text.pop();
                        self.cursor_position.x = 1 + self.text.split('\n').last().unwrap().len() as u16;
                        self.cursor_position.y -= 1;
                    } else {
                        self.cursor_position.x -= 1;
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

    fn read_key() -> Result<Key> {
        let stdin = io::stdin();
        let mut keys = stdin.keys();
        loop {
            if let Some(key) = keys.next() {
                return key.map_err(Into::into);
            }
        }
    }
}
