mod levenshtein;

use std::io;
use std::path::Path;

fn parse_gb(filename: &Path) -> io::Result<String> {
    let s = std::fs::read_to_string(filename)?;
    Ok(s.split('\n')
        .map(|s| s[10..].split(' ').collect::<String>())
        .collect::<String>())
}

fn main() -> io::Result<()> {
    let cov = parse_gb(Path::new("cov.txt"))?;
    let sars = parse_gb(Path::new("sars.txt"))?;

    println!(
        "similarity: {}",
        1.0 - levenshtein::levenshtein_dist(cov.as_bytes(), sars.as_bytes()) as f64
            / cov.len() as f64
    );

    Ok(())
}
