use std::{error::Error, sync::Arc};

use futures_util::StreamExt;
use lib::{analyze_file_by_splitting_concurrent, analyzer::word_search::WordSearch};
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

    #[structopt(long)]
    word: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    run(opt).await?;

    Ok(())
}

async fn run(opt: Opt) -> anyhow::Result<()> {
    let Opt {
        dir,
        files,
        threads,
        word,
    } = opt;

    let start = tokio::time::Instant::now();
    let word_search = Arc::new(Mutex::new(WordSearch::new(word.clone())));
    let dir = &dir;

    futures_util::stream::iter(1..files)
        .for_each_concurrent(threads, |x| {
            let word_search_c = word_search.clone();
            async move {
                let path = format!("{dir}/{x}.txt");
                println!("file = {path}");
                if let Err(err) =
                    analyze_file_by_splitting_concurrent(&path, word_search_c, b'\n').await
                {
                    eprintln!("analyze_file word_count error {}", err)
                }
            }
        })
        .await;
    let ws = word_search.lock().await;
    println!(
        "---\nword = {:?} found = {}\npaths = {:#?}",
        word,
        ws.num_paths(),
        ws.paths()
    );
    let end = tokio::time::Instant::now();
    println!("done. elapsed = {:?}", end.duration_since(start));
    Ok(())
}
