use colored::*;

use std::fs::canonicalize;

// http://stackoverflow.com/questions/5194057/better-way-to-convert-file-sizes-in-python
pub fn iec(n: u64) -> String {
    let units = ["", "k", "M", "G", "T", "P", "E", "Z", "Y"];

    let i = (n as f64).log(1024_f64).floor() as u32;
    let p = 1024_u64.pow(i);
    let s = (n as f64) / (p as f64);
    format!("{:.0}{}", s, units[i as usize])
}

// valid VG/LV characters are: a-z A-Z 0-9 + _ . -
// we use this fact and replace -- with #
// split on - and then switch # back to -
pub fn shorten_lv(path: &str) -> String {
    if path.starts_with("/dev/mapper/") {
        if let Ok(real) = canonicalize(path) {
            if let Some(spath) = real.to_str() {
                return spath.into();
            }
        }
    }

    path.to_string()
}

pub fn bargraph(mut percent: f64) -> String {
    if percent.is_nan() {
        percent = 0.0;
    }
    let chars = "■■■■■■■■■■■■■■■■■■■■";
    let s1 = (percent / 10.0).round() as usize * 2;
    let s2 = 20 - s1;
    let bar1 = chars.chars().take(s1).collect::<String>();
    let bar1 = if percent > 90.0 {
        bar1.red()
    } else if percent > 75.0 {
        bar1.yellow()
    } else {
        bar1.green()
    };
    let bar2 = chars.chars().take(s2).collect::<String>().white().dimmed();
    format!("{}{}", bar1, bar2)
}

pub fn is_virtual(fs: &str) -> bool {
    match fs {
        "dev" | "devtmpfs" | "efivarfs" | "fuse-overlayfs" | "portal" | "run" | "shm" | "tmpfs" => {
            true
        }
        _ => fs.starts_with("/dev/loop") || fs.starts_with("systemd-"),
    }
}
