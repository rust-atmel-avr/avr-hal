use anyhow::Result;

use crate::util::{console::attach_console, options::{ AttachProtocol, SerialAttachOptions}};

/// Attach a console to a chip
///
/// This is a low-level console command that requires you to specify the serial port on which the
/// chip is connected and the baud rate. You only need it if you are doing low-level development
/// with bare microcontrollers.
/// 
/// If you are just trying to attach to a serial console to an off-the-shelf board such as Arduino
/// Uno, you should use `cargo avr attach board` instead. 
#[derive(clap::Parser)]
#[derive(Debug)]
/// Options for attaching to a device
pub struct AttachOptions {
    /// The protocol used to attach to the device
    pub protocol: AttachProtocol,

    /// Serial options
    #[clap(flatten)]
    pub serial_opts: SerialAttachOptions,
}

#[derive(clap::Parser)]
#[derive(Debug)]
pub struct Command {
    #[command(flatten)]
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
