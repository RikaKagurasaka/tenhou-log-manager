use std::path::PathBuf;

use rusty_leveldb::{LdbIterator, Options, DB};
use sqlite::State;

pub fn read_leveldb(path: &PathBuf) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut options = Options::default();
    options.create_if_missing = false;
    let mut db = DB::open(path, options)?;
    let mut iter = db.new_iter()?;
    iter.seek_to_first();
    let mut logs = vec![];
    while iter.valid() {
        let mut key = vec![];
        let mut value = vec![];
        iter.current(&mut key, &mut value);
        if let (Ok(key), Ok(value)) = (
            String::from_utf8(key.clone()),
            String::from_utf8(value.clone()),
        ) {
            if key.starts_with("_https://tenhou.net\0\u{1}log") {
                logs.push(String::from(&value[1..]));
            }
        }
        iter.next();
    }
    Ok(logs)
}

pub fn read_sqlite(path: &PathBuf) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let conn = sqlite::open(path)?;
    let mut stmt = conn.prepare("SELECT * FROM data;")?;
    let mut logs = vec![];
    while let Ok(State::Row) = stmt.next() {
        let key = stmt.read::<String, _>("key").unwrap();
        let value = stmt.read::<String, _>("value").unwrap();
        if key.starts_with("log") {
            logs.push(value);
        }
    }
    Ok(logs)
}

pub fn read_all() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut logs = vec![];
    let app_data_path = PathBuf::from(std::env::var("APPDATA").unwrap());
    let edge_path = app_data_path.join(r#"Microsoft\Edge\User Data\Default\Local Storage\leveldb"#);
    logs.append(&mut read_leveldb(&edge_path).unwrap_or(vec![]));
    let chrome_path =
        app_data_path.join(r#"Google\Chrome\User Data\Default\Local Storage\leveldb"#);
    logs.append(&mut read_leveldb(&chrome_path).unwrap_or(vec![]));
    let firefox_path = glob::glob(
        &app_data_path
            .join(r#"Mozilla\Firefox\Profiles\*\storage\default\https+++tenhou.net\ls\data.sqlite"#)
            .to_str()
            .unwrap(),
    )?;
    for path in firefox_path {
        if let Ok(path) = path {
            logs.append(&mut read_sqlite(&path).unwrap_or(vec![]));
        }
    }
    let reg = regex::Regex::new(r#"\d{10}gm-\d{4}-\d{4}-[0-9a-f]{8}"#).unwrap();
    let logs = reg
        .find_iter(&logs.join(" "))
        .map(|m| m.as_str().to_string())
        .collect::<Vec<String>>();
    Ok(logs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leveldb_test() {
        let path = PathBuf::from(
            r#"C:\Users\rika_\AppData\Local\Microsoft\Edge\User Data\Default\Local Storage\leveldb"#,
        );
        println!("{:?}", read_leveldb(&path));
    }

    #[test]
    fn sqlite_test() {
        let path = PathBuf::from(
            r#"C:\Users\rika_\AppData\Roaming\Mozilla\Firefox\Profiles\8g6l5z09.default-release\storage\default\https+++tenhou.net\ls\data.sqlite"#,
        );
        println!("{:?}", read_sqlite(&path));
    }

    #[test]
    fn all_test() {
        println!("{:?}", read_all());
    }
}
