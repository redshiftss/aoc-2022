use std::{fs, usize};

fn part_1(file: &str) {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    let pairs = contents.lines();
    let number: usize = pairs
        .map(|pair| {
            let mut ranges = pair.split(",");
            let range1: Vec<_> = ranges
                .next()
                .unwrap()
                .split("-")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let range2: Vec<_> = ranges
                .next()
                .unwrap()
                .split("-")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            dbg!(range1.clone());

            if (range1[0] <= range2[0] && range1[1] >= range2[1])
                || (range1[0] >= range2[0] && range1[1] <= range2[1])
            {
                1
            } else {
                0
            }
        })
        .sum();
    println!("part 1: {}", number)
}

fn part_2(file: &str) {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    let pairs = contents.lines();
    let number: usize = pairs
        .map(|pair| {
            let mut ranges = pair.split(",");
            let range1: Vec<_> = ranges
                .next()
                .unwrap()
                .split("-")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let range2: Vec<_> = ranges
                .next()
                .unwrap()
                .split("-")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            dbg!(range1.clone());

            if (range1[0] <= range2[0] && range1[1] >= range2[1])
                || (range1[0] >= range2[0] && range1[1] <= range2[1])
                || (range1[1] >= range2[0] && range1[0] < range2[1])
                || (range2[1] >= range1[0] && range2[0] < range1[1])
            {
                1
            } else {
                0
            }
        })
        .sum();
    println!("part 2: {}", number)
}

pub fn run_day_4(file: &str) {
    part_1(file);
    part_2(file);
}
