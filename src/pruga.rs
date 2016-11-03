use config;
use std::env;
use std::fs;
use std::path::PathBuf;


/// Returns the closest ancestor path containing a `pruga.toml`.
///
/// Returns `None` if no ancestor path contains a `pruga.toml`, or if
/// the limit of MAX_ANCESTORS ancestors has been reached.
pub fn root() -> Option<PathBuf> {
    /// Checks if the directory contains `pruga.toml`
    fn contains_manifest(path: &PathBuf) -> bool {
        fs::read_dir(path).map(|entries| {
            entries.filter_map(|res| res.ok())
                   .any(|ent| &ent.file_name() == "pruga.toml")
        }).unwrap_or(false)
    }

    // From the current directory we work our way up, looking for `pruga.toml`
    env::current_dir().ok().and_then(|mut wd| {
        for _ in 0..config::MAX_ANCESTORS {
            if contains_manifest(&mut wd) {
                return Some(wd);
            }
            if !wd.pop() {
                break;
            }
        }

        None
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::Path;
    use std::path::PathBuf;

    #[test]
    fn is_pruga_project_directory() {

        // assert_eq!(None, root());

        let project_directory_path = Path::new("/home/rust/pruga_example");
        assert!(env::set_current_dir(&project_directory_path).is_ok());
        error!("Successfully changed working directory to {}!", project_directory_path.display());
        assert_eq!(Some(PathBuf::from("/home/rust/pruga_example")), root());

    }
}