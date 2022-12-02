use std::{fs, ops::Index};

fn part_1(file: &str) {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    let games: Vec<_> = contents.split('\n').collect();
    //rock, scissors, paper
    let winning_table = vec!["AX", "CZ", "BY"];
    let mut res = 0;
    for game in games {
        let moves: Vec<&str> = game.split(' ').collect();
        let score = game_result(moves[0], moves[1], winning_table.clone());
        let bonus = moves[1].chars().next().unwrap() as usize - 87;
        res = res + score + bonus;
    }
    print!("part 1: {}", res)
}

fn game_result(opp_move: &str, your_move: &str, winning_table: Vec<&str>) -> usize {
    let idx_o = winning_table
        .iter()
        .position(|&a| a.contains(opp_move))
        .unwrap();
    let idx_m = winning_table
        .iter()
        .position(|&a| a.contains(your_move))
        .unwrap();
    match (your_move, opp_move) {
        (_, _) if idx_m == idx_o => 3,
        (_, _) if (idx_o + 1) % winning_table.len() == idx_m => 0,
        _ => 6,
    }
}

fn part_2(file: &str) {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    let games: Vec<_> = contents.split('\n').collect();
    let winning_table = vec!["A", "C", "B"];
    let mut res = 0;
    for game in games {
        let moves: Vec<&str> = game.split(' ').collect();
        let bonus = game_result_correct(moves[0], moves[1], winning_table.clone());
        let expected = (moves[1].chars().next().unwrap() as usize - 88) * 3;
        dbg!(expected);
        dbg!(bonus);
        res = res + bonus + expected;
    }
    print!("part 2: {}", res)
}

fn game_result_correct(opp_move: &str, result: &str, winning_table: Vec<&str>) -> usize {
    let idx_o: i32 = winning_table
        .iter()
        .position(|&a| a.contains(opp_move))
        .unwrap()
        .try_into()
        .unwrap();
    let index: i32 = match result {
        "X" => (idx_o + 1).rem_euclid(winning_table.len().try_into().unwrap()),
        "Y" => idx_o.rem_euclid(winning_table.len().try_into().unwrap()),
        "Z" => (idx_o - 1).rem_euclid(winning_table.len().try_into().unwrap()),
        _ => panic!("unreachable"),
    };
    dbg!(index);
    dbg!(winning_table[index as usize]);
    let bonus = winning_table[index as usize].chars().next().unwrap() as usize - 64;
    bonus
}

pub fn run_day_2(file: &str) {
    part_1(file);
    part_2(file);
}
