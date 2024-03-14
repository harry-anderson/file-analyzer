use lib::{
    analyze_file_by_splitting,
    analyzer::{sentence_count::SentenceCount, word_count::WordCount},
};

#[tokio::main]
async fn main() {
    let start_time = tokio::time::Instant::now();
    let cargo_dir = std::env!("CARGO_MANIFEST_DIR");
    println!("cargo_dir = {cargo_dir:?}");
    let path = format!("{cargo_dir}/files/joined/1000.txt");
    let mut word_count = WordCount::new();
    let mut sentence_count = SentenceCount::new(5, 150);

    if let Err(err) = analyze_file_by_splitting(&path, &mut word_count, b'\n').await {
        eprintln!("analyze_file word_count error {}", err)
    }
    if let Err(err) = analyze_file_by_splitting(&path, &mut sentence_count, b'.').await {
        eprintln!("analyze_file sentence_count error {}", err)
    }

    println!(
        "n_most_frequent_words = {:?}",
        word_count.n_most_frequent(10)
    );
    println!("n_unique_words = {}", word_count.n_unique_words());
    println!("n_sentence_= {}", sentence_count.n_sentence());

    let end_time = tokio::time::Instant::now();
    println!("done. elapsed = {:?}", end_time.duration_since(start_time));
}
