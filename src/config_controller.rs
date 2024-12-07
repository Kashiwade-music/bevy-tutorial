use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub midi_file_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            midi_file_path: "C:\\Windows\\Media\\onestop.mid".to_string(),
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
        let toml_content = toml::to_string(&default_config)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let mut file = fs::File::create(config_path)?;
        file.write_all(toml_content.as_bytes())?;
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
