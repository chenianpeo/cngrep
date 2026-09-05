use std::{
    env::current_dir,
    fs::{File, read_dir},
    io::{self, IsTerminal, Read},
    path::{Path, PathBuf},
};

use crate::{error::Error, reader::Input::MultiFile};

fn recursive_path(paths: &[PathBuf]) -> Result<Vec<PathBuf>, Error> {
    let mut result: Vec<PathBuf> = Vec::new();

    for path in paths {
        if path.is_file() {
            if is_binary(path)? {
                continue;
            }

            result.push(path.clone());

            continue;
        }

        if path.is_dir() {
            let dir_path = read_dir(path)?;

            for entry in dir_path {
                let entry = entry?;
                let single_path = entry.path();

                if is_exclude(&single_path)? {
                    continue;
                }

                result.extend(recursive_path(&[single_path])?);
            }
        }
    }

    result.sort();
    Ok(result)
}

/// analyze input path file types
/// performance loss is quite serious due to read total files
fn is_binary(path: &Path) -> Result<bool, Error> {
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf.contains(&0) || std::str::from_utf8(&buf).is_err())
}

/// exclude file and directory
fn is_exclude(path: &Path) -> Result<bool, Error> {
    let components = path.components();

    use std::ffi::OsStr;

    for component in components.as_path() {
        if component.to_string_lossy().starts_with(".") {
            return Ok(true);
        }

        if component == OsStr::new("target") {
            return Ok(true);
        }
    }

    Ok(false)
}

#[derive(Debug)]
pub enum Input {
    Stdin,
    MultiFile(Vec<PathBuf>),
}

pub fn read(path: &[PathBuf]) -> Result<Input, Error> {
    let stdin = io::stdin();
    if !stdin.is_terminal() {
        return Ok(Input::Stdin);
    }

    if stdin.is_terminal() && path.is_empty() {
        return Ok(Input::MultiFile(recursive_path(&[current_dir()?])?));
    }

    if let Some(file) = path.first()
        && file.is_file()
        && path.len() == 1
    {
        return Ok(MultiFile(vec![file.to_path_buf()]));
    }

    if let Some(dir) = path.first()
        && dir.is_dir()
        && path.len() == 1
    {
        return Ok(MultiFile(recursive_path(std::slice::from_ref(dir))?));
    }

    Ok(MultiFile(recursive_path(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{create_dir_all, write};
    use tempfile::Builder;

    fn tempdir() -> tempfile::TempDir {
        Builder::new().prefix("cngrep-test-").tempdir().unwrap()
    }

    #[test]
    fn binary_file() {
        let dir = tempdir();
        let path = dir.path().join("binary");

        write(&path, [b'a', 0, b'b']).unwrap();

        assert!(is_binary(&path).unwrap());
    }

    #[test]
    fn text_file() {
        let dir = tempdir();
        let path = dir.path().join("text");

        write(&path, "hello rust").unwrap();

        assert!(!is_binary(&path).unwrap());
    }

    #[test]
    fn invalid_utf8() {
        let dir = tempdir();
        let path = dir.path().join("invalid");

        write(&path, [0xff, 0xfe]).unwrap();

        assert!(is_binary(&path).unwrap());
    }

    #[test]
    fn empty_file() {
        let dir = tempdir();
        let path = dir.path().join("empty");

        File::create(&path).unwrap();

        assert!(!is_binary(&path).unwrap());
    }

    #[test]
    fn hidden_path() {
        let path = PathBuf::from("project/.git");

        assert!(is_exclude(&path).unwrap());
    }

    #[test]
    fn target_path() {
        let path = PathBuf::from("project/target");

        assert!(is_exclude(&path).unwrap());
    }

    #[test]
    fn normal_path() {
        let path = PathBuf::from("project/src");

        assert!(!is_exclude(&path).unwrap());
    }

    #[test]
    fn nested_exclude() {
        let path = PathBuf::from("project/target/debug");

        assert!(is_exclude(&path).unwrap());
    }

    #[test]
    fn recursive_files() {
        let dir = tempdir();
        let nested = dir.path().join("src/nested");

        create_dir_all(&nested).unwrap();

        let file1 = dir.path().join("src/main.rs");
        let file2 = nested.join("lib.rs");

        write(&file1, "fn main() {}").unwrap();
        write(&file2, "pub fn test() {}").unwrap();

        let result = recursive_path(&[dir.path().to_path_buf()]).unwrap();

        assert_eq!(result.len(), 2);
        assert!(result.contains(&file1));
        assert!(result.contains(&file2));
    }

    #[test]
    fn recursive_excludes_hidden() {
        let dir = tempdir();

        let visible = dir.path().join("main.rs");
        let hidden = dir.path().join(".git/config");

        create_dir_all(hidden.parent().unwrap()).unwrap();
        write(&visible, "fn main() {}").unwrap();
        write(&hidden, "hidden").unwrap();

        let result = recursive_path(&[dir.path().to_path_buf()]).unwrap();

        assert_eq!(result, vec![visible]);
    }

    #[test]
    fn recursive_excludes_target() {
        let dir = tempdir();

        let visible = dir.path().join("main.rs");
        let target = dir.path().join("target/debug");

        create_dir_all(target.parent().unwrap()).unwrap();
        write(&visible, "fn main() {}").unwrap();
        write(&target, "binary").unwrap();

        let result = recursive_path(&[dir.path().to_path_buf()]).unwrap();

        assert_eq!(result, vec![visible]);
    }

    #[test]
    fn recursive_excludes_binary() {
        let dir = tempdir();

        let text = dir.path().join("main.rs");
        let binary = dir.path().join("app");

        write(&text, "fn main() {}").unwrap();
        write(&binary, [0, 1, 2, 3]).unwrap();

        let result = recursive_path(&[dir.path().to_path_buf()]).unwrap();

        assert_eq!(result, vec![text]);
    }

    #[test]
    fn recursive_sorts() {
        let dir = tempdir();

        let z = dir.path().join("z.txt");
        let a = dir.path().join("a.txt");
        let m = dir.path().join("m.txt");

        write(&z, "z").unwrap();
        write(&a, "a").unwrap();
        write(&m, "m").unwrap();

        let result = recursive_path(&[dir.path().to_path_buf()]).unwrap();

        assert_eq!(result, vec![a, m, z]);
    }

    #[test]
    fn recursive_single_file() {
        let dir = tempdir();
        let file = dir.path().join("test.txt");

        write(&file, "hello").unwrap();

        let result = recursive_path(std::slice::from_ref(&file)).unwrap();

        assert_eq!(result, vec![file]);
    }

    #[test]
    fn recursive_empty_directory() {
        let dir = tempdir();

        let result = recursive_path(&[dir.path().to_path_buf()]).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn recursive_multiple_paths() {
        let dir = tempdir();

        let dir1 = dir.path().join("one");
        let dir2 = dir.path().join("two");

        create_dir_all(&dir1).unwrap();
        create_dir_all(&dir2).unwrap();

        let file1 = dir1.join("one.txt");
        let file2 = dir2.join("two.txt");

        write(&file1, "one").unwrap();
        write(&file2, "two").unwrap();

        let result = recursive_path(&[dir1, dir2]).unwrap();

        assert_eq!(result.len(), 2);
        assert!(result.contains(&file1));
        assert!(result.contains(&file2));
    }
}
