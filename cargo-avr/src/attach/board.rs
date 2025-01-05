use anyhow::Result;

use crate::util::{console::attach_console, options::{AttachOptions, AttachProtocol, BoardOptions}};

#[derive(clap::Parser)]
#[derive(Debug)]
pub struct Command {
    #[command(flatten)]
    board_opts: BoardOptions,

    #[clap(flatten)]
    attach_opts: AttachOptions,
}

impl Command {
    pub fn run(self) -> Result<()> {
        match self.attach_opts.protocol {
            AttachProtocol::None => Ok(()),
            AttachProtocol::Serial => attach_console(&self.attach_opts.serial_opts),
        }
    }
}
