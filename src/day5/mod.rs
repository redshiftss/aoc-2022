use std::{fs, vec};

fn parse(file: &str) -> (Vec<Vec<char>>, Vec<(i32, i32, i32)>) {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    let crane_setup: Vec<&str> = contents.split("\n\n").collect();

    let lines: Vec<&str> = crane_setup[0].lines().collect();
    let mut stacks: Vec<Vec<char>> = vec![];
    stacks.push(vec![]);

    (1..10).for_each(|idx| {
        let v: Vec<char> = lines
            .clone()
            .into_iter()
            .map(|l| l.chars().nth((idx - 1) * 4 + 1).unwrap())
            .filter(|x| x.is_alphabetic())
            .rev()
            .collect();
        stacks.push(v);
    });

    let moves: Vec<(i32, i32, i32)> = crane_setup[1]
        .lines()
        .map(|x| {
            let spl: Vec<&str> = x.split_whitespace().collect();
            (spl[1].parse().unwrap(), spl[3].parse().unwrap(), spl[5].parse().unwrap())
        })
        .collect();
    
    
    (stacks, moves)
}

fn part_1(file: &str) {
    let (mut stacks, moves) = parse(file);
    for mv in moves{
        let from = mv.1;
        let to = mv.2;
        
        (0..mv.0).for_each(|_|{
            let item = stacks[from as usize].pop().unwrap();
            stacks[to as usize].push(item);
        });
    }
    
    for mut stack in stacks {
        if !stack.is_empty(){
            print!("{}", stack.pop().unwrap());
        }
    }
    println!("");
}

fn part_2(file: &str) {
    let (mut stacks, moves) = parse(file);
    for mv in moves{
        let from = mv.1;
        let to = mv.2;
        let mut items = vec![];

        (0..mv.0).for_each(|_|{
            items.push(stacks[from as usize].pop().unwrap());
        });
        items.reverse();
        stacks[to as usize].append(&mut items);
    }
    
    for mut stack in stacks {
        if !stack.is_empty(){
            print!("{}", stack.pop().unwrap());
        }
    }
}

pub fn run_day_5(file: &str) {
    part_1(file);
    part_2(file);
}
