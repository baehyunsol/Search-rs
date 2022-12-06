use crate::misc::check;

pub fn hash_string_5(string: &[u8]) -> Vec<u32> {

    if string.len() < 5 {
        return vec![];
    }

    let mut result = Vec::with_capacity(string.len() - 4);

    let mut curr_hash = hash_at_5(string, 0);

    for i in 5..string.len() {
        result.push(curr_hash);
        curr_hash /= 64;
        curr_hash += (string[i] % 64) as u32 * 0x1_000_000;
    }

    result.push(curr_hash);
    result
}

#[inline]
pub fn hash_at_5(string: &[u8], index: usize) -> u32 {
    (string[index] % 64) as u32
    + (string[index + 1] % 64) as u32 * 64
    + (string[index + 2] % 64) as u32 * 0x1_000
    + (string[index + 3] % 64) as u32 * 0x40_000
    + (string[index + 4] % 64) as u32 * 0x1_000_000
}

pub fn search_at_5(haystack: &[u8], index: usize, needle: &[u8]) -> Vec<usize> {

    if needle.len() < 5 {
        return vec![];
    }

    let answer_hash = hash_at_5(needle, 0);
    let mut curr_hash = hash_at_5(haystack, index);
    let mut result = vec![];

    for i in (index + 5)..(haystack.len() - needle.len() + 5) {

        if curr_hash == answer_hash && check(haystack, i - 5, needle) {
            result.push(i - 5);
        }

        curr_hash /= 64;
        curr_hash += (haystack[i] % 64) as u32 * 0x1_000_000;
    }

    if curr_hash == answer_hash {
        result.push(haystack.len() - needle.len());
    }

    result
}

pub fn hash_string_3(string: &[u8]) -> Vec<u32> {

    if string.len() < 3 {
        return vec![];
    }

    let mut result = Vec::with_capacity(string.len() - 2);

    let mut curr_hash = hash_at_3(string, 0);

    for i in 3..string.len() {
        result.push(curr_hash);
        curr_hash /= 256;
        curr_hash += string[i] as u32 * 0x10_000;
    }

    result.push(curr_hash);
    result
}

#[inline]
pub fn hash_at_3(string: &[u8], index: usize) -> u32 {
    string[index] as u32
    + string[index + 1] as u32 * 0x100
    + string[index + 2] as u32 * 0x10_000
}

pub fn search_at_3(haystack: &[u8], index: usize, needle: &[u8]) -> Vec<usize> {

    if needle.len() < 3 {
        return vec![];
    }

    let answer_hash = hash_at_3(needle, 0);
    let mut curr_hash = hash_at_3(haystack, index);
    let mut result = vec![];

    for i in (index + 3)..(haystack.len() - needle.len() + 3) {

        if curr_hash == answer_hash && check(haystack, i - 3, needle) {
            result.push(i - 3);
        }

        curr_hash /= 256;
        curr_hash += haystack[i] as u32 * 0x10_000;
    }

    if curr_hash == answer_hash {
        result.push(haystack.len() - needle.len());
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::testbench::*;

    #[test]
    fn hash_test() {
        let test_cases = vec![
            // (case, hash_5, hash_3)
            (vec![0, 0, 0, 0, 0], vec![0], vec![0, 0, 0]),
            (vec![1, 0, 0, 1, 0], vec![1 + 0x40_000], vec![1, 0x10_000, 256]),
            (vec![0, 1, 0, 0, 0], vec![64], vec![256, 1, 0]),
            (vec![0, 0, 0], vec![], vec![0]),
            (vec![0, 1, 1, 0, 0, 0, 1], vec![64 + 4096, 1 + 64, 1 + 0x1_000_000], vec![256 + 65536, 1 + 256, 1, 0, 65536]),
        ];

        for (test_case, hash_5_answer, hash_3_answer) in test_cases.into_iter() {
            assert_eq!(super::hash_string_5(&test_case), hash_5_answer);
            assert_eq!(super::hash_string_3(&test_case), hash_3_answer);
        }

    }

    #[test]
    fn rabin_karp_test() {

        for (haystack, needles, answers) in test_cases() {

            for i in 0..needles.len() {

                if needles[i].len() > 4 {
                    assert_eq!(super::search_at_5(&haystack, 0, &needles[i]), answers[i]);
                }

                if needles[i].len() > 2 {
                    assert_eq!(super::search_at_3(&haystack, 0, &needles[i]), answers[i]);
                }

            }

        }

    }

}