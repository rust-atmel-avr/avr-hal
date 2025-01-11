use std::path::PathBuf;
use anyhow::Result;

use crate::util::{avrdude::flash_binary, options::ProgrammerOptions};

/// Build a target and flash the binary to a chip
///
/// This is a low-level flashing command. To flash your code to an off-the-shelf board such as
/// Arduino Uno, you should use `cargo avr flash board` instead. 
#[derive(clap::Parser)]
#[derive(Debug)]
pub struct Command {
    #[clap(flatten)]
    programmer_opts: ProgrammerOptions,

    pub binary: PathBuf,
}

impl Command {
    pub fn run(self) -> Result<()> {
        flash_binary(&self.programmer_opts, &self.binary)
    }

}
