use std::{
    fs::File,
    io::{self, stdin, BufRead, BufReader},
    path::Path,
};

fn read_words_from_file(filename: &str) -> io::Result<Vec<String>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let words = reader.lines().filter_map(|line| line.ok()).collect();
    Ok(words)
}

fn main() {
    let words = match read_words_from_file("small.txt") {
        Ok(word_list) => {
            println!("Words in file");
            for word in &word_list {
                println!("{word}")
            }
            word_list
        }
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("failed to read line");
        let word_list: Vec<String> = input.trim().split_whitespace().map(String::from).collect();
        let missing_words: Vec<usize> = word_list
            .iter()
            .enumerate()
            .filter(|(_, word)| !words.contains(word))
            .map(|(i, _)| i)
            .collect();

        // Clean
        for missed in &missing_words {
            let mut input_words = word_list.clone();
            input_words[*missed] = format!("~~{}~~", input_words[*missed]);
            println!("{}:{}", *missed + 1, input_words.join(" "));
        }

        // Efficient
        let mut words_clone: Vec<String> = word_list.clone();
        for missed in &missing_words {
            let temp = words_clone[*missed].clone();
            words_clone[*missed] = format!("~~{}~~", words_clone[*missed]);
            println!("{}:{}", *missed + 1, words_clone.join(" "));
            words_clone[*missed] = temp;
        }
    }
}
