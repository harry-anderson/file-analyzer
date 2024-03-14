use std::error::Error;

use futures_util::StreamExt;
use lib::BUCKET_URL;
use reqwest::StatusCode;
use structopt::StructOpt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long, short)]
    out: String,
    #[structopt(long, short)]
    start: usize,
    #[structopt(long, short)]
    end: usize,
    #[structopt(long, short)]
    threads: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    run(opt).await?;
    Ok(())
}

async fn run(opt: Opt) -> anyhow::Result<()> {
    let start_time = tokio::time::Instant::now();
    let Opt {
        out,
        start,
        end,
        threads,
    } = opt;
    let out_dir = &out;

    let client = reqwest::Client::new();

    futures_util::stream::iter(start..=end)
        .for_each_concurrent(threads, |i| {
            let c = client.clone();
            async move {
                let res = c.get(format!("{}/{}.txt", BUCKET_URL, i)).send().await;
                match res {
                    Err(err) => eprintln!("{i}.txt error: {err}"),
                    Ok(response) => {
                        if response.status() != StatusCode::OK {
                            eprintln!("{i}.txt error status = {}", response.status())
                        } else {
                            let body = response.text().await.unwrap();
                            let mut file =
                                File::create(format!("{out_dir}/{i}.txt")).await.unwrap();
                            file.write_all(body.as_bytes()).await.unwrap();
                            println!("saved {out_dir}/{i}.txt");
                        }
                    }
                }
            }
        })
        .await;

    let end_time = tokio::time::Instant::now();
    println!("done. elapsed = {:?}", end_time.duration_since(start_time));
    Ok(())
}
