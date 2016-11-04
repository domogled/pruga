use std::fs;
use std::path::{Path, PathBuf};

/// Iteratively search for `file` in `pwd` and its parents, returning
/// the path to the file.
pub fn find_project_root_directory(pwd: &Path, manifest_name: &str) -> Result<PathBuf, String> {
    let mut current = pwd;

    loop {
        let manifest = current.join(manifest_name);
        if fs::metadata(&manifest).is_ok() {
            
            return Ok(current.to_path_buf())
        }

        match current.parent() {
            Some(p) => current = p,
            None => break,
        }
    }

    Err(format!("could not find `{}` in `{}` or any parent directory",
          manifest_name, pwd.display()))
}


#[cfg(test)]
mod tests {
    use super::*;
    // use std::env;
    use std::path::Path;
    // use std::path::PathBuf;

    #[test]
    fn is_pruga_project_directory() {

        // assert_eq!(None, root());

        // let project_directory_path = Path::new("/home/rust/pruga_example");
        // assert!(env::set_current_dir(&project_directory_path).is_ok());
        // error!("Successfully changed working directory to {}!", project_directory_path.display());
        // assert_eq!(Some(PathBuf::from("/home/rust/pruga_example")), find_project_manifest(&project_directory_path, "pruga.toml"));

        assert!(find_project_root_directory(&Path::new("/home/rust/pruga_example"), "pruga.toml").is_ok());
        assert!(find_project_root_directory(&Path::new("/home"), "pruga.toml").is_err());

    }
}