use browser_ls_reader::read_all;
use tenhou_logs_downloader::download_files;

#[tauri::command]
pub fn scan_local_logs() -> String {
    read_all().unwrap().join("\n")
}

#[tauri::command]
pub async fn download_logs(ids: String) -> ((u64, u64, u64), String) {
    let urls = ids.split("\n").map(|s| "https://tenhou.net/0/log/?".to_string() + s).collect();
    let save_dir = std::env::current_dir().unwrap().join("logs");
    match download_files(urls, &save_dir).await {
        Ok((skipped, downloaded, fail)) => ((skipped, downloaded, fail), "".to_string()),
        Err(e) => ((0, 0, 0), e)
    }
}