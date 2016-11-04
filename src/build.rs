// use std;
// use toml;

// use parse_file;
// use run_php;
// use std::path::PathBuf;
use std::path::Path;

/*
kompiluje zdrojový soubor
*/
pub fn file(root: &Path, file: &str){
    println!("kompiluji {:?} z adresáře projektu {:?}", file, root);

    let path = Path::new(file);
    let file = path.file_name();
    let extension = path.extension();
    let parent_dir = path.parent();

    println!("path {:?}\nfile {:?}\nextension {:?}\nparent_dir {:?}\n", path, file, extension, parent_dir);
}

/*
pub fn run(source_dir: &str) {
    
    // fn path_to_str(path: std::path::PathBuf) -> &str {
    //     "cesta je štěrk"
    // }

    // let source_path = std::path::PathBuf::from(source_dir);


    let mut view_file = std::path::PathBuf::from(source_dir);
    view_file.push("example_src");
    view_file.push("view.md");
    _read_file(view_file.to_str().unwrap(), compile_view_from_markdown);
    _parse_file(view_file.to_str().unwrap(), run_php::parse);
    // run_php::parse(view_file.to_str().unwrap(), compile_view_from_markdown);
    
}

/*fn compile_main(source: toml::Value) {
    let view = match source.lookup("main.view") {
            Some(hodnota) => /*hodnota.as_str().unwrap_or("NENE")*/println!("VIEW {:?}", hodnota),
            None => {
                error!("Není\n------------------\n[main]\nview=\n------------------\n");
                std::process::exit(64);
            },
        };
}
*/

fn _read_file<F: Fn(std::string::String) -> Result<String, String> >(source_path: &str, callback: F) {

    match parse_file::from_file(source_path, callback) {
        Ok(value) => {
            println!("PRUGA OK _read_file => {:?}", value);
        },
        Err(e) => panic!("PRUGA _read_file :: parsování souboru {} selhalo, error {:?}", source_path, e),
    }
}

fn _parse_file<F: Fn(&str) -> Result<String, String> >(source_path: &str, callback: F) {

    match callback(source_path) {
        Ok(value) => {
            println!("PRUGA OK _read_file => {:?}", value);
        },
        Err(e) => panic!("PRUGA _read_file :: parsování souboru {} selhalo, error {:?}", source_path, e),
    }
}

fn compile_view_from_markdown(source: String) -> Result<String, String> {
    println!("PRUGA compile_view {:?}", source);
    Ok(source)
}

*/