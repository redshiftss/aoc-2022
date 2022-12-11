use std::{collections::VecDeque, fs, cell::RefCell, ops::{DerefMut, Deref}};
use num::integer::lcm;

#[derive(Clone, Copy, Debug)]
enum Operation {
    PLUS,
    TIMES,
}

#[derive(Clone, Copy, Debug)]
enum Operand {
    Old,
    Immediate(i64),
}

#[derive(Clone, Copy, Debug)]
struct BinOp {
    operand_1: Operand,
    operand_2: Operand,
    operation: Operation,
}

#[derive(Clone, Debug)]
struct Monkey {
    item_queue: VecDeque<i64>,
    operation: BinOp,
    monkey_business: usize,
    //mod, true, false
    test: (i64, i64, i64),
}

impl BinOp {
    fn execute(self, num : i64) -> i64 {
        let op1 = match self.operand_1 {
            Operand::Old => num,
            Operand::Immediate(x) => x,
        };

        let op2 = match self.operand_2 {
            Operand::Old => num,
            Operand::Immediate(x) => x,
        };

        match self.operation {
            Operation::PLUS => op1 + op2,
            Operation::TIMES => op1 * op2,
        }
    }
}

fn part_1(file: &str) {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    let raw = contents.split("\n\n").filter(|x| !x.is_empty());
    let mut monkeys = raw.map(|monkey| {
        let rows: Vec<&str> = monkey.lines().collect();
        
        let items: VecDeque<i64> = rows[1]
            .replace("  Starting items: ", "")
            .split(", ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().expect(&format!("Unable to parse input {}", x)))
            .collect();

        let op = parse_operation(rows[2].replace("  Operation: new = ", ""));
        
        let modulo = rows[3].split_whitespace().nth(3).unwrap().parse::<i64>().unwrap();
        let to_true = rows[4].split_whitespace().nth(5).unwrap().parse::<i64>().unwrap();
        let to_false = rows[5].split_whitespace().nth(5).unwrap().parse::<i64>().unwrap();

        RefCell::new(Monkey{
            item_queue: items,
            operation: op,
            monkey_business: 0,
            test: (modulo, to_true, to_false),
        })
    }).collect::<Vec<_>>();
    let mut monkey_business = Vec::new();

    for monkey in &monkeys {
        monkey_business.push(0);
    }

    for round in 0..20 {
        for monkeycell in &monkeys {
            let mut monkey = monkeycell.borrow_mut();
            while !monkey.item_queue.is_empty(){
                let mut current_worry = monkey.item_queue.pop_front().unwrap();
                current_worry = monkey.operation.execute(current_worry);
                current_worry /= 3;
                monkey.monkey_business += 1;
                
                if current_worry % monkey.test.0 == 0{
                    monkeys[monkey.test.1 as usize].borrow_mut().deref_mut().item_queue.push_back(current_worry);
                } else {
                    monkeys[monkey.test.2 as usize].borrow_mut().deref_mut().item_queue.push_back(current_worry);
                }
            }
        }
    }

    monkeys.sort_by_key(|m| m.borrow().monkey_business);
    monkeys.reverse();
    dbg!(monkeys[0].borrow().monkey_business * monkeys[1].borrow().monkey_business);
}

fn parse_operation(s: String) -> BinOp {
    let tokens: Vec<_> = s.split(" ").filter(|x| !x.is_empty()).collect();

    let operand_1 = match tokens[0] {
        "old" => Operand::Old,
        x => Operand::Immediate(x.parse().unwrap()),
    };
    let operation = match tokens[1] {
        "+" => Operation::PLUS,
        "*" => Operation::TIMES,
        _ => panic!(),
    };
    let operand_2 = match tokens[2] {
        "old" => Operand::Old,
        x => Operand::Immediate(x.parse().unwrap()),
    };
    BinOp {
        operand_1,
        operand_2,
        operation,
    }
}



fn part_2(file: &str) {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    let raw = contents.split("\n\n").filter(|x| !x.is_empty());
    let mut monkeys = raw.map(|monkey| {
        let rows: Vec<&str> = monkey.lines().collect();
        
        let items: VecDeque<i64> = rows[1]
            .replace("  Starting items: ", "")
            .split(", ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().expect(&format!("Unable to parse input {}", x)))
            .collect();

        let op = parse_operation(rows[2].replace("  Operation: new = ", ""));
        
        let modulo = rows[3].split_whitespace().nth(3).unwrap().parse::<i64>().unwrap();
        let to_true = rows[4].split_whitespace().nth(5).unwrap().parse::<i64>().unwrap();
        let to_false = rows[5].split_whitespace().nth(5).unwrap().parse::<i64>().unwrap();

        RefCell::new(Monkey{
            item_queue: items,
            operation: op,
            monkey_business: 0,
            test: (modulo, to_true, to_false),
        })
    }).collect::<Vec<_>>();

    let mut modulo = 1;

    for monkey in &monkeys {
        modulo = lcm(monkey.deref().borrow().test.0, modulo)
    }

    for round in 0..10000 {
        for monkeycell in &monkeys {
            let mut monkey = monkeycell.borrow_mut();
            while !monkey.item_queue.is_empty(){
                let mut current_worry = monkey.item_queue.pop_front().unwrap();

                current_worry = monkey.operation.execute(current_worry) % modulo;
                monkey.monkey_business += 1;

                if current_worry % monkey.test.0 == 0 {
                    monkeys[monkey.test.1 as usize].borrow_mut().deref_mut().item_queue.push_back(current_worry);
                } else {
                    monkeys[monkey.test.2 as usize].borrow_mut().deref_mut().item_queue.push_back(current_worry);
                }
            }
        }
    }

    monkeys.sort_by_key(|m| m.borrow().monkey_business);
    monkeys.reverse();
    dbg!(monkeys[0].borrow().monkey_business * monkeys[1].borrow().monkey_business);
}

pub fn run_day_11(file: &str) {
    part_1(file);
    part_2(file);
}
