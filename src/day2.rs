use std::f32::{INFINITY, NEG_INFINITY};

use crate::utils;

pub fn part1() {
    let input = utils::read_input("src/input/day2.txt");

    let mut sum = 0;
    input.split("\n").for_each(|line| {
        let (min, max) = line
            .split("\t")
            .fold((INFINITY as i32, NEG_INFINITY as i32), |acc, e| {
                let n = e.parse::<i32>().expect("could not parse string to number");
                match acc {
                    (l, r) if n < l && n > r => {
                        return (n, n);
                    }
                    (l, r) if n < l => {
                        return (n, r);
                    }
                    (l, r) if n > r => {
                        return (l, n);
                    }
                    (l, r) => return (l, r),
                }
            });
        sum += max - min
    });

    println!("part1: {sum}")
}

fn evenly_div_res(a: i32, b: i32) -> i32 {
    if a % b == 0 {
        return (a / b) as i32;
    };
    if b % a == 0 {
        return (b / a) as i32;
    };
    return 0;
}

pub fn part2() {
    let input = utils::read_input("src/input/day2.txt");

    let mut sum = 0;
    input.split("\n").for_each(|line| {
        for (i1, e1) in line.split("\t").enumerate() {
            let n1 = e1.parse::<i32>().expect("could not parse string to number");
            let prev_sum = sum;

            for (i2, e2) in line.split("\t").enumerate() {
                let n2 = e2.parse::<i32>().expect("could not parse string to number");
                if i2 == i1 {
                    continue;
                }
                let res = evenly_div_res(n1, n2);
                if res > 0 {
                    sum += res;
                    break;
                }
            }
            if prev_sum != sum {
                break;
            }
        }
    });

    println!("part2: {sum}")
}
