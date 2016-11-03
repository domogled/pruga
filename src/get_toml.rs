use std::fs;
use std::io::prelude::*;
use toml;

pub fn from_file(manifest_path: &str) -> Result<toml::Value, String> {


    match fs::File::open(manifest_path) {
        Ok(mut f) => {
            let mut s = String::new();
            f.read_to_string(&mut s).unwrap();
            from_string(s)
        },
        Err(e) => {
            let msg = format!("nelze otevřít soubor {}. Error {:?}", manifest_path, e);
            // error!(&msg.as_str());
            Err(msg)
        },
    }
}

pub fn from_string(manifest_string: String) -> Result<toml::Value, String> {

    let mut parser = toml::Parser::new(&manifest_string);
    match parser.parse() {
        Some(value) => {
            info!("found toml: {:?}", value);
            Ok(toml::Value::Table(value))
        },
        None => {
            let msg = format!("parse errors: {:?}", parser.errors);
            // error!("{}", msg);
            Err(msg)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    // use std::env;
    // use std::io::prelude::*;
    // use toml;
    // use std::path::PathBuf;

    #[test]
    fn parse_toml_string() {

        let toml = r#"
    [test]
    foo = "bar"
"#;
        let value = from_string(String::from(toml)).unwrap();

        let naparsovano = match value.lookup("test.foo") {
            Some(hodnota) => hodnota.as_str().unwrap(),
            None => "NONE"
        };
        assert_eq!(naparsovano, "bar");

    }

    #[test]
    fn parse_invalid_toml_string() {

        let toml = r#"
    [test]
    foo = bar
"#;
        
         match from_string(String::from(toml)) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true) /*assert_eq!(e, "bar")*/,
        }

        

    }
}
