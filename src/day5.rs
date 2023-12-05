use std::{fs::File, io::{self, BufRead}, path::Path, vec};

const NR_MAPS:usize = 7;  // The number of maps

pub(crate) 
fn day5(){
    println!("This is day 5");
    let (seeds, maps) = process_data();
    let mut location: Vec<u64> = vec![];
    for &seed in &seeds {
        location.push(walk_though_all_maps(seed,&maps));
    }
    println!("Lowest value: {}",location.iter().min().expect("Could not take the minimum value"));

    // Part 2
    println!("Part 2");
    // Walk to the maps in the other direction, from location to seed
    'outer: for location in 0..4520479{
        let seed = walk_though_all_maps_inv(location, &maps);
        // check if it is a seed
        for i in 0..seeds.len()/2{
            if seed >= seeds[i*2] && seed < seeds[i*2] + seeds[i*2+1]{
                println!("Lowest value: {}\nSeed: {}",location,seed);  // Lowest value: 2520479
                break 'outer;
            }
        }
    }

}

// Extract the data into rgb arrays per game
fn process_data() -> (Vec<u64>, [Vec<[u64; 3]>; NR_MAPS]) {
    // The arrays to store the data
    let mut seeds: Vec<u64> = vec![];
    const VAL: Vec<[u64;3]> = vec![];
    let mut maps: [Vec<[u64;3]>;NR_MAPS] = [VAL;NR_MAPS];  // [Category, map, map elements]
    // Open file
    let file_path = "inputs/day5.txt";
    // Collect data to array
    if let Ok(lines) = read_lines(file_path){
        // Current location in the file
        let mut current_var = "Seeds".to_string();
        let mut category_index: usize = 0;  // Current category index
        for line in lines{
            if let Ok(line) = line{
                if line == ""{ // Switch between blocks
                    current_var = "".to_string();
                    continue;
                }
                match current_var.as_str() {
                    "Seeds" => {
                        let seed_str = line.split("seeds: ").nth(1).expect("Could not split the Seeds line");
                        seeds = seed_str.split_whitespace().collect::<Vec<&str>>().iter().map(|f| f.parse::<u64>().expect(&format!("Could not parse the seeds string: {}", f))).collect();
                    },
                    "" => { // Change of categories
                        current_var = line;
                        if current_var != "seed-to-soil map:"{
                            category_index += 1
                        }
                    },
                    _ => { // Add the maps
                        let map_elements: Vec<u64> = line.split_whitespace().collect::<Vec<&str>>().iter().map(|f| f.parse::<u64>().expect(&format!("Could not parse the map string: {}", f))).collect();
                        assert!(map_elements.len() == 3, "Elements of the map are not equal to 3, line: {}",line);
                        let mut arr: [u64; 3] = [0, 0, 0];
                        for i in 0..3 {
                            arr[i] = map_elements[i];
                        }
                        maps[category_index].push(arr);
                    }
                }
            }
        }
    }else{
        println!("Error: Could not read the file!:\n{}",file_path);
    }
    return (seeds,maps);
}

fn walk_though_all_maps(seed: u64, maps: &[Vec<[u64; 3]>; NR_MAPS])-> u64 {
    let mut current_value = seed;
    for i in 0..NR_MAPS{
        current_value = map_seed(current_value,&maps[i]);
    }
    return current_value;
}

fn map_seed(current_value: u64, map: &Vec<[u64; 3]>) -> u64 {
    // A map consist out of [Destination_start   Source_start  Source_range]
    // Walk through all the sub-maps
    for i in 0..map.len(){
        // Check if it is in the range
        if current_value >= map[i][1] && current_value < map[i][1] + map[i][2]{
            // Map it using this sub-map
            let offset = current_value - map[i][1];
            return map[i][0]+offset;
        }
    }
    // If it is not mapped, then it maps to the same value
    return current_value;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn walk_though_all_maps_inv(location: u64, maps: &[Vec<[u64; 3]>; NR_MAPS])-> u64 {
    let mut current_value = location;
    for i in (0..NR_MAPS).rev(){
        current_value = map_seed_inv(current_value,&maps[i]);
    }
    return current_value;
}

fn map_seed_inv(current_value: u64, map: &Vec<[u64; 3]>) -> u64 {
    // A map consist out of [Destination_start   Source_start  Range]
    // Walk through all the sub-maps
    for i in 0..map.len(){
        // Check if it is in the range
        if current_value >= map[i][0] && current_value < map[i][0] + map[i][2]{
            // Map it using this sub-map
            let offset = current_value - map[i][0];
            return map[i][1]+offset;
        }
    }
    // If it is not mapped, then it maps to the same value
    return current_value;
}