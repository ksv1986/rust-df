use std::cmp::Ordering;

use util::shorten_lv;

#[derive(Debug)]
pub struct Stats {
    pub filesystem: String,
    pub size: u64,
    pub used: u64,
    pub avail: u64,
    pub percent: f64,
    pub mount: String,
    pub fsid: u64,
    score: usize,
}

impl Stats {
    pub fn new(fs: &str, size: u64, avail: u64, mount: &str, fsid: u64) -> Stats {
        let used = size - avail;
        let percent = 100.0 * used as f64 / size as f64;
        let score = score(fs);
        Stats {
            filesystem: shorten_lv(fs),
            size,
            avail,
            used,
            percent,
            mount: mount.to_string(),
            fsid,
            score,
        }
    }

    pub fn is_network(&self) -> bool {
        self.score == FS_NET
    }

    pub fn is_same(&self, other: &Stats) -> bool {
        self.fsid == other.fsid && self.size == other.size
    }
}

const FS_DEV: usize = 0;
const FS_DEFAULT: usize = 500;
const FS_NET: usize = 1000;

fn score(fs: &str) -> usize {
    if fs.contains(':') {
        FS_NET
    } else if fs.starts_with("/dev/") {
        FS_DEV + fs.len()
    } else {
        FS_DEFAULT
    }
}

impl Ord for Stats {
    fn cmp(&self, other: &Stats) -> Ordering {
        let cmp = self.score.cmp(&other.score);
        if cmp != Ordering::Equal {
            return cmp;
        }

        self.filesystem.cmp(&other.filesystem)
    }
}

impl PartialOrd for Stats {
    fn partial_cmp(&self, other: &Stats) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Stats {
    fn eq(&self, other: &Stats) -> bool {
        self.filesystem == other.filesystem
    }
}

impl Eq for Stats {}
