use crate::misc::check;
use crate::testbench::test_cases;

pub fn naive(haystack: &[u8], needle: &[u8]) -> Vec<usize> {
    (0..haystack.len() - needle.len() + 1).filter(|index| check(haystack, *index, needle)).collect()
}

#[test]
fn naive_test() {

    for (haystack, needles, answers) in test_cases() {

        for i in 0..needles.len() {
            assert_eq!(naive(&haystack, &needles[i]), answers[i]);
        }

    }

}