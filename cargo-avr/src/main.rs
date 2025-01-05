use clap::Parser;
use anyhow::Result;
use tracing_subscriber::FmtSubscriber;

mod flash;
mod attach;
mod util;

// Subcommands of `cargo avr`
#[derive(clap::Subcommand, Debug)]
enum Subcommand {
    Flash(flash::Command),
    Attach(attach::Command),
}

#[derive(clap::Parser, Debug)]
pub struct AvrCommand {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

impl AvrCommand {
    pub fn run(self) -> Result<()> {
        match self.subcommand {
            Subcommand::Flash(cmd) => cmd.run(),
            Subcommand::Attach(cmd) => cmd.run(),
        }
    }
}

// Top-level parsing
#[derive(clap::Subcommand)]
enum CargoCommand {
    Avr(AvrCommand),
}

#[derive(clap::Parser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: CargoCommand,
}

fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let args = Cli::parse();
    match args.subcommand {
        CargoCommand::Avr(cmd) => cmd.run(),
    }
}

