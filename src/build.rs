use get_toml;
use std;
use toml;

pub fn run(source_file: &str) {
    
    match get_toml::from_file(source_file) {
        Ok(value) => {
            println!("source SOURCE {:?}", value);
            compile_main(value)
        },
        Err(e) => error!("parsování souboru {} selhalo, error {:?}", source_file, e),
    }
}

fn compile_main(source: toml::Value) {
    let view = match source.lookup("main.view") {
            Some(hodnota) => /*hodnota.as_str().unwrap_or("NENE")*/println!("VIEW {:?}", hodnota),
            None => {
                error!("Není\n------------------\n[main]\nview=\n------------------\n");
                std::process::exit(64);
            },
        };
}