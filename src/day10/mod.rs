use core::num;
use std::fs;

#[derive(Eq, PartialEq)]
enum ProcessorStatus {
    BUSY,
    DONE,
}

fn part_1(file: &str) {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    let insructions = contents.lines();
    let mut X = 1;
    let mut cycles = 0;
    let mut signal_strengths = 0;

    for instr in insructions {
        let mnemonic = instr
            .split(" ")
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>()[0];
        match mnemonic {
            "noop" => {
                tick(&mut X, &mut signal_strengths, &mut cycles, &mut 0, &mut 1);
            }
            "addx" => {
                let mut count = 0;
                let num: i32 = instr
                    .split(" ")
                    .filter(|x| !x.is_empty())
                    .collect::<Vec<_>>()[1]
                    .parse()
                    .unwrap();
                while tick(
                    &mut X,
                    &mut signal_strengths,
                    &mut cycles,
                    &mut count,
                    &mut 1,
                ) == ProcessorStatus::BUSY
                {
                    // print!("doing something");
                }
                X += num;
            }
            _ => (),
        }
    }
}

fn tick(
    X: &mut i32,
    signal_Str: &mut i32,
    totalCycles: &mut i32,
    count: &mut i32,
    numCycles: &mut i32,
) -> ProcessorStatus {
    *totalCycles += 1;

    if (*totalCycles - 20) % 40 == 0 {
        *signal_Str += *totalCycles * *X;
    }

    if *count == *numCycles {
        return ProcessorStatus::DONE;
    }
    *count += 1;
    ProcessorStatus::BUSY
}

fn part_2(file: &str) {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    let insructions = contents.lines();
    let mut X = 1;
    let mut cycles = 0;
    let mut signal_strengths = 0;

    for instr in insructions {
        let mnemonic = instr
            .split(" ")
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>()[0];
        match mnemonic {
            "noop" => {
                tick_and_draw(&mut X, &mut signal_strengths, &mut cycles, &mut 0, &mut 1);
            }
            "addx" => {
                let mut count = 0;
                let num: i32 = instr
                    .split(" ")
                    .filter(|x| !x.is_empty())
                    .collect::<Vec<_>>()[1]
                    .parse()
                    .unwrap();
                while tick_and_draw(
                    &mut X,
                    &mut signal_strengths,
                    &mut cycles,
                    &mut count,
                    &mut 1,
                ) == ProcessorStatus::BUSY
                {
                    // print!("doing something");
                }
                X += num;
            }
            _ => (),
        }
    }
}

fn tick_and_draw(
    X: &mut i32,
    signal_Str: &mut i32,
    totalCycles: &mut i32,
    count: &mut i32,
    numCycles: &mut i32,
) -> ProcessorStatus {
    if (*X + 1) == *totalCycles % 40 || (*X) == *totalCycles % 40 || (*X - 1) == *totalCycles % 40 {
        print!("#")
    } else {
        print!(" ")
    }

    if (*totalCycles + 1) % 40 == 0 {
        println!()
    }
    *totalCycles += 1;

    if *count == *numCycles {
        return ProcessorStatus::DONE;
    }
    *count += 1;
    ProcessorStatus::BUSY
}

pub fn run_day_10(file: &str) {
    part_1(file);
    part_2(file);
}
