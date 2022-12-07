use core::panic;
use std::{fs::{self, File}, io::Lines, collections::HashMap};

#[derive(Debug, Clone)]
enum FileSystemObject{
    // Children(idx), Parent
    Dir(Vec<String>, Option<String>),
    File(usize),
}

fn part_1(file: &str) {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    let instrs = contents.lines();
    let mut fs: HashMap<String, FileSystemObject> = HashMap::new();
    fs.insert("/".to_owned(), FileSystemObject::Dir(vec![], None));
    let mut curr = "/".to_owned();
    
    for instr in instrs {
        let spl : Vec<&str> = instr.split(' ').collect();
        match spl[0]{
            "$" => {
                match spl[1] {
                    "cd" => {
                        if spl[2] == ".." {
                            if let FileSystemObject::Dir(_, parent) = fs.get(&curr).unwrap() {
                                let p = &parent.as_ref().unwrap();
                                curr = p.to_string();
                            } else {
                                panic!("trying to cd into a file");
                            }
                        } else {
                            curr = if curr == "/"{
                                format!("/{}", spl[2])
                            }else{
                                format!("{}/{}", curr, spl[2])
                            };
                            
                        }
                    }
                    "ls" => continue,
                    _ => panic!("this should never, ever happen")
                }
            },
            "dir" => {
                if let FileSystemObject::Dir(children, _) = fs.get_mut(&curr).unwrap() {
                    let new_child = FileSystemObject::Dir(vec![], Some(curr.to_string()));
                    // let name = curr;
                    let name = if curr == "/"{
                        format!("{}{}", curr, spl[1])
                    }else{
                        format!("{}/{}", curr, spl[1])
                    };
                    children.push(name.clone());
                    fs.insert(name, new_child);
                }else{
                    panic!("file not found");
                }

            },
            num => {
                let f = fs.get_mut(&curr).unwrap();

                if let FileSystemObject::Dir(children, _) = f {
                    let new_child = FileSystemObject::File(num.parse().unwrap());
                    let name = if curr == "/"{
                        format!("{}{}", curr, spl[1])
                    }else{
                        format!("{}/{}", curr, spl[1])
                    };
                    children.push(name.clone());
                    fs.insert(name, new_child);
                }else{
                    panic!("file not found");
                }
            }
        }
    }

    let mut x = 0;
    let mut tot = 0;

    for item in fs.clone() {
        
        match item.1{
            FileSystemObject::Dir(_, _) => {
                let prev_x = x;
                calculate_folder(item.0, fs.clone() , &mut x);
                if x - prev_x < 100000 {
                    tot += x - prev_x
                }
            },
            FileSystemObject::File(_) => (),
        }
    }
    dbg!(tot);
}

fn calculate_folder(dir : String, fs : HashMap<String, FileSystemObject>, result : &mut i32) {
    if let FileSystemObject::Dir(children, _) = fs.get(&dir).unwrap() {
        for child in children {
            match fs.get(child).unwrap() {
                FileSystemObject::Dir(_, _) => calculate_folder(child.to_string(), fs.clone(), result),
                FileSystemObject::File(val) => *result += *val as i32,
            }
        }
    } else {
        panic!("trying to cd into a file");
    }
}



fn part_2(file: &str) {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    let instrs = contents.lines();
    let mut fs: HashMap<String, FileSystemObject> = HashMap::new();
    fs.insert("/".to_owned(), FileSystemObject::Dir(vec![], None));
    let mut curr = "/".to_owned();
    
    for instr in instrs {
        let spl : Vec<&str> = instr.split(' ').collect();
        match spl[0]{
            "$" => {
                match spl[1] {
                    "cd" => {
                        if spl[2] == ".." {
                            if let FileSystemObject::Dir(_, parent) = fs.get(&curr).unwrap() {
                                let p = &parent.as_ref().unwrap();
                                curr = p.to_string();
                            } else {
                                panic!("trying to cd into a file");
                            }
                        } else {
                            curr = if curr == "/"{
                                format!("/{}", spl[2])
                            }else{
                                format!("{}/{}", curr, spl[2])
                            };
                            
                        }
                    }
                    "ls" => continue,
                    _ => panic!("this should never, ever happen")
                }
            },
            "dir" => {
                if let FileSystemObject::Dir(children, _) = fs.get_mut(&curr).unwrap() {
                    let new_child = FileSystemObject::Dir(vec![], Some(curr.to_string()));
                    // let name = curr;
                    let name = if curr == "/"{
                        format!("{}{}", curr, spl[1])
                    }else{
                        format!("{}/{}", curr, spl[1])
                    };
                    children.push(name.clone());
                    fs.insert(name, new_child);
                }else{
                    panic!("file not found");
                }

            },
            num => {
                let f = fs.get_mut(&curr).unwrap();

                if let FileSystemObject::Dir(children, _) = f {
                    let new_child = FileSystemObject::File(num.parse().unwrap());
                    let name = if curr == "/"{
                        format!("{}{}", curr, spl[1])
                    }else{
                        format!("{}/{}", curr, spl[1])
                    };
                    children.push(name.clone());
                    fs.insert(name, new_child);
                }else{
                    panic!("file not found");
                }
            }
        }
    }

    let mut x = 0;
    let mut tot = 0;
    let mut min = 1000000000;
    calculate_folder("/".to_string(), fs.clone() , &mut x);
    let unused = 70000000 - x;
    let mut x = 0;
    
    for item in fs.clone() {
        
        match item.1{
            FileSystemObject::Dir(_, _) => {
                let prev_x = x;
                calculate_folder(item.0, fs.clone() , &mut x);
                let dirsize = x - prev_x;
                if dirsize + unused >= 30000000 && dirsize < min {
                    min = dirsize
                }
            },
            FileSystemObject::File(_) => (),
        }
    }
    dbg!(min);
}

pub fn run_day_7(file: &str){
    part_1(file);
    part_2(file);
}
