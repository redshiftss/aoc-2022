fn part_1(file: &str) {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    let elves: Vec<&str> = contents.split("\n\n").collect();

    let mut max_elf : Option<usize> = None;

    for elf in elves{
        let elf_backpack : Vec<usize>= elf.split('\n').map(|s| s.parse().unwrap()).collect();
        let elf_calorie_count = elf_backpack.into_iter().reduce(|a, x| {x + a}).unwrap();

        max_elf = match max_elf {
            None =>  Some(elf_calorie_count),
            Some(food) if food < elf_calorie_count => Some(elf_calorie_count), 
            Some(food) => Some(food),
        }
    }
    print!("{}", max_elf.unwrap())
}


fn part_2(file: &str) {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    let elves: Vec<&str> = contents.split("\n\n").collect();

    let mut elf_backpacks : Vec<usize> = vec![];

    for elf in elves{
        let elf_backpack : Vec<usize>= elf.split('\n').map(|s| s.parse().unwrap()).collect();
        let elf_calorie_count = elf_backpack.into_iter().reduce(|a, x| {x + a}).unwrap();
        elf_backpacks.push(elf_calorie_count);
    }
    
    elf_backpacks.sort_by(|a, b| b.cmp(a));
    let sum = elf_backpacks[0] + elf_backpacks[1] + elf_backpacks[2];
    println!("{}", sum)
}

pub fn run_day_1(file: &str){
    part_1(file);
    part_2(file);
}