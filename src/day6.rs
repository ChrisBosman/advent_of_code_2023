use std::{fs::File, io::{self, BufRead}, path::Path, vec};

pub(crate) 
fn day6(){
    println!("This is day 6");
    let (times,distances) = process_data();
    println!("Times: {:?}",times);
    println!("Distances: {:?}",distances);
    println!("Part 1:\nScore: {}",part1(&times,&distances));
    println!("Part 2:\nScore: {}",part2(times, distances));
}

fn part1(times: &Vec<u32>,max_distances: &Vec<u32>)-> u32 {
    let mut score = 1; 
    // For every race
    for i in 0..times.len(){
        let mut distance: Vec<u32> = vec![];
        // Go over every possible solution  (t_p is how long to press the button, d = v*t_f, where v=t_p,  t_f = t-t_p)
        for t_p in 1..times[i]{
            distance.push(t_p*(times[i]-t_p));
        }
        score *= number_of_ways_to_win(distance,max_distances[i]);
    }
    if score == 1 { return 0}
    return score;
}

fn part2(times: Vec<u32>,max_distances: Vec<u32>)-> usize {
    // Merge the time and distances
    let time = times.iter().map(|f| f.to_string()).collect::<String>().parse::<u64>().expect("Cannot merge times to a single time");
    let max_distance = max_distances.iter().map(|f| f.to_string()).collect::<String>().parse::<u64>().expect("Cannot merge distances to a single distance");

    let mut distance: Vec<u64> = vec![];
    // Go over every possible solution  (t_p is how long to press the button, d = v*t_f, where v=t_p,  t_f = t-t_p)
    for t_p in 1..time{
        let d = t_p*(time-t_p);
        if d > max_distance{
            distance.push(d);
        }else if !distance.is_empty() {
            break;
        }
    }
    let score = distance.len();
    return score;
}

// Return the score of each race
fn number_of_ways_to_win(distances: Vec<u32>,max_distance: u32)-> u32 {
    let mut ways: u32 = 0;
    for i in 0..distances.len(){
        if distances[i] > max_distance{
            ways += 1;    
        }
    }
    return ways;
}

// Extract the data into rgb arrays per game
fn process_data() -> (Vec<u32>, Vec<u32>) {
    let mut times: Vec<u32> = vec![];
    let mut distances: Vec<u32> = vec![];
    // Open file
    let file_path = "inputs/day6.txt";
    // Collect data to array
    if let Ok(lines) = read_lines(file_path){
        for line in lines{
            if let Ok(line) = line{
                let data = line.split(":").nth(1).expect(&format!("Could not split string on \":\": {}", line));
                if times.is_empty(){
                    times = data.split_whitespace().map(|f| f.parse::<u32>().expect(&format!("Could not map data to u32: {}", f))).collect();
                }else{
                    distances = data.split_whitespace().map(|f| f.parse::<u32>().expect(&format!("Could not map data to u32: {}", f))).collect();
                }
            }
        }
    }else{
        println!("Error: Could not read the file!:\n{}",file_path);
    }
    return (times,distances);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}