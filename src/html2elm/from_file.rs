use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use html2elm::from_string;

pub fn from_file(source_path: &Path) -> Result<String, String> {

    let display = source_path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&source_path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    let result = match file.read_to_string(&mut s) {
        Err(why) => Err(format!("couldn't read {}: {}", display, Error::description(&why))),
        Ok(_) => from_string(s),
    };

    result

}