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
        analyzer.process_chunk(&path, chunk);
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

    use super::*;

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
}
