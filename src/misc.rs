use std::collections::HashSet;

pub fn remove_duplicate<T: std::hash::Hash + std::cmp::Eq>(vec: Vec<T>) -> Vec<T> {
    HashSet::<T>::from_iter(vec.into_iter()).into_iter().collect()
}

pub fn intersect<T: std::hash::Hash + std::cmp::Eq + Copy>(v1: Vec<T>, v2: Vec<T>) -> Vec<T> {
    HashSet::<T>::from_iter(v1.into_iter()).intersection(&HashSet::<T>::from_iter(v2.into_iter())).map(|e| *e).collect()
}

pub fn check(haystack: &[u8], index: usize, needle: &[u8]) -> bool {
    &haystack[index..(index + needle.len())] == needle
}