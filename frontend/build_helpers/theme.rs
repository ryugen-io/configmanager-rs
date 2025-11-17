use std::fs;

pub fn load_theme_config() {
    let theme_content = fs::read_to_string("../sys/theme/theme.toml")
        .expect("Failed to read ../sys/theme/theme.toml");
    let theme: toml::Value = toml::from_str(&theme_content).expect("Failed to parse theme.toml");

    let colors = theme
        .get("colors")
        .and_then(|c| c.as_table())
        .expect("Missing [colors] section in theme.toml");

    for (name, value) in colors {
        if let Some(rgb) = parse_rgb_array(value) {
            set_theme_color_env(name, rgb);
        }
    }
}

fn parse_rgb_array(value: &toml::Value) -> Option<(u8, u8, u8)> {
    let rgb_array = value.as_array()?;
    if rgb_array.len() != 3 {
        return None;
    }

    let r = rgb_array[0].as_integer()? as u8;
    let g = rgb_array[1].as_integer()? as u8;
    let b = rgb_array[2].as_integer()? as u8;

    Some((r, g, b))
}

fn set_theme_color_env(name: &str, (r, g, b): (u8, u8, u8)) {
    let env_name = format!("THEME_COLOR_{}", name.to_uppercase());
    println!("cargo:rustc-env={}={},{},{}", env_name, r, g, b);
}
