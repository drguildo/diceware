pub fn generate_password(wordlist: &Vec<String>, num_words: u8) -> String {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let mut password_words: Vec<String> = vec![];
    for _ in 0..num_words {
        let random_word_index = rng.gen_range(0..wordlist.len());
        let random_word = wordlist.get(random_word_index).unwrap();
        password_words.push(random_word.clone());
    }

    password_words.join(" ")
}
