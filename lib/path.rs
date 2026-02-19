// lib/path.rs
use std::path::PathBuf;

pub fn split_string_into_vec_pathbuf(str: String, c: char) -> (bool, Vec<PathBuf>) {
    let mut success = false;
    let mut vec: Vec<PathBuf> = Vec::new();
    if !str.is_empty() {
        let str_list = str.split(c);
        for i in str_list {
            vec.push(PathBuf::from(i));
        }
        success = true;
    }
    (success, vec)
}

pub fn pathbuf_to_string(path: PathBuf) -> (bool, String) {
    let mut success = false;
    let mut str = String::new();
    match path.as_path().to_str() {
        Some(s) => {
            str.push_str(s);
            success = true;
        }
        _ => (),
    }
    (success, str)
}

pub fn vec_pathbuf_to_string(vec: &Vec<PathBuf>, c: char) -> (bool, String) {
    let mut success = false;
    let mut str = String::new();
    if vec.len() > 0 {
        for i in vec {
            match i.as_path().to_str() {
                Some(s) => {
                    str.push_str(s);
                    str.push_str(c.to_string().as_str());
                }
                _ => (),
            }
        }
        // Remove the last semicolon
        str.pop();
        success = true;
    }
    (success, str)
}

pub enum PathVecType {
    OnlyFiles,
    OnlyDir,
    None,
}

pub fn verify_path_vec(vec: &Vec<PathBuf>) -> PathVecType {
    let mut path_t = PathVecType::None;
    if vec.len() <= 0 {
        return path_t;
    } else if vec.len() == 1 {
        if vec[0].is_file() {
            path_t = PathVecType::OnlyFiles;
        } else if vec[0].is_dir() {
            path_t = PathVecType::OnlyDir;
        } else {
            return path_t;
        }
    } else {
        for i in vec {
            if i.is_file() {
                path_t = PathVecType::OnlyFiles;
            } else {
                path_t = PathVecType::None;
                return path_t;
            }
        }
    }
    return path_t;
}
