use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use rayon::prelude::*;

pub fn sequential_frequency(texts: &[&str]) -> HashMap<char, usize> {
    let mut map = HashMap::new();

    for line in texts {
        for chr in line.chars().filter(|c| c.is_alphabetic()) {
            if let Some(c) = chr.to_lowercase().next() {
                (*map.entry(c).or_insert(0)) += 1;
            }
        }
    }

    map
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    input
    .chunks(worker_count)
    

}
