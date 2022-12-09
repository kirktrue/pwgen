use rand::Rng;
use std::error::Error;
use std::process;
use structopt::StructOpt;

const MIN_LENGTH: usize = 8;

struct Pwgen {
    lines: Vec<String>,
}

impl Pwgen {
    pub fn generate_password(self, max_len: usize, verbose: bool) -> String {
        let mut pwd = String::new();
        let mut rando = rand::thread_rng();

        loop {
            let index = rando.gen_range(0..self.lines.len()) as usize;
            let word = self.lines[index].clone();
            let token_len = word.len() + 1;

            if verbose {
                println!("pwd: {}, word: {}, token_len: {}", pwd, word, token_len);
            }

            // The extra two characters we reserve here are for the two-digit digit suffix.
            if token_len > (max_len - pwd.len() - 2) {
                println!("max_len: {}, pwd (len): {}, token_len: {}, continuing...", max_len, pwd.len(), token_len);
                continue;
            }

            if pwd.len() == 0 {
                pwd.push_str(&word[0..1].to_uppercase());
                pwd.push_str(&word[1..]);
            } else {
                pwd.push_str(word.as_str());
            }

            pwd.push('-');

            if verbose {
                println!("max_len: {}, pwd: {}, word: {}, token_len: {}", max_len, pwd, word, token_len);
            }

            if max_len - pwd.len() <= 2 {
                println!("max_len: {}, pwd (len): {}, breaking...", max_len, pwd.len());
                break;
            }
        }

        loop {
            let range = 0..10;
            let suffix:u32 = rando.gen_range(range);

            if suffix > 0 {
                pwd.push_str(format!("{:02}", suffix).as_str());
                break;
            }
        }

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
    let lines = words.lines().map(|s| s.to_owned()).collect();

    let pwgen = Pwgen{lines};
    let pwd = pwgen.generate_password(args.max_len, args.verbose);

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
        let lines = words.lines().map(|s| s.to_string()).collect();
        Pwgen { lines }
    }

    #[test]
    pub fn test() {
        let pwgen = init();
        let password = pwgen.generate_password(32, true);
        assert_eq!(32, password.len());
    }
}
