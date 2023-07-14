use anyhow::Result;
use std::{
    cell::RefCell,
    io::{stdout, Stdout},
};
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Size {
    pub height: u16,
    pub width: u16,
}

pub struct Terminal {
    pub size: Size,
    pub stdout: RawTerminal<Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self> {
        let (width, height) = termion::terminal_size()?;
        Ok(Self {
            size: Size { height, width },
            // INFO: This _stdout should be alive for the lifetime of the editor. Later move it to
            // somewhere else
            stdout: stdout().into_raw_mode()?,
        })
    }

    // pub fn enable_raw_mode(&mut self) -> Result<()> {
    //     let stdout = stdout().into_raw_mode()?;
    //     self.stdout = Some(stdout);
    //     Ok(())
    // }
    //
    // pub fn disable_raw_mode(&mut self) -> Result<()> {
    //     if let Some(mut stdout) = self.stdout.take() {
    //         stdout.suspend_raw_mode()?;
    //         write!(stdout, "{}", termion::cursor::Show)?;
    //         stdout.flush()?;
    //     }
    //     Ok(())
    // }
}
