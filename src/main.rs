use rand::prelude::ThreadRng;
use rand::Rng;
use std::error::Error;
use std::process;
use structopt::StructOpt;

const MIN_LENGTH: usize = 8;

struct Pwgen {
    words: Vec<String>,
    rng: ThreadRng,
}

impl Pwgen {
    pub fn new(words: Vec<String>) -> Pwgen {
        let rng = rand::thread_rng();
        Pwgen { words, rng }
    }

    pub fn generate_password(
        &mut self,
        max_len: usize,
        max_words: usize,
        max_tries: usize,
        alliterate: bool,
        verbose: bool,
    ) -> String {
        let mut pwd = String::new();
        let mut curr_words = 0;
        let mut first_letter = String::new();

        for curr_try in 1..max_tries {
            let index = self.rng.gen_range(0..self.words.len());
            let word = self.words[index].clone();

            if verbose {
                println!("curr_try: {:2}, first_letter: {}, curr_words: {:2}, max_words: {:2}, max_len: {:2}, curr_len: {:2}, pwd: {:32}, word: {:32}", curr_try, first_letter, curr_words, max_words, max_len, pwd.len(), pwd, word);
            }

            // The extra two characters we reserve here are for the two-digit digit suffix.
            if (word.len() + 1) > (max_len - pwd.len() - 2) {
                continue;
            }

            let curr_first_letter = &word[0..1].to_uppercase().to_string();

            if pwd.is_empty() {
                pwd.push_str(curr_first_letter);
                pwd.push_str(&word[1..]);
                first_letter = curr_first_letter.to_string();
            } else {
                if alliterate && curr_first_letter.ne(&first_letter) {
                    // Make sure to only create passwords that maintain alliteration
                    continue;
                }

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
        default_value = "28",
        short = "m",
        long = "max-length"
    )]
    pub max_len: usize,
    #[structopt(help = "Generate passwords that start with the same letter", short = "a", long = "alliterate")]
    pub alliterate: bool,
    #[structopt(
        help = "Do not print (omit) the trailing newline character",
        short = "n",
        long = "newline-omit"
    )]
    pub omit_newline: bool,
    #[structopt(help = "Output logging", short = "v", long = "verbose")]
    pub verbose: bool,
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
    let pwd = pwgen.generate_password(args.max_len, 3, 1000, args.alliterate, args.verbose);

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
        pwgen.generate_password(32, 5, 20, true, true);
    }
}
