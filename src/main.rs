use rand::Rng;
use std::error::Error;
use std::process;
use structopt::StructOpt;

const MIN_LENGTH: usize = 8;

#[derive(Debug, StructOpt)]
#[structopt(about = "Generates passwords from a set of the most common English words.")]
pub struct Args {
    #[structopt(
        help = "Maximum length of the password",
        default_value = "32",
        short = "m",
        long = "max-length"
    )]
    pub max_len: usize,
    #[structopt(help = "Specify for verbosity", short = "v", long = "verbose")]
    pub verbose: bool,
    #[structopt(
        help = "Do not print (omit) the trailing newline character",
        short = "n",
        long = "newline-omit"
    )]
    pub omit_newline: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::from_args();

    if args.max_len < MIN_LENGTH {
        eprintln!("Please ensure -m/--max-length is >= {}.", MIN_LENGTH);
        process::exit(1);
    }

    let bytes = include_bytes!("../words.txt");
    let words = String::from_utf8_lossy(bytes);
    let lines: Vec<&str> = words.lines().collect();

    let mut available_len = args.max_len - 2; // max_words is also number of trailing dashes, 2 is the digit suffix

    let mut rando = rand::thread_rng();
    let mut words = vec![];

    loop {
        let index = rando.gen_range(0, lines.len());
        let word = lines[index];

        if args.verbose {
            println!("available_len: {}, candidate word: {}", available_len, word);
        }

        let word_len = word.len();

        if word_len > available_len {
            continue;
        }

        words.push(word);
        available_len -= word_len;

        if args.verbose {
            println!("Remaining available_len: {}", available_len);
        }

        if available_len <= 4 {
            break;
        }
    }

    let mut pwd = String::new();

    for (i, word) in words.iter().enumerate() {
        if args.verbose {
            println!("Element at position {}: {:?}", i, word);
        }

        if i == 0 {
            let first_letter = &word[0..1].to_uppercase();
            let rest_of_letters = &word[1..];

            if args.verbose {
                println!("first_letter: {}", first_letter);
                println!("rest_of_letters: {}", rest_of_letters);
            }

            pwd.push_str(first_letter);
            pwd.push_str(rest_of_letters);
        } else {
            pwd.push('-');
            pwd.push_str(word);
        }
    }

    let mut suffix;

    loop {
        suffix = rando.gen_range(0, 10);

        if suffix > 0 {
            break;
        }
    }

    pwd.push_str(format!("-{:02}", suffix).as_str());

    if args.omit_newline {
        print!("{}", pwd);
    } else {
        println!("{}", pwd);
    }

    Ok(())
}
