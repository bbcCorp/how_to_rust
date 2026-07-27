use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use std::io::Error;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

fn main() {
    println!(" === Word Count Occurance finder ===");

    let common_words = Arc::new(vec![
        String::from("Gullivers"),
        String::from("Gutenberg"),
        String::from("travel"),
    ]);

    let wc_result: HashMap<String, i32> = find_word_occurances(common_words).unwrap();
    println!("Total Word count: {:?}", wc_result);
}

fn find_word_occurances(
    common_words: Arc<Vec<String>>,
) -> Result<HashMap<String, i32>, std::io::Error> {
    let mut wordcount_map: HashMap<String, i32> = HashMap::new();

    // Demo - Buffered reading
    let file_path = Path::new("src/gullivers_travels.txt");
    let fp = File::open(file_path)?;
    let reader = BufReader::new(fp);

    // we will read 1000 lines from a file and process them
    let buffer_limit: i32 = 1000; // number of lines to read
    let mut line_count: i32 = 0;

    let mut lines_buffer: Vec<String> = Vec::new();

    let (tx, rx): (Sender<HashMap<String, i32>>, Receiver<HashMap<String, i32>>) = channel();
    let mut children: Vec<JoinHandle<()>> = Vec::new();

    let start = Instant::now();

    for (_i, line) in reader.lines().enumerate() {
        // check if we have reached the buffer limit
        if line_count >= buffer_limit {
            // Clone before the closure so the originals stay usable on the next loop iteration.
            // `move` would otherwise consume tx/common_words/lines_buffer on the first iteration.
            let tx_clone = tx.clone();
            let shared_common_words = Arc::clone(&common_words);
            let buffer_snapshot = lines_buffer.clone();

            // pass on the content for processing to a new thread
            let child: JoinHandle<()> = thread::spawn(move || {
                // process the buffer contents
                let result: HashMap<String, i32> =
                    find_word_count_in_buffer(&buffer_snapshot, shared_common_words).unwrap();

                let _ = tx_clone.send(result);
            });
            children.push(child);

            line_count = 0;
            lines_buffer.clear();
        }
        let line = line?;
        lines_buffer.push(line);
        line_count += 1;
    }

    // if !lines_buffer.is_empty() {
    //     // process the last buffer
    //     let child = thread::spawn(move || {
    //         // process the buffer contents
    //         let shared_common_words = common_words.clone();
    //         let result: HashMap<String, i32> =
    //             find_word_count_in_buffer(&lines_buffer, shared_common_words).unwrap();

    //         tx.send(result);
    //     });
    //     children.push(child);

    //     lines_buffer.clear();
    // }

    // Drop the original sender so the channel closes once all thread tx_clones are dropped.
    // Without this, rx never ends because `tx` keeps the channel open from the main thread.
    drop(tx);

    for received in rx {
        let result = received;
        for (word_key, word_count) in result.iter() {
            *wordcount_map.entry(word_key.clone()).or_insert(0) += word_count;
        }
    }

    let total_duration: Duration = start.elapsed();
    println!("Total time elapsed: {:?} ms", total_duration.as_millis());

    Ok(wordcount_map)
}

fn find_word_count_in_buffer(
    lines_buffer: &Vec<String>,
    search_words: Arc<Vec<String>>,
) -> Result<HashMap<String, i32>, Error> {
    let spawned_thread_id = thread::current().id();

    let process_start = Instant::now();

    let mut wordcount_map: HashMap<String, i32> = HashMap::new();

    // Access the Vec using deref
    let searched_words = &*search_words;
    for word in searched_words {
        wordcount_map.insert(word.to_lowercase(), 0);
    }

    for line in lines_buffer {
        let cleaned_line: String = line
            .to_lowercase()
            .chars()
            .filter(|c| *c != ',' && *c != '!' && *c != '.')
            .collect();

        // parse line and get words
        let words_in_line: Vec<&str> = cleaned_line.split_whitespace().collect();

        for word in words_in_line {
            let word_check: String = word.trim().to_string();

            if wordcount_map.contains_key(&word_check) {
                *wordcount_map.entry(word_check.clone()).or_insert(0) += 1;
            }
        }
    }

    let process_duration: Duration = process_start.elapsed();

    println!(
        "Thread id: {:?} Word count: {:?} process duration: {:?} ms",
        spawned_thread_id,
        wordcount_map,
        process_duration.as_millis()
    );
    Ok(wordcount_map)
}
