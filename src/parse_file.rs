use std::fs;
use std::io::prelude::*;
// use toml;
use std;

pub fn from_file<F: Fn(std::string::String) -> Result<String, String> >(source_path: &str, callback: F) -> Result<String, String> {

    println!("FROM FILE {:?}", source_path);

    match fs::File::open(source_path) {
        Ok(mut f) => {
            let mut s = String::new();
            f.read_to_string(&mut s).unwrap();
            callback(s)
            
        },
        Err(e) => {
            let msg = format!("nelze otevřít soubor {}. Error {:?}", source_path, e);
            // error!(&msg.as_str());
            Err(msg)
        },
    }
}


#[cfg(test)]
mod tests {
    // use super::*;
    // use std::env;
    // use std::io::prelude::*;
    // use toml;
    // use std::path::PathBuf;

    /*#[test]
    fn parse_toml_string() {

        let toml = r#"
    [test]
    foo = "bar"
"#;
        let value = from_toml(String::from(toml)).unwrap();

        let naparsovano = match value.lookup("test.foo") {
            Some(hodnota) => hodnota.as_str().unwrap(),
            None => "NONE"
        };
        assert_eq!(naparsovano, "bar");

    }
*/
   /* #[test]
    fn parse_invalid_toml_string() {

        let toml = r#"
    [test]
    foo = bar
"#;
        
         match from_toml(String::from(toml)) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true) /*assert_eq!(e, "bar")*/,
        }

        

    }*/
}
