#![warn(clippy::pedantic)]

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
/// Feedback character legend:
///
/// Initial: 0
///
/// Gray: 1
///
/// Yellow: 2
///
/// Green: 3
///
/// Example:
///
/// -----:00000,arose:31112,amend:31211
pub struct Args {
    /// Play state
    #[clap(global = true)]
    pub state: String,

    /// Show number of results
    #[clap(short, long)]
    pub count: bool,

    /// Blocked words
    #[clap(short, long)]
    pub blocked: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let history = retrun::get_guesses(&args.state);
    let filtered_data = retrun::algorithm::guess(&history, args.blocked);
    render(filtered_data.0, args.count.then(|| filtered_data.1));
}

fn render(word: &str, word_count: Option<usize>) {
    if let Some(word_count) = word_count {
        println!("{} ({})", word, word_count);
    } else {
        println!("{}", word);
    }
}
