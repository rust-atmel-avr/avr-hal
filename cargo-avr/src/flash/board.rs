use std::path::PathBuf;
use anyhow::Result;

use crate::util::{avrdude::flash_binary, options::BoardOptions};

/// Build a target and flash the binary to a board
///
/// This is a high-level flashing command intended for used with off-the-shelf boards such as Arduino Uno.
/// 
/// If you are doing low-level development, look at the `cargo avr flash chip` command instead.
#[derive(clap::Parser)]
#[derive(Debug)]
pub struct Command {
    #[command(flatten)]
    board_opts: BoardOptions,

    pub binary: PathBuf,
}

impl Command {
    pub fn run(self) -> Result<()> {
        let programmer_opts = self.board_opts.into_programmer_options()?;
        flash_binary(&programmer_opts, &self.binary)
    }
}
