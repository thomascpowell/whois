use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
pub struct WhoisEntry {
    pub tld: String,
    pub server: String,
}

fn load_tld_map() -> Vec<WhoisEntry> {
    let file = File::open("data/tlds.csv").expect("failed to open tlds.csv");
    let reader = BufReader::new(file);
    reader
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            if line.starts_with('#') || line.trim().is_empty() {
                return None;
            }
            let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
            if parts.len() != 2 {
                return None;
            }
            Some(WhoisEntry {
                tld: parts[0].to_string(),
                server: parts[1].to_string(),
            })
        })
        .collect()
}

fn get_entry_for<'a>(domain: &str, entries: &'a [WhoisEntry]) -> Option<&'a WhoisEntry> {
    let tld = domain.rsplit('.').next()?.to_lowercase();
    entries.iter().find(|e| e.tld == tld)
}

pub fn get_server(domain: &str) -> Option<String> {
    let entries = load_tld_map();
    let entry = get_entry_for(domain, &entries)?;
    Some(format!("{}:43", entry.server))
}
