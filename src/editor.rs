use anyhow::{anyhow, Result};
use log::debug;
use std::io::stdout;
use std::io::{stdin, Error, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::terminal::Terminal;

pub struct Editor {
    terminal: Terminal,
    text: String,
    cursor_position: (u16, u16),
    save: bool,
    quit: bool,
}

impl Editor {
    pub fn default() -> Result<Self> {
        Ok(Self {
            terminal: Terminal::default()?,
            text: "".into(),
            cursor_position: (1, 1),
            save: false,
            quit: false,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            if self.quit {
                die(&anyhow::anyhow!("quit"));
                break Ok(());
            }
            if let Err(err) = self.refresh_screen() {
                die(&err);
            }
            if let Err(err) = self.process_keypress() {
                die(&err);
            }
        }
    }

    fn process_keypress(&mut self) -> Result<()> {
        let key = read_key()?;
        match key {
            Key::Ctrl('q') => self.quit = true,
            Key::Char(a) => {
                if a == '\n' {
                    self.text.push('\r');
                    self.cursor_position = (1, self.cursor_position.1 + 1);
                } else {
                    self.cursor_position = (self.cursor_position.0 + 1, self.cursor_position.1);
                }
                self.text.push(a);
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
            termion::cursor::Goto(self.cursor_position.0, self.cursor_position.1)
        );
        stdout().flush()?;

        Ok(())
    }
}

fn read_key() -> Result<Key, Error> {
    let mut it = stdin().keys();
    loop {
        if let Some(key) = it.next() {
            return key;
        }
    }
}

fn die(e: &anyhow::Error) {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    panic!("{}", e);
}
