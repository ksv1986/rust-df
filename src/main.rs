extern crate clap;
extern crate colored;
extern crate nix;

mod stats;
mod util;

use clap::{Arg, Command};
use colored::*;
use nix::sys::statvfs::statvfs;
use std::cmp;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::process;

use stats::Stats;
use util::{bargraph, iec, is_virtual};

const FS_SPEC: usize = 0;
const FS_FILE: usize = 1;

fn main() {
    let matches = Command::new("rdf")
        .version("0.1.0")
        .author("Mike Sampson <mike@sda.io>")
        .arg(
            Arg::new("all")
                .long("all")
                .short('a')
                .help("Display all filesystems"),
        )
        .get_matches();

    let file = match File::open("/proc/mounts") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: Could not open /proc/mounts - {}", e);
            process::exit(1);
        }
    };
    let reader = BufReader::new(&file);

    let mut stats: Vec<Stats> = Vec::new();
    let mut max_width = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let fields: Vec<&str> = line.split_whitespace().collect();
                if !matches.is_present("all") && is_virtual(fields[FS_SPEC]) {
                    continue;
                }
                let statvfs = match statvfs(fields[FS_FILE]) {
                    Ok(s) => s,
                    Err(err) => {
                        eprintln!("Error: statvfs({}) failed: {}", fields[FS_FILE], err);
                        continue;
                    }
                };
                let size = statvfs.blocks() * statvfs.block_size();
                if size == 0 && !matches.is_present("all") {
                    continue;
                }
                let avail = statvfs.blocks_available() * statvfs.block_size();
                let s = Stats::new(
                    fields[FS_SPEC],
                    size,
                    avail,
                    fields[FS_FILE],
                    statvfs.filesystem_id(),
                );
                if stats.iter().any(|o| o.is_same(&s)) {
                    continue;
                }
                max_width = cmp::max(max_width, s.filesystem.len());
                stats.push(s);
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }

    stats.sort();

    let headers = [
        "Filesystem",
        "Size",
        "Used",
        "Avail",
        "Use%",
        "",
        "Mounted on",
    ];
    let headers: Vec<ColoredString> = headers.iter().map(|x| x.yellow()).collect();
    println!(
        "{:width$} {:>5} {:>5} {:>5} {:>5} {:20} {}",
        headers[0],
        headers[1],
        headers[2],
        headers[3],
        headers[4],
        headers[5],
        headers[6],
        width = max_width
    );

    for stat in stats {
        let fs = if stat.is_network() {
            stat.filesystem.cyan()
        } else {
            stat.filesystem.normal()
        };
        let percent = if stat.percent.is_nan() {
            "    -".to_string()
        } else {
            format!("{:>5.1}", stat.percent)
        };
        println!(
            "{:width$} {:>5} {:>5} {:>5} {} {:20} {}",
            fs,
            iec(stat.size),
            iec(stat.used),
            iec(stat.avail),
            percent,
            bargraph(stat.percent),
            stat.mount,
            width = max_width
        );
    }
}
