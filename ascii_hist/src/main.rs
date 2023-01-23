use std::fs;
use std::collections::HashMap;

fn ascii_histogram(slice: &str) -> HashMap<char, usize> {
    let mut hist_map = HashMap::new();

    for ch in slice.chars() {
        *hist_map.entry(ch).or_insert(0) += 1;
    }

    hist_map
}

fn main() {
    let input = fs::read_to_string("./bird.txt")
        .expect("Unable to read file.");

    println!("{:?}", ascii_histogram(&input));
}
