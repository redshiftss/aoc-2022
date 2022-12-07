use std::fs;
use std::collections::HashSet;

fn part_1(file: &str) {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    let idx = calc_marker(4, &contents);
    println!("{}", idx)
}

fn part_2(file: &str) {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    let idx = calc_marker(14, &contents);
    println!("{}", idx)
}

fn calc_marker(d : i32, stream : &str) -> i32 {
    for i in d..(stream.len() as i32){
        let mut set = HashSet::new();
        for j in 0..d {
            set.insert(stream.chars().nth((i - j).try_into().unwrap()).unwrap());
        }
        if set.len() == d as usize {
            return i + 1
        }
    }
    -1
}

pub fn run_day_6(file: &str){
    part_1(file);
    part_2(file);
}