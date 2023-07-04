use clap::{value_parser, Arg, Command};

fn main() {
    let matches = Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("numwords")
                .long("number-of-words")
                .short('n')
                .value_parser(value_parser!(u8))
                .default_value("6"),
        )
        .arg(
            Arg::new("path")
                .long("wordlist-path")
                .short('p')
                .default_value("wordlist.txt"),
        )
        .get_matches();

    let verbose = matches.get_flag("verbose");
    let wordlist_path = matches.get_one::<String>("path").unwrap();
    match load_words(wordlist_path) {
        Ok(wordlist) => {
            if verbose {
                eprintln!("wordlist length: {}", wordlist.len());
            }

            let numwords = matches.get_one::<u8>("numwords").unwrap();

            println!("{}", diceware::generate_password(&wordlist, *numwords));
        }
        Err(e) => {
            eprintln!("Failed to load word list: {}", e.to_string());
            std::process::exit(1);
        }
    }
}

fn load_words(wordlist_path: &str) -> std::io::Result<Vec<String>> {
    use std::io::BufRead;

    let file = std::fs::File::open(wordlist_path)?;
    std::io::BufReader::new(file).lines().collect()
}
