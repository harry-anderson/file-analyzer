use std::{error::Error, sync::Arc};

use futures_util::StreamExt;
use lib::{
    analyze_file_by_splitting_concurrent,
    analyzer::{sentence_count::SentenceCount, word_count::WordCount},
};
use structopt::StructOpt;
use tokio::sync::Mutex;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long)]
    dir: String,
    #[structopt(long)]
    files: usize,
    #[structopt(long)]
    threads: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let Opt {
        dir,
        files,
        threads,
    } = Opt::from_args();

    run(&dir, files, threads).await?;
    Ok(())
}

async fn run(dir: &str, num_files: usize, num_threads: usize) -> anyhow::Result<()> {
    let start = tokio::time::Instant::now();
    let word_count = Arc::new(Mutex::new(WordCount::new()));
    let sentence_count = Arc::new(Mutex::new(SentenceCount::new(5, 150)));

    futures_util::stream::iter(1..num_files)
        .for_each_concurrent(num_threads, |x| {
            let word_count_c = word_count.clone();
            let sentence_count_c = sentence_count.clone();
            async move {
                let path = format!("{dir}/{x}.txt");
                println!("file = {path}");
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
    Ok(())
}
