#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate rustc_serialize;
extern crate docopt;

// use std::io::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// use docopt::Docopt;

docopt!(Args derive Debug, "
Пруга html2elm.

Usage:
  html2elm <source> [<destination>]
  

Options:
  -h --help     Show this screen.
  --version     Show version.
");

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    // println!("{:?}", args);

    println!("zdroj {}", args.arg_source);


    let source = Path::new(& args.arg_source);
    
    let mut file = match File::open(&source) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", args.arg_source,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", args.arg_source,
                                                   why.description()),
        Ok(_) => print!("{} contains:\n{}", args.arg_source, s),
    }
}