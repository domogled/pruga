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
Usage:
    pruga from <part>... to <destination>... [--project-directory=<project_directory>]
    pruga set <part> to <value> [--project-directory=<project_directory>]

Options:
  -h, --help    Display this message
  --version     Show version.
  --project-directory=<project_directory>     set root project directory

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

    let cwd = is_workspace(&args.flag_project_directory).unwrap();
    
    let pruga_root_path = find_project_root_directory(&cwd, "pruga.toml")
        .expect("Adresář {:?} není adresářem projektu Пруга");
    
    
    info!("Пруга project manifest {:?}", pruga_root_path);

    build::file(pruga_root_path.as_path(), "view.md");
    
}


#[test]
fn parse_options() {
    
    fn s(x: &str) -> String { x.to_string() }

    let argv = || vec!["pruga", "from", "file1", "file2", "to", "dest/"];

    let args: Args = Args::docopt().argv(argv().into_iter()).decode().unwrap_or_else(|e| e.exit());

    // Now access your argv values.
    assert!(args.cmd_from);
    assert_eq!(args.arg_part, vec![s("file1"), s("file2")]);
    assert!(args.cmd_to);
    assert_eq!(args.arg_destination, vec![s("dest/")]);
    assert_eq!(args.flag_project_directory, s(""));
    // assert_eq!(args.arg_dest, s(""));
}

#[test]
fn parse_options_2() {
    
    fn s(x: &str) -> String { x.to_string() }

    let argv = || vec!["pruga", "from", "file1", "file2", "to", "dest/", "--project-directory", "./home/rust/pruga"];

    let args: Args = Args::docopt().argv(argv().into_iter()).decode().unwrap_or_else(|e| e.exit());

    // Now access your argv values.
    assert!(args.cmd_from);
    assert_eq!(args.arg_part, vec![s("file1"), s("file2")]);
    assert!(args.cmd_to);
    assert_eq!(args.arg_destination, vec![s("dest/")]);
    assert_eq!(args.flag_project_directory, s("./home/rust/pruga"));
    // assert_eq!(args.arg_dest, s(""));
}
