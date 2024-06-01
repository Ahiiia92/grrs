use clap::Parser;
use grrs::find_matches;
use grrs::Cli;

fn main() -> std::io::Result<()> {
    let pb = indicatif::ProgressBar::new(100);
    let args = Cli::parse();
    find_matches(&args, &mut std::io::stdout())?;
    for i in 0..100 {
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");

    Ok(())
}
