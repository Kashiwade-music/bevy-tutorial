use crate::global_vars::Config;
use std::fs;
use std::io::{self};
use std::path::Path;

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
