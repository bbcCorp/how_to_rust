use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use std::io::Error;

fn main() {
    println!(" === Word Count Occurance finder ===");

    let common_words: Vec<String> = vec![String::from("Gutenberg"), String::from("travel")];

    let wc_result = find_word_occurances(&common_words).unwrap();
    println!("Word count: {:?}", wc_result);
}

fn find_word_occurances(
    common_words: &Vec<String>,
) -> Result<HashMap<String, i32>, std::io::Error> {
    let mut wordcount_map: HashMap<String, i32> = HashMap::new();

    // Demo - Buffered reading
    let file_path = Path::new("gullivers_travels.txt");
    let fp = File::open(file_path)?;
    let reader = BufReader::new(fp);

    // we will read 1000 lines from a file and process them
    let buffer_limit = 1000; // number of lines to read
    let mut line_count = 0;

    let mut lines_buffer = Vec::new();
    for (_i, line) in reader.lines().enumerate() {
        if line_count >= buffer_limit {
            // process the buffer contents
            wordcount_map = find_word_count_in_buffer(&lines_buffer, &common_words)?;

            line_count = 0;
            lines_buffer.clear();
        }
        let line = line?;
        lines_buffer.push(line);
        line_count += 1;
    }

    if !lines_buffer.is_empty() {
        // process the last buffer
        wordcount_map = find_word_count_in_buffer(&lines_buffer, &common_words).unwrap();
    }

    Ok(wordcount_map)
}

fn find_word_count_in_buffer(
    lines_buffer: &Vec<String>,
    search_words: &Vec<String>,
) -> Result<HashMap<String, i32>, Error> {
    let mut wordcount_map: HashMap<String, i32> = HashMap::new();

    for word in search_words {
        wordcount_map.insert(word.to_lowercase(), 0);
    }

    for line in lines_buffer {
        // parse line and get words
        let words_in_line: Vec<&str> = line.split_whitespace().collect();

        for word in words_in_line {
            let word_check = word.to_lowercase().trim().to_string();

            if wordcount_map.contains_key(&word_check) {
                *wordcount_map.entry(word_check.clone()).or_insert(0) += 1;
            }
        }
    }

    println!("Word count: {:?}", wordcount_map);
    Ok(wordcount_map)
}
