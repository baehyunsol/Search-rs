use crate::file::*;
use hserde::HSerde;
use std::collections::HashMap;

// It has all the info related to searching
#[derive(Clone)]
pub struct FileIndex {
    // usize -> FileName
    pub files: Vec<String>,

    // FileName -> usize
    pub rev_files: HashMap<String, usize>,

    pub db_path: String
}

impl FileIndex {

    pub fn empty() -> FileIndex {
        FileIndex {
            files: vec![],
            rev_files: HashMap::new(),
            db_path: String::new()
        }
    }

    // make an index of the files in `./dir/`. It also recursively visits its sub-dirs.
    pub fn init_dir(dir: String) -> Result<FileIndex, ()> {
        // `rmdir` may not be unwrapped!! -> if `dir/.index` doesn't exist, it returns an error but it's fine.
        rmdir(&join(&dir, ".index")?);
        mkdir(&join(&dir, ".index")?)?;
        let files = collect_file_recursively(read_dir(&dir)?, &mut vec![])?;
        let mut rev_files = HashMap::with_capacity(files.len());

        for (index, file) in files.iter().enumerate() {
            rev_files.insert(file.to_string(), index);
        }

        let files_serialized = files.to_bytes();

        write_to_file(&join(&join(&dir, ".index")?, "files")?, &files_serialized)?;

        Ok(FileIndex {
            files, rev_files,
            db_path: join(&dir, ".index/db")?
        })
    }

    pub fn read_dir(dir: String) -> Result<FileIndex, ()> {
        let files_serialized = read_bytes(&join(&join(&dir, ".index")?, "files")?)?;
        let files: Vec<String> = match HSerde::from_bytes(&files_serialized, 0) {
            Ok(f) => f,
            _ => {return Err(());}
        };
        let mut rev_files = HashMap::with_capacity(files.len());

        for (index, file) in files.iter().enumerate() {
            rev_files.insert(file.to_string(), index);
        }

        Ok(FileIndex {
            files, rev_files,
            db_path: join(&dir, ".index/db")?
        })
    }

}

fn collect_file_recursively(files_and_dirs: Vec<String>, result: &mut Vec<String>) -> Result<Vec<String>, ()> {

    for f in files_and_dirs.into_iter() {

        if is_dir(&f) {
            collect_file_recursively(read_dir(&f)?, result)?;
        }

        else {
            match get_len(&f) {
                // this search engine can search keywords that are longer than 3 characters
                Ok(l) if l > 2 => {
                    result.push(f);
                }
                _ => {}
            }
        }

    }

    Ok(result.to_vec())
}

#[cfg(test)]
mod tests {
    use crate::file::*;
    use super::FileIndex;
    
    #[test]
    fn file_index_tests() {
        file_index_test("empty", 0);
        file_index_test("collatz", 999);
        file_index_test("fast_collatz", 1);
        file_index_test("primes", 256);
        file_index_test("ipsum", 4);
    }

    fn file_index_test(dir: &str, len: usize) {

        // it writes garbage before the test
        // in order to see if it's initialized correctly
        mkdir(&format!("./test_data/{}/.index", dir));
        let garbage = vec![1, 2, 3, 4, 5, 6];
        write_to_file(&format!("./test_data/{}/.index/files", dir), &garbage).unwrap();

        let file_index1 = FileIndex::init_dir(format!("./test_data/{}", dir)).unwrap();
        let file_index2 = FileIndex::read_dir(format!("./test_data/{}", dir)).unwrap();

        assert_eq!(file_index1.files, file_index2.files);
        assert_eq!(file_index1.rev_files, file_index2.rev_files);
        assert_eq!(file_index1.files.len(), len);

        rmdir(&format!("./test_data/{}/.index", dir));
    }

}