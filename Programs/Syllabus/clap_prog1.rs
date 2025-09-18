use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author = "Your Name", version = "1.0", about = "Count ERROR/WARN/INFO lines in a log")]
struct Args {
    file: Option<String>,
}

fn main() {
    let args = Args::parse();

    match args.file {
        Some(file_path) => {
            let contents = fs::read_to_string(&file_path).expect("Failed to read file");
            let error_count = contents.lines().filter(|line| line.contains("ERROR")).count();
            let warn_count = contents.lines().filter(|line| line.contains("WARN")).count();
            let info_count = contents.lines().filter(|line| line.contains("INFO")).count();
            println!("ERROR lines: {}", error_count);
            println!("WARN lines: {}", warn_count);
            println!("INFO lines: {}", info_count);
        }
        None => println!("No file provided. Run with --help for usage."),
    }
}
