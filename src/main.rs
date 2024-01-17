use llama_cpp_rs::{
    options::{ModelOptions, PredictOptions},
    LLama,
};
use std::env;
// you need this in your cargo.toml
// reqwest = { version = "0.11.3", features = ["stream"] }
// futures-util = "0.3.14"
// indicatif = "0.15.0"
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::cmp::min;
use std::fs;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use downloader::Downloader;

pub async fn download_file(client: &Client, url: &str, path: &str) -> Result<(), String> {
    // Reqwest setup
    let res = client
        .get(url)
        .header("Range", format!("bytes={}-", 0))
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;

    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    pb.set_message(&format!("Downloading {}", url));

    // download chunks
    let mut file = File::create(format!("{}{}", path, "phi-2")).or(Err(format!(
        "Failed to create file '{}'",
        format!("{}{}", path, "phi-2")
    )))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write_all(&chunk)
            .or(Err(format!("Error while writing to file")))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        println!("downlpaded {}", downloaded);
        pb.set_position(new);
    }

    pb.finish_with_message(&format!("Downloaded {} to {}", url, path));
    return Ok(());
}

fn main() {
    let model_options = ModelOptions {
        n_gpu_layers: 1,

        ..Default::default()
    };

    let llama = LLama::new(
        "/Users/yared/.ollama/models/blobs/sha256:e8a35b5937a5e6d5c35d1f2a15f161e07eefe5e5bb0a3cdd42998ee79b057730".into(),
        &model_options,
    )
    .unwrap();

    let predict_options = PredictOptions {
        tokens: 0,
        threads: 3,
        top_k: 90,
        top_p: 0.1,
        token_callback: Some(Box::new(|token| {
            println!("token1: {}", token);

            true
        })),
        ..Default::default()
    };

    llama
        .predict(
            "what are the national animals of india".into(),
            predict_options,
        )
        .unwrap();
}
