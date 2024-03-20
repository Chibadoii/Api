use std::fs;

fn select_first_file_in_directory() -> Option<String> {
    let dir_path = "actix-web/actix_web2/ex";
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_name) = path.file_name() {
                        if let Some(file_name_str) = file_name.to_str() {
                            return Some(file_name_str.to_string());
                        }
                    }
                }
            }
        }
    }

    None
}
