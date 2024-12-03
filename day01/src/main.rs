use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut reader = BufReader::new(File::open(&args[1]).expect("invalid file"));
    let mut buffer = String::new();
    let mut firsts = BinaryHeap::new();
    let mut seconds = BinaryHeap::new();
    let mut similarities = HashMap::new();

    while let Ok(size) = reader.read_line(&mut buffer) {
        if size == 0 || buffer.trim().is_empty() {
            break;
        }
        let mut split = buffer.trim().split("   ");
        let first: i64 = split
            .next()
            .expect("Invalid line")
            .parse()
            .expect("Invalid int");
        let second: i64 = split
            .next()
            .expect("Invalid line")
            .parse()
            .expect("Invalid int");
        firsts.push(Reverse(first));
        seconds.push(Reverse(second));
        similarities
            .entry(second)
            .and_modify(|counter| *counter += 1i64)
            .or_insert(1);
        buffer.clear();
    }

    assert_eq!(firsts.len(), seconds.len());
    let mut total = 0i64;
    let mut similarity = 0i64;
    while let (Some(Reverse(first)), Some(Reverse(second))) = (firsts.pop(), seconds.pop()) {
        let multiplier = similarities.entry(first).or_default();
        total += (second - first).abs();
        similarity += first * *multiplier;
    }
    println!("Total: {total}");
    println!("Similarity: {similarity}");
}
