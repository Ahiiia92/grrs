use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
pub struct Cli {
    /// The pattern to look for
    #[arg(name = "pattern", short, long)]
    pub pattern: String,
    /// The path to the file to read
    #[arg(name = "path", long, default_value = ".")]
    pub path: std::path::PathBuf,
}

pub fn find_matches(args: &Cli, mut writer: impl std::io::Write) -> std::io::Result<()> {
    let f = File::open(&args.path)?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let line = line?;
        if line.contains(&args.pattern) {
            writeln!(writer, "{}", line).unwrap();
        }
    }

    Ok(())
}
