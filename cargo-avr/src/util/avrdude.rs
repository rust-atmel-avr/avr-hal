use std::path::Path;
use anyhow::Result;

use super::options::ProgrammerOptions;

pub fn flash_binary(programmer_opts: &ProgrammerOptions, artifact_path: &Path) -> Result<()> {
    let (avrdude_opts, port) = programmer_opts.into_avrdude_options(artifact_path)?;
    let mut avrdude = ravedude::Avrdude::run(&avrdude_opts, port, artifact_path, true)?;
    avrdude.wait()
}
