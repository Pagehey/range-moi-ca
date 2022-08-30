use std::{fs::{self, DirEntry, ReadDir}, collections::HashMap, io};
use chrono::prelude::*;
use filetime;

pub fn move_entry_to_folder(entry: &DirEntry, folder_name: &String) -> Result<(), io::Error> {
    let filename = entry.file_name();
    if let Some(filename) = filename.to_str() {
        fs::rename(entry.path(), format!("{}/{}", folder_name, filename))?;
        return Ok(());
    }
    Err(io::Error::new(io::ErrorKind::Other, "No filename."))
}

pub fn extract_entries_by_date(dir: ReadDir) -> HashMap<Date<Utc>, Vec<DirEntry>> {
    let mut entries_by_date: HashMap<Date<Utc>, Vec<DirEntry>> = HashMap::new();
    for entry in dir {
        if let Ok(entry) = entry {
            if is_hidden_file(&entry) { continue };

            if let Ok(metadata) = entry.metadata() {
                if let Some(ft) = filetime::FileTime::from_creation_time(&metadata) {
                    let unixsecs = ft.unix_seconds();
                    let naive = NaiveDateTime::from_timestamp(unixsecs, 0);
                    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
                    let date = datetime.date();

                    let files = entries_by_date.entry(date).or_insert(Vec::new());
                    files.push(entry);
                }
            }
        }
    };
    return entries_by_date;
}

#[cfg(unix)]
fn is_hidden_file(entry: &DirEntry) -> bool {
    return entry.file_name().to_string_lossy().starts_with(".");
}
