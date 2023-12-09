use std::{fs::File, io::{self, BufRead}, path::Path, vec};

use itertools::{Itertools, any};

pub(crate) 
fn day9(){
    println!("This is day 9");
    process_data();
}

// Extract the data into rgb arrays per game
fn process_data() {
    let mut sum = 0;
    let mut sum_part2 = 0;
    // Open file
    let file_path = "inputs/day9.txt";
    // Collect data to array
    if let Ok(lines) = read_lines(file_path){
        for line in lines{
            if let Ok(line) = line{
                let data: Vec<i32> = line.split_whitespace().map(|f| f.parse().expect(&format!("Could not parse into i32: {:?}",f))).collect_vec();
                let tree = calculate_diff_tree(data);
                sum += predict(&tree);
                sum_part2 += predict_backwards(&tree);
            }
        }
    }else{
        println!("Error: Could not read the file!:\n{}",file_path);
    }
    println!("Part 1: Sum: {sum}");
    println!("Part 2: Sum: {sum_part2}");
}

fn predict(tree: &Vec<Vec<i32>>)-> i32 {
    let mut prev_ele = 0;
    for i in (0..tree.len()-1).rev(){
        if let Some(last) = tree[i].last(){
            prev_ele = last + prev_ele;
        } else {println!("[predict] Could not find last element in tree[i]: {:?}",tree[i]);}
    }
    return prev_ele
}

fn predict_backwards(tree: &Vec<Vec<i32>>)-> i32 {
    let mut prev_ele = 0;
    for i in (0..tree.len()-1).rev(){
        if let Some(first) = tree[i].first(){
            prev_ele = first - prev_ele;
        } else {println!("[predict_backwards] Could not find first element in tree[i]: {:?}",tree[i]);}
    }
    return prev_ele
}

fn calculate_diff_tree(data: Vec<i32>) -> Vec<Vec<i32>>{
    let mut tree: Vec<Vec<i32>> = vec![];
    tree.push(data);
    // Keep taking the difference
    loop{
        if let Some(last) = tree.last() {
            let new_val = compute_diff(last);
            tree.push(new_val.clone());
            if !any(new_val, |v| v != 0) {break;}
        }else{  
            println!("[calculate_diff_tree] Could not find last element in tree");
            break;
        }
    }
    // println!("{:?}",tree);
    return tree
}

fn compute_diff(vec: &[i32]) -> Vec<i32> {
    return vec.windows(2).map(|v| v[1]-v[0]).collect_vec();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}