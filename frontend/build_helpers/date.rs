pub fn set_build_date() {
    if let Some(date) = get_system_date() {
        println!("cargo:rustc-env=BUILD_DATE={}", date);
    } else {
        println!("cargo:rustc-env=BUILD_DATE=unknown");
    }
}

fn get_system_date() -> Option<String> {
    if let Ok(output) = std::process::Command::new("date").arg("+%Y-%m-%d").output()
        && let Ok(date_str) = String::from_utf8(output.stdout)
    {
        return Some(date_str.trim().to_string());
    }
    None
}
