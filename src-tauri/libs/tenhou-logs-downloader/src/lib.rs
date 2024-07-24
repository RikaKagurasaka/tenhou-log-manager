use std::path::PathBuf;
use std::sync::Arc;
use futures::future::join_all;
use reqwest::Client;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::sync::Semaphore;

async fn download_file(
    client: &Client,
    url: &str,
    path: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = client.get(url).send().await?;
    let mut file = fs::File::create(path).await?;
    let content = response.bytes().await?;
    file.write_all(&content).await?;
    Ok(())
}

async fn check_and_download(
    client: Arc<Client>,
    url: String,
    path: PathBuf,
    semaphore: Arc<Semaphore>,
) -> Result<bool, ()> {
    let _permit = semaphore.acquire().await;

    if path.exists() {
        let metadata = fs::metadata(&path).await.map_err(|_| ())?;
        if metadata.len() > 0 {
            println!(
                "File already exists and is not empty, skipping download: {}",
                path.to_str().unwrap()
            );
            return Ok(false);
        }
    }

    println!("Downloading: {}", url);
    download_file(&client, &url, path).await.map_err(|e| eprintln!("Error: {}", e))?;
    println!("Downloaded: {}", url);

    Ok(true)
}

pub async fn download_files(urls: Vec<String>, save_dir: &PathBuf) -> Result<(u64, u64, u64), String> {
    fs::create_dir_all(save_dir).await.map_err(|e| e.to_string())?;
    let client = Arc::new(Client::new());
    let semaphore = Arc::new(Semaphore::new(5));
    let (mut skipped, mut downloaded, mut fail) = (0u64, 0u64, 0u64);

    let futures = urls.into_iter().map(|url| {
        let client = Arc::clone(&client);
        let semaphore = Arc::clone(&semaphore);
        let path = save_dir.join(
            PathBuf::from(&url)
                .with_extension("xml")
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .replace("?", ""),
        );
        check_and_download(client, url, path, semaphore)
    });

    let results = join_all(futures).await;

    for result in results {
        match result {
            Ok(true) => downloaded += 1,
            Ok(false) => skipped += 1,
            Err(_) => {
                fail += 1;
            }
        }
    }

    Ok((downloaded, skipped, fail))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let urls = "2024072317gm-0089-0000-87c78500,2024072217gm-0089-0000-d5b489ec,2024072017gm-0089-0000-1fdd9250,2024071919gm-0009-0000-adcf8a34,2024071817gm-0009-0000-1d267e0f"
            .split(',').map(|s| "https://tenhou.net/0/log/?".to_string() + s).collect();
        let save_dir = PathBuf::from(r#"D:\Projects\tenhou-log-manager\src-tauri\logs"#);

        download_files(urls, &save_dir).await.expect("Failed to download files");
    }
}
