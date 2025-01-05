use std::num::NonZeroU32;
use std::path::Path;
pub use crate::board::{config::{BoardConfig, ResetOptions}, get_board_from_name};

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct RavedudeConfig {
    #[serde(rename = "general")]
    pub general_options: RavedudeGeneralConfig,

    #[serde(rename = "board")]
    pub board_config: Option<BoardConfig>,
}

impl RavedudeConfig {
    pub fn from_args(args: &crate::Args) -> anyhow::Result<Self> {
        Ok(Self {
            general_options: RavedudeGeneralConfig {
                open_console: args.open_console,
                serial_baudrate: match args.baudrate {
                    Some(serial_baudrate) => Some(
                        NonZeroU32::new(serial_baudrate)
                            .ok_or_else(|| anyhow::anyhow!("baudrate must not be 0"))?,
                    ),
                    None => None,
                },
                port: args.port.clone(),
                reset_delay: args.reset_delay,
                board: args.legacy_board_name().clone(),
            },
            board_config: Default::default(),
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct RavedudeGeneralConfig {
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub open_console: bool,
    pub serial_baudrate: Option<NonZeroU32>,
    pub port: Option<std::path::PathBuf>,
    pub reset_delay: Option<u64>,
    pub board: Option<String>,
}

impl RavedudeGeneralConfig {
    /// Apply command-line overrides to this configuration. Command-line arguments take priority over Ravedude.toml
    pub fn apply_overrides_from(&mut self, args: &crate::Args) -> anyhow::Result<()> {
        if args.open_console {
            self.open_console = true;
        }
        if let Some(serial_baudrate) = args.baudrate {
            self.serial_baudrate = Some(
                NonZeroU32::new(serial_baudrate)
                    .ok_or_else(|| anyhow::anyhow!("baudrate must not be 0"))?,
            );
        }
        if let Some(port) = args.port.clone() {
            self.port = Some(port);
        }
        if let Some(reset_delay) = args.reset_delay {
            self.reset_delay = Some(reset_delay);
        }
        Ok(())
    }
}

pub fn get_config_from_board_name(board_name: &str) -> anyhow::Result<RavedudeConfig> {
    Ok(RavedudeConfig {
        board_config: Some(get_board_from_name(board_name)?),
        ..Default::default()
    })
}

pub fn get_config_from_manifest(manifest_path: &Path) -> anyhow::Result<RavedudeConfig> {
    Ok({
        let file_contents = std::fs::read_to_string(manifest_path)
            .map_err(|err| anyhow::anyhow!("Ravedude.toml read error:\n{}", err))?;

        let mut board: RavedudeConfig = toml::from_str(&file_contents)
            .map_err(|err| anyhow::anyhow!("invalid Ravedude.toml:\n{}", err))?;

        if let Some(board_config) = board.board_config.as_ref() {
            if let Some(board_name) = board.general_options.board.as_deref() {
                anyhow::bail!(
                    "can't both have board in [general] and [board] section; set inherit = \"{}\" under [board] to inherit its options",
                    board_name
                )
            }
            if let Some(inherit) = board_config.inherit.as_deref() {
                let base_board = get_config_from_board_name(inherit)?.board_config.unwrap();
                board.board_config = Some(board.board_config.take().unwrap().merge(base_board));
            }
        } else if let Some(board_name) = board.general_options.board.as_deref() {
            let base_board = get_config_from_board_name(board_name)?.board_config.unwrap();
            board.board_config = Some(base_board);
        }
        board
    })
}
