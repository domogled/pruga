extern crate pruga;

use pruga::html2elm;

use std::fs;
// use std::io;

use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

// extern crate pruga::adder;


fn read_file(path: &Path) -> Result<String, String> {

    let display = path.display();

    let mut file = match File::open(path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut source = String::new();
    match file.read_to_string(&mut source) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => print!("{} contains:\n{}", display, source),
    }

    Ok(source)

}

#[test]
fn html2elm_empty_source() {
    assert_eq!(html2elm::from_string(String::new()).unwrap(), String::new());
}


#[test]
fn html2elm_examples() {

    let paths = fs::read_dir("./examples").unwrap();

    fn build_project(path: &Path) {

        let view_path = path.join("src/view.html");
        let output_path = path.join("test/View.elm");

        let source = html2elm::from_file(&view_path).unwrap();
        let output = read_file(&output_path).unwrap();

        assert_eq!(source, output);

    }

    for entry in paths {
        // let entry = try!(entry);
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            println!("kompiluji {:?}", path);
            build_project(&path);
        } else {
            println!("přeskakuji {:?}", path);
        }
    }


    // assert_eq!(4, 5);
}