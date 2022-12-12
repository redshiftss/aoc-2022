use std::{fs, vec, collections::{VecDeque, HashSet, BinaryHeap}};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Node {
    val : char,
    x : i32,
    y : i32,
}

#[derive(Clone, Eq, PartialEq)]
struct QueueNode {
    distance : i32,
    n : Node,
}

impl PartialOrd for QueueNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}



fn part_1(file: &str) {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    let map : Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect()).collect();
    let mut nodes = vec![];
    let mut visited = HashSet::new();

    let mut prioqueue = BinaryHeap::new();

    for i in 0..map.len() as usize {
        for j in 0.. map[1].len() {
            let mut n = Node{val: map[i][j], x: i as i32, y: j as i32};
            let mut distance = i32::MAX;
            if n.val == 'S' {
                distance = 0;
            }
            nodes.push(n);

            prioqueue.push(QueueNode { distance, n });
        }
    }
    let nodes_iter = nodes.into_iter();
    
    while let Some(qn) = prioqueue.pop(){
        if visited.contains(&qn.n){
            continue
        }
        visited.insert(qn.n);
        let mut neighbors = HashSet::new();

        if qn.n.val == 'E' {
            dbg!(qn.distance );
            return
        }
        let qni = qn.n.x;
        let qnj = qn.n.y;

        for di in -1..2{
            for dj in -1..2{
                if di * dj == 0{
                    let nb = nodes_iter.clone().filter(|node| node.x == qni + di && node.y == qnj + dj);
                    // dbg!(&nb);
                    nb.for_each(|x| { neighbors.insert(x); });
                }   
            }
        }

        neighbors.remove(&qn.n);
        let qv = qn.n.val;
        
        let legal_neighbors = neighbors.into_iter().filter(|node| is_legal_move(qv, node.val));
        

        let dist = qn.distance;
        for node in legal_neighbors {
            prioqueue.push(QueueNode{ distance: dist + 1,  n: node })
        }
    }
} 

fn is_legal_move(curr : char, other : char) -> bool {
    if curr == 'S'{
        return other == 'a'
    }
    if other == 'E'{
        return curr == 'z'
    }
    if (other as i32 - curr as i32) <= 1 {
        return true
    }

    false
}

fn part_2(file: &str) {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    let map : Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect()).collect();
    let mut nodes = vec![];
    let mut visited = HashSet::new();

    let mut prioqueue = BinaryHeap::new();

    for i in 0..map.len() as usize {
        for j in 0.. map[1].len() {
            let mut n = Node{val: map[i][j], x: i as i32, y: j as i32};
            let mut distance = i32::MAX;
            if n.val == 'E' {
                distance = 0;
            }
            nodes.push(n);

            prioqueue.push(QueueNode { distance, n });
        }
    }
    let nodes_iter = nodes.into_iter();
    
    while let Some(qn) = prioqueue.pop(){
        if visited.contains(&qn.n){
            continue
        }
        visited.insert(qn.n);
        let mut neighbors = HashSet::new();

        if qn.n.val == 'a' {
            dbg!(qn.distance );
            return
        }
        let qni = qn.n.x;
        let qnj = qn.n.y;

        for di in -1..2{
            for dj in -1..2{
                if di * dj == 0{
                    let nb = nodes_iter.clone().filter(|node| node.x == qni + di && node.y == qnj + dj);
                    // dbg!(&nb);
                    nb.for_each(|x| { neighbors.insert(x); });
                }   
            }
        }

        neighbors.remove(&qn.n);
        let qv = qn.n.val;
        
        let legal_neighbors = neighbors.into_iter().filter(|node| is_legal_move_rev(qv, node.val));
        

        let dist = qn.distance;
        for node in legal_neighbors {
            prioqueue.push(QueueNode{ distance: dist + 1,  n: node })
        }
    }
    
}

fn is_legal_move_rev(curr : char, other : char) -> bool {
    if curr == 'E'{
        return other == 'z'
    }
    if (curr as i32 - other as i32) <= 1 {
        return true
    }
    false
}

pub fn run_day_12(file: &str){
    part_1(file);
    part_2(file);
}