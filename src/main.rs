#![feature(plugin)]
#![plugin(docopt_macros)]

#[macro_use]
extern crate log;
extern crate env_logger;
// extern crate toml;

extern crate rustc_serialize;
extern crate docopt;

use util::important_paths::{find_project_root_directory};
use std::env;
use std::path::PathBuf;
// use std::path::Path;

mod build;
mod run_php;
mod util;

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
    
    // @TODO better default value from command line
    fn is_workspace(working_path: &str) -> Result<PathBuf, std::io::Error> {
        match working_path {
            "" => env::current_dir(), //.unwrap().as_path()
            path => Ok(PathBuf::from(&path)),  
        }
    }

    let cwd = is_workspace(&args.arg_filename).unwrap();
    
    let pruga_root_path = find_project_root_directory(&cwd, "pruga.toml")
        .expect("Adresář {:?} není adresářem projektu Пруга");
    
    
    info!("Пруга project manifest {:?}", pruga_root_path);

    build::file(pruga_root_path.as_path(), "view.md");
    
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