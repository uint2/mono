use anyhow::Result;
use futures_util::StreamExt;
use indicatif::MultiProgress;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

fn bar(content_len: u64) -> ProgressBar {
    let s = ProgressStyle::with_template("{bar:40.cyan/blue} [{bytes_per_sec}]");
    ProgressBar::new(content_len).with_style(s.unwrap())
}

async fn download(url: String, filepath: PathBuf, bar: ProgressBar) -> Result<()> {
    if let Some(dir) = filepath.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let mut file = File::create(&filepath)?;
    let response = reqwest::get(url).await?;
    let content_len = response.content_length();
    bar.set_length(content_len.unwrap_or(0));
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(chunk) => {
                match content_len {
                    Some(_) => bar.inc(chunk.len() as u64),
                    None => bar.inc_length(chunk.len() as u64),
                }
                file.write_all(&chunk)?;
            }
            Err(e) => {
                let _ = std::fs::remove_file(filepath);
                return Err(e)?;
            }
        };
    }
    bar.finish_and_clear();
    Ok(())
}

pub struct Downloader {
    tasks: Vec<(String, PathBuf)>,
    bars: MultiProgress,
}

impl Downloader {
    pub fn new() -> Self {
        Self { tasks: vec![], bars: MultiProgress::new() }
    }

    pub fn add<P: AsRef<Path>>(&mut self, url: &str, file: P) {
        if file.as_ref().is_file() {
            println!("[{}] already exists.", file.as_ref().display());
            return;
        }
        let url = url.to_string();
        let file = file.as_ref().to_path_buf();
        self.tasks.push((url, file));
    }

    pub async fn run(&mut self, max_concurrent: usize) {
        let tasks = std::mem::take(&mut self.tasks);
        let mut js = tokio::task::JoinSet::new();
        for (url, filepath) in tasks {
            while js.len() >= max_concurrent {
                let _ = js.join_next().await.unwrap().unwrap();
            }
            let bar = self.bars.add(bar(1));
            js.spawn(download(url, filepath, bar));
        }
        while let Some(output) = js.join_next().await {
            let _ = output.unwrap();
        }
    }
}
