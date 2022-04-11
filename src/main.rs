use clap::{crate_authors, crate_description, crate_name, crate_version};

fn main() {
    let matches = clap::App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            clap::Arg::with_name("numwords")
                .long("number-of-words")
                .short("n")
                .takes_value(true)
                .default_value("6"),
        )
        .arg(
            clap::Arg::with_name("path")
                .long("wordlist-path")
                .short("p")
                .takes_value(true)
                .default_value("wordlist.txt"),
        )
        .get_matches();

    let wordlist_path = matches.value_of("path").unwrap();
    match load_words(wordlist_path) {
        Ok(wordlist) => {
            println!("wordlist length: {}", wordlist.len());

            let numwords = matches.value_of("numwords").unwrap().parse::<u8>().unwrap();

            println!("{}", diceware::generate_password(&wordlist, numwords));
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
