use rand::Rng;
use std::error::Error;
use std::process;
use rand::prelude::ThreadRng;
use structopt::StructOpt;

const MIN_LENGTH: usize = 8;

struct Pwgen {
    words: Vec<String>,
    rng: ThreadRng,
}

impl Pwgen {

    pub fn new(words: Vec<String>) -> Pwgen {
        let rng = rand::thread_rng();
        Pwgen{ words, rng }
    }

    pub fn generate_password(&mut self, max_len: usize, max_words: u8, max_tries: usize, verbose: bool) -> String {
        let mut pwd = String::new();
        let mut curr_words = 0;

        for curr_try in 1..max_tries {
            let index = self.rng.gen_range(0..self.words.len()) as usize;
            let word = self.words[index].clone();

            if verbose {
                println!("curr_try: {:2}, curr_words: {:2}, max_words: {:2}, max_len: {:2}, curr_len: {:2}, pwd: {:32}, word: {:32}", curr_try, curr_words, max_words, max_len, pwd.len(), pwd, word);
            }

            // The extra two characters we reserve here are for the two-digit digit suffix.
            if (word.len() + 1) > (max_len - pwd.len() - 2) {
                continue;
            }

            if pwd.len() == 0 {
                pwd.push_str(&word[0..1].to_uppercase());
                pwd.push_str(&word[1..]);
            } else {
                pwd.push_str(word.as_str());
            }

            curr_words += 1;

            pwd.push('-');

            if curr_words >= max_words || max_len - pwd.len() <= 4 {
                break;
            }
        }

        let suffix = self.rng.gen_range(0..100) as usize;
        pwd.push_str(format!("{:02}", suffix).as_str());

        if verbose {
            println!("pwd: {}", pwd);
        }

        pwd
    }
}

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
    let words = words.lines().map(|s| s.to_owned()).collect();

    let mut pwgen = Pwgen::new(words);
    let pwd = pwgen.generate_password(args.max_len, 4, 20, args.verbose);

    if args.omit_newline {
        print!("{}", pwd);
    } else {
        println!("{}", pwd);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Pwgen;

    fn init() -> Pwgen {
        let bytes = include_bytes!("../words.txt");
        let words = String::from_utf8_lossy(bytes);
        let words = words.lines().map(|s| s.to_string()).collect();
        Pwgen::new(words)
    }

    #[test]
    pub fn test() {
        let mut pwgen = init();
        pwgen.generate_password(32, 5,20, true);
    }
}
