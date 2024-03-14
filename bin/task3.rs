use std::sync::Arc;

use futures_util::StreamExt;
use lib::{
    analyze_file_by_splitting_concurrent,
    analyzer::{sentence_count::SentenceCount, word_count::WordCount},
};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let cargo_dir = std::env!("CARGO_MANIFEST_DIR");
    println!("cargo_dir = {cargo_dir:?}");
    let start = tokio::time::Instant::now();
    let word_count = Arc::new(Mutex::new(WordCount::new()));
    let sentence_count = Arc::new(Mutex::new(SentenceCount::new(5, 150)));

    futures_util::stream::iter(1..500)
        .for_each_concurrent(20, |x| {
            let word_count_c = word_count.clone();
            let sentence_count_c = sentence_count.clone();
            async move {
                let path = format!("{cargo_dir}/files/raw/{x}.txt");
                if let Err(err) =
                    analyze_file_by_splitting_concurrent(&path, word_count_c, b'\n').await
                {
                    eprintln!("analyze_file word_count error {}", err)
                }

                if let Err(err) =
                    analyze_file_by_splitting_concurrent(&path, sentence_count_c, b'.').await
                {
                    eprintln!("analyze_file sentence_count error {}", err)
                }
            }
        })
        .await;

    println!(
        "n_most_frequent_words = {:?}",
        word_count.lock().await.n_most_frequent(10)
    );

    println!(
        "n_unique_words = {}",
        word_count.lock().await.n_unique_words()
    );

    println!("n_sentences = {}", sentence_count.lock().await.n_sentence());

    let end = tokio::time::Instant::now();
    println!("done. elapsed = {:?}", end.duration_since(start));
}
