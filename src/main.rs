use std::error::Error;
use rand::Rng;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Download binaries.")]
pub struct App {
    #[structopt(about = "Maximum length of the password", default_value = "64", short = "m", long = "max-length")]
    pub max_len: usize,
    #[structopt(about = "Specify for verbosity", short = "v", long = "verbose")]
    pub verbose: bool,
    #[structopt(about = "Do not print the trailing newline character", short = "n", long = "newline-omit")]
    pub omit_newline: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::from_args();
    let bytes = include_bytes!("../words.txt");
    let words = String::from_utf8_lossy(bytes);
    let lines: Vec<&str> = words.lines().collect();

    // let max_len = 16;
    // let verbose = false;

    let max_words = 4;
    let max_tries = 20;
    let mut available_len = app.max_len - 2; // max_words is also number of trailing dashes, 2 is the digit suffix

    let mut rando = rand::thread_rng();
    let mut words= vec![];

    for _i in 1..max_tries {
        let index = rando.gen_range(0, lines.len());
        let word = lines[index];

        if app.verbose {
            println!("available_len: {}, candidate word: {}", available_len, word);
        }

        let word_len = word.len();

        if word_len > available_len {
            continue
        }

        words.push(word);
        available_len -= word_len;

        if app.verbose {
            println!("Remaining available_len: {}", available_len);
        }

        if words.len() >= max_words || available_len <= 4 {
            break;
        }
    }

    let mut pwd = String::new();

    for (i, word) in words.iter().enumerate() {
        if app.verbose {
            println!("Element at position {}: {:?}", i, word);
        }

        if i == 0 {
            let first_letter = &word[0..1].to_uppercase();
            let rest_of_letters = &word[1..];

            if app.verbose {
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
            break
        }
    }

    pwd.push_str(format!("-{:02}", suffix).as_str());

    if app.omit_newline{
        print!("{}", pwd);
    } else {
        println!("{}", pwd);
    }

    Ok(())
}
