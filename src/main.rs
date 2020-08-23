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
        
            println!("{}", generate_password(&wordlist, numwords));
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

fn generate_password(wordlist: &Vec<String>, num_words: u8) -> String {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let mut password_words: Vec<String> = vec![];
    for _ in 0..num_words {
        let random_word_index = rng.gen_range(0, wordlist.len());
        let random_word = wordlist.get(random_word_index).unwrap();
        password_words.push(random_word.clone());
    }

    password_words.join(" ")
}
