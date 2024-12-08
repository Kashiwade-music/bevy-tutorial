use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub main_config: MainConfig,
    pub theme: Theme,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MainConfig {
    pub midi_file_path: String,
    pub window_height: u32,
    pub window_width: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Theme {
    pub background_hex: String,
    pub note_channel_1_hex: String,
    pub main_text_color: String,
    pub secondary_text_color: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            main_config: MainConfig {
                midi_file_path: "C:\\Windows\\Media\\onestop.mid".to_string(),
                window_height: 1080,
                window_width: 1920,
            },
            theme: Theme {
                background_hex: "#260819".to_string(),
                note_channel_1_hex: "#FF0000".to_string(),
                main_text_color: "#FFFFFF".to_string(),
                secondary_text_color: "#FF0000".to_string(),
            },
        }
    }
}

pub fn load_config() -> io::Result<Config> {
    let config_path = Path::new("config.toml");

    if config_path.exists() {
        // config.toml が存在する場合は読み込む
        let content = fs::read_to_string(config_path)?;
        let config: Config =
            toml::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(config)
    } else {
        // config.toml が存在しない場合はデフォルト値でファイルを作成する
        let default_config = Config::default();
        save_config(&default_config)?;
        Ok(default_config)
    }
}

pub fn save_config(config: &Config) -> io::Result<()> {
    let config_path = Path::new("config.toml");
    let toml_content =
        toml::to_string(config).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    fs::write(config_path, toml_content)?;
    Ok(())
}
