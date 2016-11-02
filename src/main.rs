#[macro_use]
extern crate log;
extern crate env_logger;

mod pruga;
mod config;

fn main() {
    println!("Hello, Пруга!");

    // Initialize logger
    env_logger::init().unwrap();

    // Check if we are (somewhere) in a cargo project directory
    let pruga_dir = match pruga::root() {
        Some(path) => path,
        None => {
            error!("Not a Пруга project, aborting.");
            std::process::exit(64);
        },
    };

    println!("Пруга project directory {:?}", pruga_dir);
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