use std::{fs::File, io::{self, BufRead}, path::Path, vec, collections::HashMap, ops::ControlFlow};

struct xmas_range{
    x: [i32;2],
    m: [i32;2],
    a: [i32;2],
    s: [i32;2],
}

// init the xmas range
impl xmas_range{
    fn new() -> xmas_range{
        xmas_range{
            x: [0, 4000],
            m: [0, 4000],
            a: [0, 4000],
            s: [0, 4000],
        }
    }
    fn new_from_vec(vec: &Vec<i32>) -> xmas_range{
        xmas_range{
            x: [vec[0], vec[0]],
            m: [vec[1], vec[1]],
            a: [vec[2], vec[2]],
            s: [vec[3], vec[3]],
        }
    }
    fn new_from_self(&self) -> xmas_range{
        xmas_range{
            x: self.x,
            m: self.m,
            a: self.a,
            s: self.s,
        }
    }
}

pub(crate) 
fn day19(){
    println!("This is day 19");
    let (parts, workflows, workflow_indices) = process_data();
    let mut sum = 0;
    // Print out all the keys of the hash map
    for i in 0..parts.len(){
        if walk_workflow(&parts[i], &workflows){
            // println!("Part {:?} is approved",parts[i]);
            sum += parts[i].iter().sum::<i32>();
        }
    }
    println!("The sum of all approved parts is: {}",sum);
    // Part two, move in reverse over the list, from "A" to "in"
    let (mut current_workflows, mut current_indices) = workflow_indices["A"].clone();
    let mut ranges: Vec<[usize;2]> = vec![[1, 4000]; current_workflows.len()];
    while !current_workflows.is_empty() {
        for i in 0..current_workflows.len(){
            
        }
    }
}

// Walk through the workflow and return if it is approved or rejected
fn walk_workflow(part: &Vec<i32>, workflows: &HashMap<String, (Vec<Box<dyn Fn(&Vec<i32>) -> bool>>, Vec<String>)>) -> bool {
    let mut current_workflow = "in".to_string();
    loop {
        let (conditions, next_workflows) = workflows.get(&current_workflow).expect(&format!("Could not find workflow: {}", current_workflow).as_str());
        for i in 0..conditions.len(){
            if conditions[i](part) {
                // The condition is true, so we can go to the next workflow or accept/reject
                current_workflow = next_workflows[i].clone();
                match current_workflow.as_str() {
                    "R" => { return false},
                    "A" => { return true},
                    _ => {break;},
                }
            }
        }
    }
}

// Extract the data into rgb arrays per game
fn process_data() -> (Vec<Vec<i32>>, HashMap<String,
      (Vec<Box<dyn Fn(&Vec<i32>) -> bool>>, Vec<String>)>,
      HashMap<String,(Vec<String>,Vec<usize>)>) {
    let mut parts: Vec<Vec<i32>> = vec![];
    // Create a hash map to store the workflow name and a tuple with the condition and the next workflow
    let mut work_flows: HashMap<String,(Vec<Box<dyn Fn(&Vec<i32>) -> bool>>,Vec<String>)> = HashMap::new();
    let mut workflow_indices: HashMap<String,(Vec<String>,Vec<usize>)> = HashMap::new();

    // Open file
    let file_path = "inputs/day19.txt";
    // Collect data to array
    if let Ok(lines) = read_lines(file_path){
        let mut saving_work_flows = true;
        for line in lines{
            if let Ok(line) = line{
                if let ControlFlow::Break(_) = part1_data(line, &mut saving_work_flows, &mut work_flows, &mut parts) {
                    continue;
                }
            }
        }
        // Make a hash map that stores the workflow name and index where each workflow is called
        // Init hashmap
        for (key,_) in work_flows.iter(){
            workflow_indices.insert(key.clone(), (vec![],vec![]));
        }
        workflow_indices.insert("R".to_string(), (vec![],vec![]));
        workflow_indices.insert("A".to_string(), (vec![],vec![]));

        for (key,(_,next_workflows)) in work_flows.iter(){
            // For each step in workflow
            for i in 0..next_workflows.len(){ 
                 if let Some(current_val) = workflow_indices.get_mut(&next_workflows[i]) {
                    current_val.0.push(key.clone());
                    current_val.1.push(i);
                 } else {println!("Index not found in hash map, key: \"{}\"",next_workflows[i])}
            }
        }
    }else{
        println!("Error: Could not read the file!:\n{}",file_path);
    }
    return (parts, work_flows, workflow_indices);
}

fn part1_data(line: String, saving_work_flows: &mut bool, work_flows: &mut HashMap<String, (Vec<Box<dyn Fn(&Vec<i32>) -> bool>>, Vec<String>)>, parts: &mut Vec<Vec<i32>>) -> ControlFlow<()> {
    if line == ""{
        *saving_work_flows = false;
        return ControlFlow::Break(());
    }
    if *saving_work_flows{
        // Save the work flows
        let mut slices = line.split("{");
        let key = slices.next().expect("Could not extract key").to_string();
        let steps = slices.next().expect("Could not extract value").split(",").map(|f| f.to_string()).collect::<Vec<String>>();
        let mut conditions: Vec<Box<dyn Fn(&Vec<i32>) -> bool>> = vec![];
        let mut next_workflows: Vec<String> = vec![];
        for step in steps{
            // Extract the condition and the next key. step = "a<2006:qkq"      
            let mut slices = step.split(":");
            let first = slices.next().expect("Could not extract first part");
            if let Some(next_work_flow) = slices.next() {
                let (mut char_iter, rating_type) = get_rating_type(first, &line);
                let logic_operator = char_iter.next().expect("Could not extract logic operator");
                let rating = char_iter.as_str().parse::<i32>().expect("Could not extract rating");
                match logic_operator {
                    '<' => {
                        conditions.push(Box::new(move |x: &Vec<i32>| x[rating_type] < rating));
                    },
                    '>' => {
                        conditions.push(Box::new(move |x: &Vec<i32>| x[rating_type] > rating));
                    },
                    _ => panic!("Unknown logic operator: {}", logic_operator),
                };
                next_workflows.push(next_work_flow.to_string());
            } else {
                // The condition is missing, so the condition is always true
                conditions.push(Box::new(|_| true));
                next_workflows.push(first.strip_suffix("}").unwrap_or(first).to_string());
            }
        }
        // println!("Saved workflow: {} -> {:?}",key, next_workflows);
        work_flows.insert(key,(conditions,next_workflows));
    }else{
        // Save the parts
        let part_ratings = readf::<i32>(vec!["{x=",",m=",",a=",",s=","}"],line);
        parts.push(part_ratings);
    }
    ControlFlow::Continue(())
}

fn get_rating_type<'a>(first: &'a str, line: &'a String) -> (std::str::Chars<'a>, usize) {
    let mut char_iter = first.chars();
    let rating_type = match char_iter.next().expect("Could not extract rating type") {
        'x' => {0},
        'm' => {1},
        'a' => {2},
        's' => {3},
        _ => panic!("Unknown rating type: {}",line),
    };
    (char_iter, rating_type)
}

/// ### Read formated string,
/// Format, the text around the values to extract, i.e. "Game 1, red 4, green 5"
/// -> format = vec!["Game ",", red ",", green "]
fn readf<T: std::str::FromStr + std::fmt::Display>(format: Vec<&str>, input: String)-> Vec<T> {
    let mut output: Vec<T> = vec![];
    let mut next_string = input;
    for ele in format {
        let tmp = next_string.splitn(2,&ele).map(|f| f.to_string()).collect::<Vec<String>>();
        next_string = tmp[1].to_string();
        if let Ok(val) = tmp[0].to_string().parse::<T>(){
            output.push(val);
        }
    }
    if let Ok(val) = next_string.parse::<T>(){
        output.push(val);
    }
    return output
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}