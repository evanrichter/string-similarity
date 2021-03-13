use std::path::PathBuf;

use colored::*;

/// Compare two strings using Levenshtein edit distance.
#[derive(argh::FromArgs)]
struct Args {
    /// file from which to load plaintext. "plaintext.txt" by default.
    #[argh(option, short = 'f', default = "PathBuf::from(\"plaintext.txt\")")]
    file: PathBuf,

    /// disable color output.
    #[argh(switch, short = 'n')]
    nocolor: bool,
}

fn main() -> anyhow::Result<()> {
    // parse arguments
    let args: Args = argh::from_env();

    // remove terminal colors if desired
    if args.nocolor {
        colored::control::set_override(false);
    }

    // alert where we are reading from
    let fileprint = format!("{:?}", args.file).green();
    println!("Reading plaintext from {}", fileprint);

    // check if file exists
    if !args.file.exists() {
        println!("Error: Plaintext file {} doesn't exist.", fileprint.red());
        std::process::exit(-1);
    }

    // load file to string
    let plaintext = std::fs::read_to_string(args.file)?;
    println!("Plaintext:\n{}", plaintext.blue());

    // ask for guess on stdin
    println!("Enter the guessed plaintext followed by a newline:");

    // read one line from stdin
    let stdin = std::io::stdin();
    let mut guess = String::new();
    stdin.read_line(&mut guess)?;

    // print what we read as guess
    println!("\nGuessed plaintext:\n{}", guess.green());

    // calculate Levenshtein edit distance
    let edit_distance = strsim::levenshtein(&plaintext, &guess);

    // calculate "percentage correct" using edit distance
    let correct = 100.0 - (100 * edit_distance) as f64 / plaintext.len() as f64;

    // print out score
    let score = if correct > 99.9 {
        format!("{:>3.2}", 100.0).green()
    } else if correct < 0.01 {
        format!("{:>3.2}", 0.0).red()
    } else {
        format!("{:>3.2}", correct).blue()
    };

    // print overall correctness
    println!("Correctness: {}", score);

    Ok(())
}
