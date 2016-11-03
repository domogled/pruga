#![feature(plugin)]
#![plugin(docopt_macros)]

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate toml;

extern crate rustc_serialize;
extern crate docopt;

mod pruga;
mod config;
mod parse_file;
mod build;

docopt!(Args derive Debug, "
Usage: pruga [<filename>]

Options:
  -h, --help    Display this message
  --version     Show version.

`pruga filename` compile fielname to web application

If no arguments are provided, then 'пруга' will run `build` and `test`
");

fn main() {

    // let argv = || vec!["pruga", "pruga.toml"];

    // println!("Hello, Пруга!");
    
    // let current_line = line!();
    // let this_file = file!();
    // println!("defined in file: {} on line: {}", this_file, current_line);

    // println!("log level = {:?}", log::max_log_level());

    // Initialize logger
    env_logger::init().unwrap();
    println!("log level = {:?}", log::max_log_level());

    // Parse CLI parameters
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    println!("argumenty {:?}", args);

    // Check if we are (somewhere) in a cargo project directory
    let pruga_dir = match pruga::root() {
        Some(path) => path,
        None => {
            error!("Not a Пруга project, aborting.");
            std::process::exit(64);
        },
    };

    println!("Пруга project directory {:?}", pruga_dir);


    // build::run(&args.arg_filename);
    build::run(pruga_dir.to_str().unwrap());
    
}

/*#[test]
fn it_works() {
    assert!(true);
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_2() {
        assert_eq!(4, add_two(2));
    }
}*/