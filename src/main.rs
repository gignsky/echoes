// ARGS Parser
use clap::Parser;
// // TUI Stuff
// use color_eyre::Result;
// use crossterm::event::{self, Event};
// use ratatui::{DefaultTerminal, Frame};

#[derive(Parser, Debug)]
#[clap(author = "Maxwell Rupp", version, about)]
/// Application configuration
struct Args {
    /// a REQUIRED message to be passed as an argument
    #[arg()]
    message: Option<String>,
}

// Non-TUI Stuff
fn main() {
    let args = Args::parse();
    let message = match args.message {
        Some(msg) => msg,
        None => "No message provided".to_string(),
    };
    println!("{}", message);
}
