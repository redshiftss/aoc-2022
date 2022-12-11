// use std::{fs, collections::HashSet, cmp::max};

// #[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
// struct Vector {
//     x : i32,
//     y : i32,
// }

// impl Vector {
//     fn distance(self, other : Vector) -> i32{
//         max(self.x.abs_diff(other.x), self.y.abs_diff(other.y)) as i32
//     }
// }

// fn part_1(file: &str) {
//     let contents = fs::read_to_string(file).expect("Should have been able to read the file");
//     let moves = contents.lines();
//     let mut curr_head = Vector{x : 0, y : 0};
//     let mut curr_tail = Vector{x : 0, y : 0};
//     let mut tail_positions : HashSet<Vector> = HashSet::new();
//     tail_positions.insert(curr_tail);

//     for mv in moves {
//         dbg!(mv);
//         let direction = mv.split(" ").collect::<Vec<_>>()[0];
//         let magnitude : i32 = mv.split(" ").collect::<Vec<_>>()[1].parse().unwrap();
//         let dv = match direction {
//             "U" => Vector{ x : 0, y : 1},
//             "D" => Vector { x: 0, y: -1 },
//             "L" => Vector { x: -1, y: 0},
//             "R" => Vector { x: 1, y: 0},
//             _ => panic!("invalid dir")
//         };
//         for i in 0..magnitude{
//             let new_pos = make_move(dv, curr_head, curr_tail);
//             curr_head = new_pos.0;
//             curr_tail = new_pos.1;
//             tail_positions.insert(curr_tail);
//         }
//     }
//     // dbg!(tail_positions.len());
// }

// fn prop_tail(upd_head : Vector, curr_tail : Vector) -> (Vector, Vector) {
//     let mut upd_tail = curr_tail;

//     if upd_head.distance(curr_tail) <= 1{
//         upd_tail = curr_tail;
//         return (upd_head, upd_tail)
//     }

//     if upd_head.distance(curr_tail) >= 2 {
//         // deal with x first
//         let mut new_x = curr_tail.x;
//         let direction_x = if (upd_head.x - curr_tail.x).is_negative() {-1} else {1};
//         if upd_head.x.abs_diff(new_x) > 1 {
//             new_x += direction_x;
//         }
//         // deal with y second
//         let mut new_y = curr_tail.y;
//         let direction_y = if (upd_head.y - curr_tail.y).is_negative() {-1} else {1};
//         if upd_head.y.abs_diff(new_y) > 1 {
//             new_y += direction_y;
//         }
//         upd_tail = Vector{x : new_x, y : new_y};
//         return (upd_head, upd_tail)
//     }
//     // let upd_tail = Vector{x : curr_tail.x + direction.x, y : curr_tail.y + direction.y};
//     unreachable!();
// }

// fn part_2(file: &str) {
//     let contents = fs::read_to_string(file).expect("Should have been able to read the file");
//     let moves = contents.lines();
//     //tails vector of 0 is head, last one is tail
//     let mut tails_vector : Vec<Vector> = vec![Vector {x:0, y:0}; 10];
//     let mut tail_positions : HashSet<Vector> = HashSet::new();
//     tail_positions.insert(tails_vector[9]);

//     for mv in moves {
//         dbg!(mv);
//         let direction = mv.split(" ").collect::<Vec<_>>()[0];
//         let magnitude : i32 = mv.split(" ").collect::<Vec<_>>()[1].parse().unwrap();
//         let dv = match direction {
//             "U" => Vector{ x : 0, y : 1},
//             "D" => Vector { x: 0, y: -1 },
//             "L" => Vector { x: -1, y: 0},
//             "R" => Vector { x: 1, y: 0},
//             _ => panic!("invalid dir")
//         };

//         for i in 0..magnitude{
//             dbg!("aa");

//             let pos = make_move(dv, tails_vector[0], tails_vector[1]);
//             print(&tails_vector);
//             tails_vector[0] = pos.0;
//             // tails_vector[1] = pos.1;
//             for j in 0..9 {
//                 let pos = prop_tail(tails_vector[j], tails_vector[j + 1]);
//                 tails_vector[j + 1] = pos.1;
//             }

//             tail_positions.insert(tails_vector[9]);
//         }

//     }

//     dbg!(tail_positions.len());

// }

// fn print(tail_positions : &Vec<Vector>) {
//     for i in 0..10{
//         for j in 0..10{
//             let v = Vector{x: j, y: i};
//             if tail_positions.contains(&v){
//                 print!("*");
//             }else{
//                 print!(".")
//             }
//         }
//         println!("")
//     }
//     println!("")
// }

// pub fn run_day_9(file: &str){
//     part_1(file);
//     part_2(file);
// }
