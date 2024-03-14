use std::sync::Arc;

use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
    sync::Mutex,
};

use analyzer::Analyzer;

pub mod analyzer;

pub const BUCKET_URL: &str = "https://diffusion-corpus.s3.eu-west-2.amazonaws.com";

pub async fn analyze_file_by_splitting(
    path: &str,
    analyzer: &mut impl Analyzer,
    byte: u8,
) -> anyhow::Result<()> {
    let file = File::open(path).await?;
    let mut buf_reader = BufReader::new(file);
    let mut buf = Vec::new();

    while buf_reader.read_until(byte, &mut buf).await? != 0 {
        let chunk = std::str::from_utf8(&buf)?.trim();
        if chunk.is_empty() {
            continue;
        }
        analyzer.process_chunk(path, chunk);
        buf.clear();
    }

    Ok(())
}

pub async fn analyze_file_by_splitting_concurrent(
    path: &str,
    analyzer: Arc<Mutex<impl Analyzer>>,
    byte: u8,
) -> anyhow::Result<()> {
    let file = File::open(path).await?;
    let mut buf_reader = BufReader::new(file);
    let mut buf = Vec::new();

    while buf_reader.read_until(byte, &mut buf).await? != 0 {
        let chunk = std::str::from_utf8(&buf)?.trim();
        if chunk.is_empty() {
            continue;
        }
        let mut lock = analyzer.lock().await;
        lock.process_chunk(path, chunk);
        buf.clear();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use reqwest::StatusCode;

    use crate::analyzer::{
        sentence_count::SentenceCount, word_count::WordCount, word_search::WordSearch,
    };

    use super::*;
    #[tokio::test]
    async fn download() {
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{BUCKET_URL}/1.txt"))
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let body = res.text().await.unwrap();
        println!("body_len = {}", body.len())
    }

    #[tokio::test]
    async fn concurrent_download() {
        use futures_util::StreamExt;
        let client = reqwest::Client::new();

        futures_util::stream::iter(1..=50)
            .for_each_concurrent(50, |i| {
                let c = client.clone();
                async move {
                    let status = c
                        .get(format!("{}/{}.txt", BUCKET_URL, i))
                        .send()
                        .await
                        .unwrap()
                        .status();

                    println!("{i}.txt\t{}", status);
                    assert_eq!(status, StatusCode::OK)
                }
            })
            .await;
    }

    #[tokio::test]
    async fn word_count() {
        let mut wc = WordCount::new();
        let cargo_dir = std::env!("CARGO_MANIFEST_DIR");
        let path = format!("{cargo_dir}/files/raw/1.txt");
        analyze_file_by_splitting(&path, &mut wc, b'\n')
            .await
            .unwrap();

        assert_eq!(wc.n_unique_words(), 4704);

        let most_freq = wc.n_most_frequent(1);
        let most_freq = most_freq.get(0).unwrap();
        assert_eq!(most_freq.0, String::from("the"));
        assert_eq!(most_freq.1, 1248);
    }

    #[tokio::test]
    async fn sentence_count() {
        let mut sc = SentenceCount::new(5, 150);
        let cargo_dir = std::env!("CARGO_MANIFEST_DIR");
        let path = format!("{cargo_dir}/files/raw/1.txt");
        analyze_file_by_splitting(&path, &mut sc, b'\n')
            .await
            .unwrap();

        assert_eq!(sc.n_sentence(), 1712);
    }

    #[tokio::test]
    async fn word_search() {
        let mut ws = WordSearch::new(String::from("the"));
        let cargo_dir = std::env!("CARGO_MANIFEST_DIR");
        let path = format!("{cargo_dir}/files/raw/1.txt");
        analyze_file_by_splitting(&path, &mut ws, b'\n')
            .await
            .unwrap();

        assert_eq!(ws.num_paths(), 1);
    }
}
