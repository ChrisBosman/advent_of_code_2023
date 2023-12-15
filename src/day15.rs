use std::{fs::File, io::{self, BufRead}, path::Path};

pub(crate) 
fn day15(){
    println!("This is day 15");
    process_data();
}

// Extract the data into rgb arrays per game
fn process_data() {
    const VEC_INIT: Vec<(String, i32)> = vec![];
    let mut boxes: [Vec<(String, i32)>; 256] = [VEC_INIT; 256];
    // Open file
    let file_path = "inputs/day15.txt";
    // Collect data to array
    if let Ok(lines) = read_lines(file_path){
        for line in lines{
            if let Ok(line) = line{
                let sum = line.split(",").map(hash).sum::<i32>();
                println!("Sum: {}",sum);
                // Part 2
                let parts = line.split(",").map(|s| s.to_string()).collect::<Vec<String>>();
                for ele in parts {
                    // Check if it contains a = or -
                    if ele.contains("="){
                        let parts2 = ele.split("=").map(|s| s.to_string()).collect::<Vec<String>>();
                        let label = &parts2[0];
                        let value = parts2[1].parse::<i32>().unwrap();
                        let index = hash(&parts2[0]) as usize;
                        // If there is a lens with the same label, replace it
                        if let Some(position) = boxes[index].iter().position(|(v,_)| *v == *label) {
                            boxes[index][position].1 = value;
                        } else {
                            boxes[index].push((label.to_string(),value));
                        }
                    }else if ele.contains("-"){
                        // Remove the lense in the box with the same label
                        let parts2 = ele.split("-").map(|s| s.to_string()).collect::<Vec<String>>();
                        let label = &parts2[0];
                        let index = hash(parts2[0].as_str()) as usize;
                        if let Some(position) = boxes[index].iter().position(|(v,_)| *v == *label) {
                            boxes[index].remove(position);
                        }
                    }
                }
                println!("Boxes {:?}",boxes.get(0..4));
                let focussing_power = boxes.clone().iter().enumerate().map(focus_power_box).sum::<i32>();
                println!("Focussing Power sum: {}", focussing_power); // 7731880 too high
            }
        }
    }else{
        println!("Error: Could not read the file!:\n{}",file_path);
    }
}

fn focus_power_box(input: (usize,&Vec<(String, i32)>))-> i32 {
    let i = input.0;
    let b = input.1;
    if b.is_empty() {return 0}
    return b.iter().enumerate().map(|(slot_num,(_,focal))|  (i as i32 + 1)*(slot_num as i32+1)* *focal).sum::<i32>();
}

fn hash(v: &str)-> i32 {
    let mut current_value = 0;
    for &value in v.as_bytes(){
        current_value += value as i32;
        current_value *= 17;
        current_value %= 256;
    }
    return current_value;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}