
use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn fgroups() {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    for line in io::stdin().lock().lines() {
        let line = line.unwrap().trim().to_string();

        if line.is_empty() {
            continue;
        }

        let mut parts = line.splitn(2, char::is_whitespace);
        let fingerprint = parts.next().unwrap_or("");
        let name = parts.next().unwrap_or("").trim();

        if fingerprint.is_empty() || name.is_empty() {
            eprintln!("Invalid input");
            continue;
        }
        if fingerprint.len() > 512 {
            eprintln!("Invalid input");
            continue;
        }
        groups.entry(fingerprint.to_string()).or_default().push(name.to_string());
    }

    let mut first = true;

    for (_fingerprint, names) in groups {
        if names.len() < 2 {
            continue;
        }

        if !first {
            println!();
        }
        first = false;

        for name in names {
            println!("{}", name);
        }
    }
}

fn main() {
    fgroups();
}