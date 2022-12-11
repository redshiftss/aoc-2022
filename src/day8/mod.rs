use std::fs;

fn part_1(file: &str) {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    let lines = contents.split('\n');

    let trees: Vec<_> = lines
        .map(|line| {
            line.split("")
                .filter(|x| *x != "")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    let height = trees.len();
    let width = trees[0].len();
    let mut count = (height - 1) * 2 + (width - 1) * 2;
    for i in 1..height - 1 {
        for j in 1..width - 1 {
            if isVisibleTree(trees.clone(), j, i) {
                count += 1;
            }
        }
    }
    dbg!(count);
}

fn isVisibleTree(trees: Vec<Vec<i32>>, x: usize, y: usize) -> bool {
    let b = isVisibleFromEast(trees.clone(), x, y)
        || isVisibleFromNorth(trees.clone(), x, y)
        || isVisibleFromSouth(trees.clone(), x, y)
        || isVisibleFromWest(trees.clone(), x, y);
    b
}

fn isVisibleFromNorth(trees: Vec<Vec<i32>>, x: usize, y: usize) -> bool {
    let w = x;
    let tree = trees[y][x];
    for i in 0..y {
        if tree <= trees[i][w] {
            return false;
        }
    }
    true
}

fn isVisibleFromSouth(trees: Vec<Vec<i32>>, x: usize, y: usize) -> bool {
    let w = x;
    let tree = trees[y][x];
    //check all trees on the vertical above our tree
    for i in y + 1..trees.len() {
        if tree <= trees[i][w] {
            return false;
        }
    }
    true
}

fn isVisibleFromWest(trees: Vec<Vec<i32>>, x: usize, y: usize) -> bool {
    let w = x;
    let tree = trees[y][x];
    let h = y;
    for j in 0..x {
        if tree <= trees[h][j] {
            return false;
        }
    }
    true
}

fn isVisibleFromEast(trees: Vec<Vec<i32>>, x: usize, y: usize) -> bool {
    let w = x;
    let tree = trees[y][x];
    let h = y;
    for j in x + 1..trees[0].len() {
        if tree <= trees[h][j] {
            return false;
        }
    }
    true
}

fn part_2(file: &str) {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    let lines = contents.split('\n');

    let trees: Vec<_> = lines
        .map(|line| {
            line.split("")
                .filter(|x| *x != "")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    let height = trees.len();
    let width = trees[0].len();
    // let mut count = (height - 1) * 2 + (width - 1) * 2;
    let mut max = -1;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let scenic_score = scenicScoreEast(trees.clone(), x, y)
                * scenicScoreNorth(trees.clone(), x, y)
                * scenicScoreWest(trees.clone(), x, y)
                * scenicScoreSouth(trees.clone(), x, y);
            if scenic_score > max {
                max = scenic_score
            }
        }
    }
    dbg!(max);
}

fn scenicScoreNorth(trees: Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let w = x;
    let tree = trees[y][x];
    let mut count = 0;
    for i in (0..y).rev() {
        count += 1;
        if tree <= trees[i][w] {
            return count;
        }
    }
    count
}

fn scenicScoreSouth(trees: Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let w = x;
    let tree = trees[y][x];
    let mut count = 0;
    for i in y + 1..trees.len() {
        count += 1;
        if tree <= trees[i][w] {
            return count;
        }
    }
    count
}

fn scenicScoreWest(trees: Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let w = x;
    let tree = trees[y][x];
    let h = y;
    let mut count = 0;
    for j in (0..x).rev() {
        count += 1;
        if tree <= trees[h][j] {
            return count;
        }
    }
    count
}

fn scenicScoreEast(trees: Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let w = x;
    let tree = trees[y][x];
    let h = y;
    let mut count = 0;
    for j in x + 1..trees[0].len() {
        count += 1;
        if tree <= trees[h][j] {
            return count;
        }
    }
    count
}

pub fn run_day_8(file: &str) {
    part_1(file);
    part_2(file);
}
