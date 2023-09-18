use std::fs;

// Read HTML file as first-time content loader 
pub fn read_file(file_path: &str) -> String {
    let data: String = fs::read_to_string(file_path)
        .expect("Unable to read file");

    data
}