use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut reader = BufReader::new(File::open(&args[1]).expect("File not found"));
    let mut buffer = String::new();
    let mut safe = 0;
    let mut safe_damped = 0;
    'outer: while let Ok(size) = reader.read_line(&mut buffer) {
        if size == 0 || buffer.trim().is_empty() {
            break;
        }
        let levels: Vec<i64> = buffer
            .trim()
            .split(" ")
            .map(|r| r.parse().expect("Invalid value"))
            .collect();
        // let line = buffer.trim().to_owned();
        buffer.clear();
        let mut increase = levels[0] < levels[1];
        let mut current = levels[0];
        let mut damping = true;
        if !level_safe(levels[0], levels[1], increase) {
            damping = false;
            increase = levels[0] < levels[2];
        } else {
            current = levels[1];
        }
        // println!("{}", line.trim());
        for i in 2..levels.len() {
            let problem = !level_safe(current, levels[i], increase);
            if problem && !damping {
                continue 'outer;
            }
            if problem && damping {
                // println!("Damping on i {i}");
                if i == 2 && levels.len() > 2 {
                    let new_increase = levels[i - 2] < levels[i];
                    let first_two_safe = level_safe(levels[i - 2], levels[i], new_increase)
                        && level_safe(levels[i], levels[i + 1], new_increase);
                    if first_two_safe {
                        increase = new_increase;
                    }
                }
                if level_safe(levels[i - 2], levels[i], increase) {
                    // println!("Skipping {current}");
                    current = levels[i];
                }
                if i + 1 < levels.len() && level_safe(levels[i - 1], levels[i + 1], increase) {
                    current = levels[i - 1]
                }
                damping = false;
                continue;
            }
            current = levels[i]
        }

        // No damping used
        if damping {
            safe += 1;
        }
        safe_damped += 1;
    }
    println!("Safe: {safe}");
    println!("Safe (problem dampener): {safe_damped}");
}

fn level_safe(current: i64, next: i64, increase: bool) -> bool {
    if (current < next) != increase {
        return false;
    }
    let distance = (next - current).abs();
    if distance <= 0 || distance > 3 {
        return false;
    }
    return true;
}
