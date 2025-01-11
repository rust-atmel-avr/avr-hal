use std::path::PathBuf;

use anyhow::Result;
use merge::Merge;

use crate::util::options::AttachProtocol;
use crate::util::options::ProgrammerOptions;
use crate::util::options::AttachOptions;
use crate::util::avrdude::flash_binary;
use crate::util::console::attach_console;

/// Build a target, flash the binary to a chip, and attach to the running binary
///
/// This is a low-level flashing command for flashing bare chips. To flash your code to an
/// off-the-shelf board such as Arduino Uno, you use `cargo avr flash board` instead. 
#[derive(clap::Parser)]
#[derive(Debug)]
pub struct Command {
    #[clap(flatten)]
    programmer_opts: ProgrammerOptions,

    #[clap(flatten)]
    attach_opts: AttachOptions,

    pub binary: PathBuf,
}

impl Command {
    pub fn run(mut self) -> Result<()> {
        flash_binary(&self.programmer_opts, &self.binary)?;

        self.attach_opts.merge(self.programmer_opts.into_attach_options());

        match self.attach_opts.protocol {
            AttachProtocol::None => Ok(()),
            AttachProtocol::Serial => attach_console(&self.attach_opts.serial_opts),
        }
    }

}
