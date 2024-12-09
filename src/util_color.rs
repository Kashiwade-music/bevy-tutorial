use std::error::Error;

/// HEXカラーコードをSRGB値に変換する関数
pub fn hex_to_srgb(hex: &str) -> Result<Vec<f32>, Box<dyn Error>> {
    // HEXコードの先頭に '#' があれば取り除く
    let hex = hex.trim_start_matches('#');

    // HEXコードの長さに応じて処理
    match hex.len() {
        6 => {
            // RGB
            let r = u8::from_str_radix(&hex[0..2], 16)? as f32 / 255.0;
            let g = u8::from_str_radix(&hex[2..4], 16)? as f32 / 255.0;
            let b = u8::from_str_radix(&hex[4..6], 16)? as f32 / 255.0;
            Ok(vec![r, g, b])
        }
        8 => {
            // RGBA
            let r = u8::from_str_radix(&hex[0..2], 16)? as f32 / 255.0;
            let g = u8::from_str_radix(&hex[2..4], 16)? as f32 / 255.0;
            let b = u8::from_str_radix(&hex[4..6], 16)? as f32 / 255.0;
            let a = u8::from_str_radix(&hex[6..8], 16)? as f32 / 255.0;
            Ok(vec![r, g, b, a])
        }
        _ => Err("Invalid HEX color length".into()),
    }
}

pub fn adjust_color(
    base_color: &str,
    target_color: &str,
    adjust_index: u32,
    max_index: u32,
) -> Result<Vec<f32>, Box<dyn Error>> {
    let base_color = hex_to_srgb(base_color)?;
    let target_color = hex_to_srgb(target_color)?;

    let mut result = Vec::new();
    for i in 0..base_color.len() {
        let ratio = 1.0
            - f32::powf(
                f32::powf(0.1, 1.0 / (max_index as f32 - 1.0)),
                adjust_index as f32,
            );
        result.push(base_color[i] + (target_color[i] - base_color[i]) * ratio);
    }

    Ok(result)
}
