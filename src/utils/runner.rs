use std::path::Path;
//  ! This is a list of defined files that the analyser will look for.
pub const DEFINED_FILES: [&str; 1] = [
    "package.json",
    // "Cargo.toml"
];

//  ! This function will return the current directory.
pub fn get_current_dir() -> String {
    let current_dir = std::env::current_dir().unwrap();
    let path = current_dir.to_str().unwrap().to_owned();
    path
}

//  ! This function will check if a file exists.
pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

// TODO: type file_picker function properly
//  ! This function will check if a file exists and return the path.
pub fn file_picker(path: &str) -> Option<String> {
    if path == "" {
        return file_picker(&get_current_dir());
    }

    for file in DEFINED_FILES.iter() {
        let file_path = format!("{}/{}", path, file);
        if file_exists(&file_path) {
            return Some(file_path);
        } else {
            return Some("No Config file Detected".to_string());
        }
    }

    None
}

pub fn file_reader(v_path: &str) -> String {
    std::fs::read_to_string(v_path).expect("Failed to read file")
}
