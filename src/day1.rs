use crate::utils;

pub fn part1() {
    let input = utils::read_input("src/input/day1.txt");

    let mut sum = 0;
    for idx in 0..input.len() {
        let n1 = input
            .chars()
            .nth(idx)
            .unwrap()
            .to_string()
            .parse::<i32>()
            .unwrap();
        let n2 = (|| {
            let mut next_idx = idx + 1;
            if idx == input.len() - 1 {
                next_idx = 0
            }
            return input
                .chars()
                .nth(next_idx)
                .unwrap()
                .to_string()
                .parse::<i32>()
                .unwrap();
        })();
        if n1 == n2 {
            sum += n1
        }
    }

    println!("part1: {}", sum)
}

pub fn part2() {
    let input = utils::read_input("src/input/day1.txt");

    let mut sum = 0;
    let half_len = input.len() / 2;
    for idx in 0..input.len() {
        let n1 = input
            .chars()
            .nth(idx)
            .unwrap()
            .to_string()
            .parse::<i32>()
            .unwrap();
        let n2 = (|| {
            let mut next_idx = idx + half_len;
            if idx >= input.len() - half_len {
                next_idx = (idx + half_len) - input.len()
            }
            return input
                .chars()
                .nth(next_idx)
                .unwrap()
                .to_string()
                .parse::<i32>()
                .unwrap();
        })();
        if n1 == n2 {
            sum += n1
        }
    }

    println!("part2: {}", sum)
}
