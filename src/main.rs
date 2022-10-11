use std::ops::RangeBounds;
use std::time::Duration;
use std::thread::sleep;

// a simple command line parsing program
use clap::Parser;

// search for a pattern in a file and display the lines that have it
#[derive(Parser)]
struct Cli {
    // the pattern to look for
    pattern: String,
    // the path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    // let result = std::fs::read_to_string("test.txt");
    // let content = match result {
    //     Ok(content) => { content }
    //     Err(error) => { panic!("Problem: {}", error); }
    // };
    // println!("File contents: {}", content);

    status_indicator();

}

fn status_indicator() {
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        do_hard_work();
        pb.println(format!("[+] finished with #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}

fn do_hard_work() {
    sleep(Duration::from_millis(2));
}