use lib::{analyze_file_by_splitting, analyzer::word_count::WordCount};

#[tokio::main]
async fn main() {
    let start_time = tokio::time::Instant::now();
    let cargo_dir = std::env!("CARGO_MANIFEST_DIR");
    println!("cargo_dir = {cargo_dir:?}");
    let path = format!("{cargo_dir}/files/joined/1000.txt");

    let mut word_count = WordCount::new();

    if let Err(err) = analyze_file_by_splitting(&path, &mut word_count, b'\n').await {
        eprintln!("analyze_file error {}", err)
    }

    println!("word_count = {:?}", word_count);
    let end_time = tokio::time::Instant::now();
    println!("done. elapsed = {:?}", end_time.duration_since(start_time));
}
