use std::{fs::File, io::{self, BufRead}, path::Path, vec};

pub(crate) 
fn day3(){
    println!("This is day 3");
    process_data();
}

// Extract the data into rgb arrays per game
fn process_data() {
    let mut sum: i32 = 0;
    let mut gear_ratio_sum: i32 = 0;
    // Open file
    let file_path = "inputs/day3.txt";
    // Collect data to array
    if let Ok(lines) = read_lines(file_path){
        let mut prev_prev_chars:Vec<char> = vec![];  // Two lines ago
        let mut prev_chars: Vec<char> = vec![];
        let mut prev_indices: Vec<usize> = vec![];
        for line in lines{
            if let Ok(line) = line{
                let chars: Vec<char> = line.chars().collect();
                // Check if there are numbers(this line) next to the symbols of the previous line
                for ele in &prev_indices {
                    sum += find_number(&chars,ele);
                }

                // Get the indices of each 'symbol'   Position only returns one index at a time
                let mut iter = chars.iter();
                let mut indices: Vec<usize> = vec![];
                if let Some(index) = iter.position(|&e| e != '.' && (e < '0' || e > '9')){
                    indices.push(index);
                    while let Some(index) = iter.position(|&e| e != '.' && (e < '0' || e > '9')){
                        indices.push(index+indices.last().unwrap()+1)
                    }
                }

                // Check this line
                for index in &indices {
                    sum += find_number(&chars, index);
                }

                // Check if there are numbers of prev line next to the symbols of this line
                if prev_chars.len() > 0{
                    for index in &indices {
                        sum += find_number(&prev_chars, index);
                    }
                }

                //* Finding the gears (from the previous line)
                if prev_chars.len() > 0 {
                    let mut iter = prev_chars.iter();
                    let mut possible_gears: Vec<usize> = vec![];
                    if let Some(index) = iter.position(|&e| e == '*'){
                        possible_gears.push(index);
                        while let Some(index) = iter.position(|&e|e == '*'){
                            possible_gears.push(index+possible_gears.last().unwrap()+1)
                        }
                    }

                    // Check which possible gears only have two numbers around them, those are the final gears
                    for index in &possible_gears {
                        let mut amount = 0;
                        let mut gear_ratio = 1;
                        // Top
                        if prev_prev_chars.len() > 0{
                            let (a,num1,num2) = find_number_extended(&prev_prev_chars, index);
                            amount += a;
                            if a >= 1 {gear_ratio *= num1;}
                            if a == 2 {gear_ratio *= num2;}
                        }
                        // Middle
                        let (a,num1,num2) = find_number_extended(&prev_chars, index);
                        amount += a;
                        if a >= 1 {gear_ratio *= num1;}
                        if a == 2 {gear_ratio *= num2;}
                        
                        // Bottom
                        let (a,num1,num2) = find_number_extended(&chars, index);
                        amount += a;
                        if a >= 1 {gear_ratio *= num1;}
                        if a == 2 {gear_ratio *= num2;}

                        if amount != 2 {continue;}
                        gear_ratio_sum += gear_ratio;
                    }
                }
                // println!("{:?}",indices);
                // println!("{:?}",indices.iter().map( |i| chars[*i] ).collect::<Vec<_>>());
                prev_prev_chars = prev_chars;
                prev_chars = chars;
                prev_indices = indices;
            }
        }
        println!("Sum: {}",sum);
        println!("Gear ratio sum: {}",gear_ratio_sum);
    }else{
        println!("Error: Could not read the file!:\n{}",file_path);
    }
}

// Returns the amount of number found and the value of the numbers: (amount, num1, num2)
fn find_number_extended(chars: &Vec<char>, &index: &usize) -> (usize,i32,i32) {
    // Check if there is a number on the three spots around the index,
    if chars[index] >= '0' && chars[index] <= '9' {
        // There is a number above or below the index
        return (1,search_sideways(chars,index),0);
    }

    // If there is a number to the left, than there can still be a number to the right as well:  [23 . 232]
    let mut output = vec![];
    if index > 0 && chars[index-1] >= '0' && chars[index-1] <= '9' {
        // There is a number left of the index
        output.push(search_sideways(chars,index-1));
    }
    if index+1 < chars.len() && chars[index+1] >= '0' && chars[index+1] <= '9' {
        // There is a number to the right of the index
        output.push(search_sideways(chars,index+1));
    }
    match output.len() {
        1 => return (output.len(),output[0],0),
        2 => return (output.len(),output[0],output[1]),
        _ => return (0,0,0),
    }
}

fn find_number(chars: &Vec<char>, &index: &usize) -> i32 {
    // Check if there is a number on the three spots around the index, v  v v
    //                                                                [.  i .]
    if chars[index] >= '0' && chars[index] <= '9' {
        // There is a number above or below the index
        // Search sideways to find the whole number
        return search_sideways(chars,index);
    }

    // If there is a number to the left, than there can still be a number to the right as well:  [23 . 232]
    let mut output = 0;
    if index > 0 && chars[index-1] >= '0' && chars[index-1] <= '9' {
        // There is a number left of the index
        output += search_sideways(chars,index-1);
    }
    if index+1 < chars.len() && chars[index+1] >= '0' && chars[index+1] <= '9' {
        // There is a number to the right of the index
        output += search_sideways(chars,index+1);
    }
    return output
}

// There is a number on index, search sidewways to find the full length
fn search_sideways(chars: &Vec<char>, index: usize) -> i32 {
    // Search left
    let mut nums_c: Vec<char> = vec![];
    let mut offset: usize = 0;
    while offset <= index && chars[index-offset] >= '0' && chars[index-offset] <= '9'{
        nums_c.push(chars[index-offset]);
        offset += 1;
    }
    nums_c.reverse();
    // Search right
    let mut offset: usize = 1;
    while offset+index < chars.len() && chars[index+offset] >= '0' && chars[index+offset] <= '9'{
        nums_c.push(chars[index+offset]);
        offset += 1;
    }
    let output = nums_c.iter().collect::<String>().parse::<i32>().expect(&format!("[Search_sideways] Could not parse string to i32: {}", nums_c.iter().collect::<String>())); 
    // println!("[{}] {}",index, output);
    return output;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}