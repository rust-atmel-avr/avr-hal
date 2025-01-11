use anyhow::anyhow;

use anyhow::Result;

use crate::util::options::SerialAttachOptions;

pub fn attach_console (opts: &SerialAttachOptions) -> Result<()> {
    let port = match &opts.port {
        Some(port) => Ok(port),
        None => Err(anyhow!("Serial port is required; specify it with `--serial-port` or `CARGO_AVR_SERIAL_PORT`"))
    }?;

    let baudrate = match opts.baudrate {
        Some(baudrate) => Ok(baudrate.into()),
        None => Err(anyhow!("Serial baudrate is required; specify it with `--serial-baudrate` or `CARGO_AVR_SERIAL_BAUDRATE`"))
    }?;

    ravedude::attach_console(port.as_ref(), baudrate)
}

