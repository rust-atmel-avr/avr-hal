pub mod chip;
pub mod board;

use anyhow::Result;

// Subcommands of `cargo avr flash`
#[derive(clap::Subcommand, Debug)]
enum Subcommand {
    Chip(chip::Command),
    Board(board::Command),
}

/// Build a target and flash the binary to a microcontroller
#[derive(clap::Parser, Debug)]
pub struct Command {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

impl Command {
    pub fn run(self) -> Result<()> {
        match self.subcommand {
            Subcommand::Chip(cmd) => cmd.run(),
            Subcommand::Board(cmd) => cmd.run(),
        }
    }
}
