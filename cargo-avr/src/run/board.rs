use std::path::PathBuf;

use anyhow::Result;
use merge::Merge;

use crate::util::options::{AttachOptions, AttachProtocol, BoardOptions};
use crate::util::avrdude::flash_binary;
use crate::util::console::attach_console;

/// Build a target, flash the binary to a board, and run it with an attached serial console
///
/// This is a high-level command intended for used with off-the-shelf boards such as Arduino Uno.
/// 
/// If you are doing low-level development, look at the `cargo avr run chip` command instead.
#[derive(clap::Parser)]
#[derive(Debug)]
pub struct Command {
    #[command(flatten)]
    board_opts: BoardOptions,

    #[clap(flatten)]
    attach_opts: AttachOptions,

    pub binary: PathBuf,
}

impl Command {
    pub fn run(mut self) -> Result<()> {
        let programmer_opts = self.board_opts.into_programmer_options()?;
        flash_binary(&programmer_opts, &self.binary)?;

        self.attach_opts.merge(programmer_opts.into_attach_options());

        match self.attach_opts.protocol {
            AttachProtocol::None => Ok(()),
            AttachProtocol::Serial => attach_console(&self.attach_opts.serial_opts),
        }
    }
}
