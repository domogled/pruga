#![feature(plugin)]
#![plugin(docopt_macros)]

#[macro_use]
extern crate log;
extern crate env_logger;
// extern crate toml;

extern crate rustc_serialize;
extern crate docopt;

// use util::important_paths::{find_project_root_directory};
// use std::env;
use std::path::PathBuf;
// use std::path::Path;

mod build;
mod run_php;
mod util;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(RustcDecodable)]
pub enum Transformations { Php, Md2html, Html2elm, Router2elm, Model2elm }

docopt!(Args derive Debug, "
Usage:
    pruga from <part> to <destination> with <transformation>...

Options:
  -h, --help    Display this message
  --version     Show version.
  --project-directory=<project_directory>     set root project directory [default: ./]

`pruga filename` compile fielname to web application

If no arguments are provided, then 'пруга' will run `build` and `test`
"
// @TODO: takto by to bylo pěkné, ale není, protože: Could not decode './dest/' to u8 for '<destination>'.
// ,arg_part: Option<PathBuf>
// ,arg_destination: Option<PathBuf>
,arg_part: Option<std::string::String>
,arg_destination: Option<std::string::String>

,arg_transformation: std::vec::Vec<Transformations>
// ,arg_transformation: std::vec::Vec<std::string::String>

/*, flag_project_directory: PathBuf*/);





fn main() {

    // Initialize logger
    env_logger::init().unwrap();
    println!("log level = {:?}", log::max_log_level());

    // Parse CLI parameters
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());

    debug!("argumenty {:?}", args);
    
    // @TODO what is better default value a creating Path from command line
    /*fn is_workspace(working_path: &str) -> Result<PathBuf, std::io::Error> {
        match working_path {
            "" => env::current_dir(), //.unwrap().as_path()
            path => Ok(PathBuf::from(&path)),  
        }
    }*/

    /*let cwd = /*args.flag_project_directory;*/ Path::new(&args.flag_project_directory).canonicalize().unwrap(); //is_workspace(&args.flag_project_directory).unwrap();
    
    let pruga_root_path = find_project_root_directory(&cwd, "pruga.toml")
        .expect("Adresář {:?} není adresářem projektu Пруга");
    
    
    debug!("Пруга project manifest {:?}", pruga_root_path);*/

    if args.cmd_from {
        debug!("idu da parsuji {:?} do {:?}", args.arg_part, args.arg_destination);
        // let vytvoreno = build::make(&pruga_root_path, args.arg_part.unwrap().as_str(), args.arg_destination.as_str());

        // @TODO: kdyby byly cesty dekodovány přímo, což zlobí
        // let vytvoreno = build::make(args.arg_part.unwrap().as_path(), args.arg_destination.unwrap().as_path(), args.arg_transformation);
        let vytvoreno = build::make(PathBuf::from(args.arg_part.unwrap()), PathBuf::from(args.arg_destination.unwrap()), args.arg_transformation);

        debug!("build make vrátil {:?}", vytvoreno);
    }

    
    
}


#[test]
fn parse_options() {
    
    fn s(x: &str) -> String { x.to_string() }

    let argv = || vec!["pruga", "from", "./example_src/view.md", "to", "./dest/", "with", "php", "md2html", "html2elm"];

    let args: Args = Args::docopt().argv(argv().into_iter()).decode().unwrap_or_else(|e| e.exit());

    // Now access your argv values.
    assert!(args.cmd_from);

    // assert_eq!(args.arg_part, Some(PathBuf::from("./example_src/view.md")));
    assert_eq!(args.arg_part, Some(s("./example_src/view.md")));

    assert!(args.cmd_to);

    // assert_eq!(args.arg_destination, Some(PathBuf::from("./dest/")));
    assert_eq!(args.arg_destination, Some(s("./dest/")));

    assert_eq!(args.arg_transformation, vec![Transformations::Php, Transformations::Md2html, Transformations::Html2elm]);
    // assert_eq!(args.arg_transformation, vec![s("php"), s("md2html"), s("html2elm")]);

    // assert_eq!(args.flag_project_directory, s("./"));
    // assert_eq!(args.flag_project_directory, PathBuf::from("./"));

    // assert_eq!(args.arg_dest, s(""));
}

/*#[test]
fn parse_options_2() {
    
    fn s(x: &str) -> String { x.to_string() }

    let argv = || vec!["pruga", "from", "view", "to", "dest/", "with", "php", "md2html", "html2elm"];

    let args: Args = Args::docopt().argv(argv().into_iter()).decode().unwrap_or_else(|e| e.exit());

    // Now access your argv values.
    assert!(args.cmd_from);

    assert_eq!(args.arg_part, Some(s("view")));

    assert!(args.cmd_to);

    // assert_eq!(args.arg_destination, vec![s("dest/")]);
    assert_eq!(args.arg_destination, s("dest/"));

    // assert_eq!(args.flag_project_directory, s("/home/rust/pruga"));
    // assert_eq!(args.flag_project_directory, PathBuf::from("/home/rust/pruga"));

    // assert_eq!(args.arg_dest, s(""));
}*/
