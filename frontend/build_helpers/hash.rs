use std::process::Command;

pub fn set_build_hash() {
    if let Some(hash) = get_git_hash().or_else(get_timestamp_hash) {
        println!("cargo:rustc-env=BUILD_HASH={}", hash);
    } else {
        println!("cargo:rustc-env=BUILD_HASH=unknown");
    }
}

fn get_git_hash() -> Option<String> {
    if let Ok(output) = Command::new("git")
        .args(["rev-parse", "--short=8", "HEAD"])
        .output()
        && let Ok(hash) = String::from_utf8(output.stdout)
    {
        return Some(hash.trim().to_string());
    }
    None
}

fn get_timestamp_hash() -> Option<String> {
    if let Ok(output) = Command::new("date").arg("+%s").output()
        && let Ok(timestamp) = String::from_utf8(output.stdout)
    {
        let hash = format!("{:08x}", timestamp.trim().parse::<u64>().ok()? % 0xFFFFFFFF);
        return Some(hash);
    }
    None
}
