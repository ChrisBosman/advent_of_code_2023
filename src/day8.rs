use std::{fs::File, io::{self, BufRead}, path::Path, vec};

const NR_NODES: usize = 790;

pub(crate) 
fn day8(){
    println!("This is day 8");
    let (instructions,maps,start,end,node2index) = process_data();
    println!("Start Part 1");
    run(&instructions, maps, start, end);
    println!("Start Part 2");
    // run_all(instructions,maps,node2index);
    find_loop(instructions, maps, node2index);

}

fn find_loop(instructions: Vec<usize>, maps: [[usize; 2]; 790], node2index: Vec<String>){
    // Find start location
    let mut current: Vec<usize> = node2index.iter().enumerate().filter(|(_,x)| x.ends_with('A')).map(|(i,_)|i ).collect();  // Start
    const VAL:Vec<u64> = vec![];
    let mut path_sizes: [Vec<u64>; 6] = [VAL;6];
    let mut step_nrs: [u64; 6] = [0;6];
    // let mut nr_steps: Vec<> = 0;
    let mut i_index: usize = 0;
    'outer: loop{
        // Map to next
        for i in 0..current.len(){
            current[i] = maps[current[i]][instructions[i_index]];
            step_nrs[i] += 1;
            // See if any of these are end locations and reset the loop counter
            if node2index[current[i]].ends_with("Z") {
                path_sizes[i].push(step_nrs[i].clone());
                step_nrs[i] = 0;    
                if path_sizes[i].len() > 4 { break 'outer;}
            }
        }
        i_index += 1;
        if i_index == instructions.len() {i_index=0;}
    }
    println!("path_sizes 0: {:?}",path_sizes[0]);
    println!("path_sizes 1: {:?}",path_sizes[1]);
    println!("path_sizes 2: {:?}",path_sizes[2]);
    println!("path_sizes 3: {:?}",path_sizes[3]);
    println!("path_sizes 4: {:?}",path_sizes[4]);
    println!("path_sizes 5: {:?}",path_sizes[5]);
    // Find the prime factors of the path sizes
    const VAL2: Vec<usize> = vec![];
    let mut prime_facs:[Vec<usize>; 6] = [VAL2;6];
    for i in 0..path_sizes.len(){
        prime_facs[i] = prime_factors(path_sizes[i].last().unwrap().clone() as usize);
    }
    // Lowest multiple the loops using the prime factors
    let mut lowest_multiple = 1;
    for i in 0..prime_facs.len(){
        for j in 0..prime_facs[i].len(){
            if lowest_multiple % prime_facs[i][j] != 0 {
                lowest_multiple *= prime_facs[i][j];
            }
        }
    }
    println!("Lowest multiple: {}",lowest_multiple);  // Lowest multiple:
    
}

fn run(instructions: &Vec<usize>, maps: [[usize; 2]; NR_NODES],start:usize,end:usize){
    let mut current = start;
    let mut nr_steps = 0;
    let mut i_index: usize = 0;
    while current != end{
        current = maps[current][instructions[i_index]];        
        nr_steps += 1;
        i_index += 1;
        if i_index == instructions.len() {i_index=0;}
    }
    println!("Number of steps: {nr_steps}");  // Number of steps: 20569
}

// Extract the data into rgb arrays per game
fn process_data() -> (Vec<usize>, [[usize; 2]; NR_NODES],usize,usize,Vec<String>) {
    let mut instructions: Vec<usize> = vec![];  // Left = 0, Right = 1
    let mut maps: [[usize;2];NR_NODES] = [[0,0]; NR_NODES];
    let mut node2index: Vec<String> = vec![];
    // Open file
    let file_path = "inputs/day8.txt";
    // Collect data to array
    if let Ok(lines) = read_lines(file_path){
        let mut has_read_instruction = false;
        for line in lines{
            if let Ok(line) = line{
                // Save the instructions
                if !has_read_instruction {
                    let mut str = line.replace("L", "0");
                    str = str.replace("R", "1");
                    instructions = str.chars().map(|f| f.to_string().parse::<usize>().expect(&format!("Could not parse into usize {:?}",f))).collect();
                    has_read_instruction = true;
                    continue;
                }
                // Save the nodes
                let parts: Vec<String> = line.replace(&['(', ')', ',', '='][..],"").split_whitespace().map(|f| f.to_string()).collect();
                // Check if the elements of "parts" is already known
                let mut indices: [usize;3] = [0,0,0];
                for i in 0..3{
                    indices[i] =  node2index.iter().position(|x| *x == parts[i]).unwrap_or_else(|| {
                        // Add element and return the index
                        node2index.push(parts[i].clone());
                        return node2index.len()-1
                    });
                }
                maps[indices[0]] = [indices[1],indices[2]];
            }
        }
        
    }else{
        println!("Error: Could not read the file!:\n{}",file_path);
    }
    return (instructions,maps,node2index.iter().position(|x| *x == "AAA").unwrap(), node2index.iter().position(|x| *x == "ZZZ").unwrap(),node2index)
}

fn prime_factors(n: usize) -> Vec<usize> {
    let mut n: usize = n;
    let mut factors = Vec::new();
    let mut prime = 2;
    while n >= prime * prime {
        if n % prime == 0 {
            factors.push(prime);
            n /= prime;
        } else {
            // calculate next prime
            prime = next_prime(prime);
        }
    }
    factors.push(n);
    factors
}

fn next_prime(old_prime:usize) -> usize{
    let mut p = old_prime;
    if old_prime == 2 {return 3;}
    loop{
        p += 2;
        if is_prime(p) {return p;}
    }
}

fn is_prime(p:usize) -> bool{
    if p == 2 {return true;}
    if p % 2 == 0 {return false};
    let mut i = 3;
    while i*i <= p {
        if p % i == 0 {return false;}
        i += 2;
    }
    return true;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


