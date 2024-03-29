#![allow(dead_code)]

use std::str::FromStr;
use std::fs::{self, File, read};
use std::path::PathBuf;
use std::io::{Read, Write};

// `a/b/c.d -> `d``
pub fn extension(path: &str) -> Result<String, ()> {

    match PathBuf::from_str(path) {
        Err(_) => Err(()),
        Ok(path) => match path.extension() {
            None => Err(()),
            Some(s) => match s.to_str() {
                None => Err(()),
                Some(ext) => Ok(ext.to_string())
            }
        }
    }

}

// `a/b/c.d` -> `c.d`
pub fn basename(path: &str) -> Result<String, ()> {

    match PathBuf::from_str(path) {
        Err(_) => Err(()),
        Ok(path) => match path.file_name() {
            None => Err(()),
            Some(s) => match s.to_str() {
                None => Err(()),
                Some(ext) => Ok(ext.to_string())
            }
        }
    }

}

// `a/b/c.d` -> `a/b`
pub fn parent(path: &str) -> Result<String, ()> {

    match PathBuf::from_str(path) {
        Err(_) => Err(()),
        Ok(path) => match path.parent() {
            None => Err(()),
            Some(s) => match s.to_str() {
                None => Err(()),
                Some(ext) => Ok(ext.to_string())
            }
        }
    }

}

// `a/b/c.d -> `c``
pub fn file_name(path: &str) -> Result<String, ()> {

    match PathBuf::from_str(path) {
        Err(_) => Err(()),
        Ok(path) => match path.file_stem() {
            None => Err(()),
            Some(s) => match s.to_str() {
                None => Err(()),
                Some(ext) => Ok(ext.to_string())
            }
        }
    }

}

// `a/b/, c.d` -> `a/b/c.d`
pub fn join(path: &str, child: &str) -> Result<String, ()> {

    match PathBuf::from_str(path) {
        Err(_) => Err(()),
        Ok(path) => match PathBuf::from_str(child) {
            Err(_) => Err(()),
            Ok(child) => {
                let mut path = path.clone();
                path.push(child);

                match path.to_str() {
                    None => Err(()),
                    Some(result) => Ok(result.to_string())
                }
            }
        }
    }

}

// `a/b/c.d, e` -> `a/b/c.e`
pub fn set_ext(path: &str, ext: &str) -> Result<String, ()> {

    match PathBuf::from_str(path) {
        Err(_) => Err(()),
        Ok(mut path) => if path.set_extension(ext) {
            match path.to_str() {
                None => Err(()),
                Some(result) => Ok(result.to_string())
            }
        }

        else {
            Err(())
        }

    }

}

// `a/b/c.d, e.f` -> `a/b/e.f`
pub fn set_file_name(path: &str, file: &str) -> Result<String, ()> {

    match PathBuf::from_str(path) {
        Err(_) => Err(()),
        Ok(mut path) => {
            path.set_file_name(file);

            match path.to_str() {
                None => Err(()),
                Some(result) => Ok(result.to_string())
            }

        }

    }

}

pub fn is_dir(path: &str) -> bool {

    match PathBuf::from_str(path) {
        Err(_) => false,
        Ok(path) => path.is_dir()
    }

}

pub fn is_file(path: &str) -> bool {

    match PathBuf::from_str(path) {
        Err(_) => false,
        Ok(path) => path.is_file()
    }

}

pub fn get_len(path: &str) -> Result<u64, ()> {

    match fs::metadata(path) {
        Ok(m) => Ok(m.len()),
        _ => {return Err(());}
    }

}

pub fn read_dir(path: &str) -> Result<Vec<String>, ()> {

    match fs::read_dir(path) {
        Err(_) => Err(()),
        Ok(entries) => {
            let mut result = vec![];

            for entry in entries {

                match entry {
                    Err(_) => {return Err(());}
                    Ok(e) => {
                        match e.path().to_str() {
                            None => {return Err(());}
                            Some(ee) => {
                                result.push(ee.to_string());
                            }
                        }
                    }
                }
            }

            Ok(result)
        }
    }
}

pub fn read_bytes(path: &str) -> Result<Vec<u8>, ()> {

    match read(path) {
        Ok(data) => Ok(data),
        Err(_) => Err(())
    }

}

pub fn read_string(path: &str) -> Result<String, ()> {

    let mut s = String::new();

    match File::open(path) {
        Err(_) => Err(()),
        Ok(mut f) => match f.read_to_string(&mut s) {
            Err(_) => Err(()),
            Ok(_) => Ok(s)
        }
    }

}

pub fn write_to_file(path: &str, bytes: &[u8]) -> Result<(), ()> {

    match File::create(path) {
        Err(_) => Err(()),
        Ok(mut f) => match f.write_all(bytes) {
            Err(_) => Err(()),
            Ok(_) => Ok(())
        }
    }

}

pub fn mkdir(path: &str) -> Result<(), ()> {

    match fs::create_dir_all(path) {
        Ok(_) => Ok(()),
        _ => Err(())
    }

}

pub fn rmdir(path: &str) {

    match read_dir(path) {
        Ok(cur_dir) => {

            for dir in cur_dir.iter() {

                if is_dir(dir) {
                    rmdir(&dir);
                }
        
                else {
                    fs::remove_file(dir);
                }
        
            }
        
            fs::remove_dir(path);
        },
        _ => {}
    }

}

pub fn get_sub_directories(path: &str) -> Vec<String> {

    match read_dir(path) {
        Err(_) => vec![],
        Ok(files) => files.into_iter().filter(|f| is_dir(f)).collect()
    }

}

pub fn get_sub_directories_recursive(path: &str) -> Vec<String> {

    match read_dir(path) {
        Err(_) => vec![],
        Ok(files) => {
            let sub_dirs = files.into_iter().filter(|f| is_dir(f)).collect::<Vec<String>>();

            let sub_sub = sub_dirs.iter().map(|dir| get_sub_directories_recursive(dir)).collect::<Vec<Vec<String>>>().concat();

            vec![
                sub_dirs,
                sub_sub
            ].concat()
        }
    }

}
