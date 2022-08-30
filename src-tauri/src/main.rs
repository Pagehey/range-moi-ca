#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
mod utils;
use utils::pluralize;
mod entry;
use entry::{extract_entries_by_date, move_entry_to_folder};
mod response;
use response::{Response, Status};

#[tauri::command]
fn select_folder(folder_path: &str) -> Response {
    return match fs::read_dir(folder_path) {
        Ok(dir) => {
            let entries_by_date = extract_entries_by_date(dir);
            let folders_count = entries_by_date.len();
            let entries_count = entries_by_date.into_values().fold(0, |count, entries| count + entries.len());
            let message = format!(
                "{} trouvés\n{} à créer",
                pluralize("fichier", entries_count),
                pluralize("dossier", folders_count)
            );
            return Response::new(Status::Success, message);
        },
        Err(msg) => {
            let message = format!("Impossible d'ouvrir ce dossier.\n{}", msg);
            Response::new(Status::Failure, message.into())
        }
    };
}

#[tauri::command]
fn group_files(folder_path: &str) -> Response {
    return match fs::read_dir(folder_path) {
        Ok(dir) => {
            let entries_by_date = extract_entries_by_date(dir);
            for (date, entries) in entries_by_date.iter() {
                let folder_name = format!("{}/{}", &folder_path, date.format("%Y-%m-%d").to_string());

                if let Ok(_) = fs::create_dir_all(&folder_name) {
                    for entry in entries {
                        if let Err(msg) = move_entry_to_folder(entry, &folder_name) {
                            return Response::new(Status::Failure, msg.to_string());
                        }
                    };
                } else {
                    return Response::new(Status::Failure, "".into());
                }
            };
            return Response::new(Status::Success, "".into());
        },
        _ => { Response::new(Status::Failure, "Erreur lors de l'opération.".into()) }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![select_folder, group_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
