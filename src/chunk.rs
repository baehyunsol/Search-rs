use crate::index::file::FileIndex;

pub const MAX_CHUNK_COUNT: usize = 0x10_000_000;
pub const CHUNK_SIZE: usize = 1024 + 64;
const OVERLAP: usize = 64;

impl FileIndex {

    pub fn from_chunk_index(&self, chunk_index: u64) -> (String, usize) {
        (self.files[(chunk_index / MAX_CHUNK_COUNT as u64) as usize].clone(), (chunk_index % MAX_CHUNK_COUNT as u64) as usize)
    }

    pub fn to_chunk_index(&self, file_name: String, chunk: usize) -> u64 {
        *self.rev_files.get(&file_name).unwrap() as u64 * MAX_CHUNK_COUNT as u64 + chunk as u64
    }

}

pub fn get_chunk_index(length: usize) -> Vec<usize> {

    if length <= CHUNK_SIZE - OVERLAP {
        vec![0]
    }

    else {
        let mut e = length / (CHUNK_SIZE - OVERLAP);

        if length % (CHUNK_SIZE - OVERLAP) != 0 {
            e += 1;
        }

        (0..e).map(
            |i| i * (CHUNK_SIZE - OVERLAP)
        ).collect()
    }

}

#[cfg(test)]
mod tests {
    use super::{CHUNK_SIZE, OVERLAP, get_chunk_index};

    #[test]
    fn chunk_test() {
        // if I change those constants, the asserts below will alert me.
        assert_eq!(CHUNK_SIZE, 1024 + 64);
        assert_eq!(OVERLAP, 64);

        assert_eq!(get_chunk_index(1023), vec![0]);
        assert_eq!(get_chunk_index(1024), vec![0]);
        assert_eq!(get_chunk_index(1025), vec![0, 1024]);
        assert_eq!(get_chunk_index(1088), vec![0, 1024]);
        assert_eq!(get_chunk_index(2047), vec![0, 1024]);
        assert_eq!(get_chunk_index(2048), vec![0, 1024]);
        assert_eq!(get_chunk_index(2049), vec![0, 1024, 2048]);
        assert_eq!(get_chunk_index(2176), vec![0, 1024, 2048]);
        assert_eq!(get_chunk_index(3071), vec![0, 1024, 2048]);
        assert_eq!(get_chunk_index(3072), vec![0, 1024, 2048]);
        assert_eq!(get_chunk_index(3073), vec![0, 1024, 2048, 3072]);
        assert_eq!(get_chunk_index(3264), vec![0, 1024, 2048, 3072]);
    }

}