use std::{num::NonZeroU32, path::{Path, PathBuf}};
use anyhow::Result;
use merge::Merge;

use crate::util::avr_libc::device_name_from_binary;


#[derive(clap::Parser)]
#[derive(Debug)]
/// AVR programmer configuration
pub struct ProgrammerOptions {
    /// The name of the programmer you are using to flash, such as `stk500`. Run `avrdude -c'?'` to see supported options.
    #[arg(long, env="CARGO_AVR_PROGRAMMER")]
    pub name: Option<String>,
    /// The path to the serial device used to communicate with the programmer. Autodetected if possible when not specified
    #[arg(long, env="CARGO_AVR_PROGRAMMER_PORT")]
    pub port: Option<PathBuf>,
    /// Baud rate for the serial connection to the programmer. Autodetected if possible when not specified
    #[arg(long, env="CARGO_AVR_PROGRAMMER_BAUDRATE")]
    pub baudrate: Option<NonZeroU32>,
    /// If set, erase the chip before flashing
    #[arg(long, env="CARGO_AVR_PROGRAMMER_ERASE_CHIP", value_parser = clap::builder::BoolishValueParser::new())]
    pub erase_chip: Option<bool>,
}

impl ProgrammerOptions {
    pub fn into_avrdude_options(&self, binary: impl AsRef<Path>) -> Result<(ravedude::BoardAvrdudeOptions, Option<PathBuf>)>{
        // Retrieve the chip name from the binary
        let chip = device_name_from_binary(binary)?;

        let avrdude_opts = ravedude::BoardAvrdudeOptions {
            programmer: self.name.clone(),
            partno: Some(chip),
            baudrate: Some(self.baudrate),
            do_chip_erase: self.erase_chip,
        };

        Ok((avrdude_opts, self.port.clone()))
    }

    pub fn into_attach_options(&self) -> AttachOptions {
        return AttachOptions {
            protocol: AttachProtocol::Serial,
            serial_opts: SerialAttachOptions {
                port: self.port.clone(),
                baudrate: self.baudrate
            }
        }
    }
}

#[derive(clap::Parser)]
#[derive(Debug)]
pub struct BoardOptions {
    /// The name of the board you are flashing
    pub board: String,

    #[command(flatten)]
    pub programmer_opts: ProgrammerOptions,
}

impl BoardOptions {
    pub fn into_programmer_options(self) -> Result<ProgrammerOptions>{
        let mut board = ravedude::get_board_from_name(&self.board)?;

        let avrdude_opts = board
            .avrdude
            .take()
            .ok_or_else(|| anyhow::anyhow!("board has no avrdude options"))?;

        let port = match self.programmer_opts.port {
            Some(port) => Ok(Some(port)),
            None => match board.guess_port() {
                Some(Ok(port)) => Ok(Some(port)),
                Some(Err(err)) => Err(err),
                None => Ok(None)
            }
        }?;

        Ok(ProgrammerOptions {
            name: avrdude_opts.programmer,
            port: port,
            baudrate: avrdude_opts.baudrate.flatten(),
            erase_chip: avrdude_opts.do_chip_erase,
        })

    }
}
