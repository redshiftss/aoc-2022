use std::collections::btree_set::Intersection;
use std::collections::HashSet;
use std::fs;

fn part_1(file: &str) {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    let rucksacks: Vec<_> = contents.split('\n').collect();
    let total_prio: usize = rucksacks
        .into_iter()
        .map(|rucksack| {
            let c1: HashSet<_> = rucksack[0..rucksack.len() / 2].chars().collect();
            let c2: HashSet<_> = rucksack[rucksack.len() / 2..rucksack.len()]
                .chars()
                .collect();
            let mut intersect = c1.intersection(&c2);
            get_prio(intersect.next().unwrap())
        })
        .sum();
    println!("part 1: {}", total_prio)
}

fn get_prio(ch: &char) -> usize {
    match ch {
        c if c.is_uppercase() => *c as usize - 38,
        c if c.is_lowercase() => *c as usize - 96,
        _ => 0,
    }
}

fn part_2(file: &str) {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    let rucksacks: Vec<_> = contents.split('\n').collect();

    let total_prio: usize = rucksacks
        .chunks(3)
        .map(|group| {
            let r1: HashSet<_> = HashSet::from_iter(group[0].chars());
            let r2: HashSet<_> = HashSet::from_iter(group[1].chars());
            let r3: HashSet<_> = HashSet::from_iter(group[2].chars());
            let i1: HashSet<char> = r1.intersection(&r2).cloned().collect();
            let i2: HashSet<char> = r2.intersection(&r3).cloned().collect();
            let mut i = i1.intersection(&i2);
            get_prio(i.next().unwrap())
        })
        .sum();
    println!("part 2: {}", total_prio)
}

pub fn run_day_3(file: &str) {
    part_1(file);
    part_2(file);
}
