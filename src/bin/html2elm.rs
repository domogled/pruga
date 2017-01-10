#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate rustc_serialize;
extern crate docopt;

extern crate pruga;
// use pruga::html2elm::from_file;

use std::path::Path;
use std::fs::File;
use std::io::Write;

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
    println!("cíl {:?}", args.arg_destination);

    // let source = Path::new(& args.arg_source);
    //
    // let mut file = match File::open(&source) {
    // The `description` method of `io::Error` returns a string that
    // describes the error
    // Err(why) => panic!("couldn't open {}: {}", args.arg_source,
    // why.description()),
    // Ok(file) => file,
    // };

    // Read the file contents into a string, returns `io::Result<usize>`
    // let mut s = String::new();
    // match file.read_to_string(&mut s) {
    // Err(why) => panic!("couldn't read {}: {}", args.arg_source,
    // why.description()),
    // Ok(_) => print!("{} contains:\n{}", args.arg_source, s),
    // }


    //

    let source_path = Path::new(&args.arg_source).canonicalize().unwrap();

    let output = pruga::html2elm::from_file(&source_path);

    // print!("{} compiled as:\n{}", args.arg_source, output.unwrap());

    match args.arg_destination.is_empty() {
        false => {
            let destination = args.arg_destination;
            let mut f = File::create(&destination).unwrap();
            f.write_all(output.unwrap().as_bytes());
            println!("code is writed into file {}", destination);
        }
        true => println!("{}", output.unwrap()),
    }

    // println!("****************************\n{}", pruga::html2elm::from_string(String::new()).unwrap());
}
