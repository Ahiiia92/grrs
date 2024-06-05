use clap::Parser;
use grrs::find_matches;
use grrs::Cli;

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    find_matches(&args, &mut std::io::stdout())?;
    Ok(())
}
