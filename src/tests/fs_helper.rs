use std::path::{Path, PathBuf};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_file_as_string<P>(filename: P) -> String
    where P: AsRef<Path>, {
    // Create a path to the desired file
    let display = filename.as_ref().display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&filename) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                           why.description()),
        Ok(_) => {
            return s;
        }
    }
}

pub fn read_file_from_relative_path(path: &str) -> String {
    let mut filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    filename.push(path);
    // File hosts must exist in current path before this produces output
    return read_file_as_string(filename);
}
