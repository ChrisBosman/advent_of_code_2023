use std::{fs::File, io::{self, BufRead}, path::Path};

pub(crate) 
fn day1(){
    println!("This is day 1");
    process_data();
}

fn process_data(){
    let mut sum: i32 = 0;
    // Open file
    let file_path = "inputs/day1.txt";
    // Collect data to array  and make the beginnings of a step map
    if let Ok(lines) = read_lines(file_path){
        for line in lines{
            if let Ok(line) = line{
                let chars: Vec<char> = line.chars().collect();
                let mut nums: String = " ".to_string();
                let mut slice: String = "".to_string();
                // Forward pass
                for ch in chars{
                    if ch >= '0' && ch <= '9'{
                        // Arrived at the first number
                        nums = ch.to_string();
                        slice = "".to_string();
                        break;
                    }
                    // Check if it is a number like zero or nine
                    else if is_contained_in_numbers(ch){
                        slice.push(ch);
                        // If it contains a number
                        let potential_num = contains_number(&slice);
                        if potential_num == 'n' {continue;}
                        // Arrived at first number
                        nums = potential_num.to_string();
                        slice = "".to_string();
                        break;
                    }else {
                        slice = "".to_string();
                    }
                }
                assert!(nums != " ","No number found in forward pass");
                let chars: Vec<char> = line.chars().collect();
                // Backwards pass
                for &ch in chars.iter().rev(){
                    if ch >= '0' && ch <= '9'{
                        // Arrived at the first number
                        nums.push(ch);
                        break;
                    }
                    // Check if it is a number like zero or nine
                    else if is_contained_in_numbers(ch){
                        slice.push(ch);
                        // If it contains a number
                        let potential_num = contains_number_backwards(&slice);
                        if potential_num == 'n' {continue;}
                        // Arrived at first number
                        nums.push(potential_num);
                        break;
                    }else {
                        slice = "".to_string();
                    }
                }
                sum += nums.parse::<i32>().expect("[53] Could not parse string to int");
                println!("sum: {}",sum);
            }
        }
    }else{
        println!("Error: Could not read the file!:\n{}",file_path);
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_contained_in_numbers(ch: char) -> bool{
    match ch {
        'o' => return true,
        'n' => return true,
        'e' => return true,
        't' => return true,
        'w' => return true,
        'h' => return true,
        'r' => return true,
        'f' => return true,
        'u' => return true,
        'i' => return true,
        'v' => return true,
        's' => return true,
        'x' => return true,
        'g' => return true,
        'z' => return true,
        _ => return false
    }
}

// Returns number or 'n' if no number is found
fn contains_number(slice: &String) -> char {
    if slice.len() < 3 {return 'n'}  // Too short
    if slice.contains("one")    {return '1'};
    if slice.contains("two")    {return '2'};
    if slice.contains("three")  {return '3'};
    if slice.contains("four")   {return '4'};
    if slice.contains("five")   {return '5'};
    if slice.contains("six")    {return '6'};
    if slice.contains("seven")  {return '7'};
    if slice.contains("eight")  {return '8'};
    if slice.contains("nine")   {return '9'};
    if slice.contains("zero")   {return '0'};
    return 'n'
}

fn contains_number_backwards(slice: &String) -> char {
    if slice.len() < 3 {return 'n'}  // Too short
    if slice.contains("eno")    {return '1'};
    if slice.contains("owt")    {return '2'};
    if slice.contains("eerht")  {return '3'};
    if slice.contains("ruof")   {return '4'};
    if slice.contains("evif")   {return '5'};
    if slice.contains("xis")    {return '6'};
    if slice.contains("neves")  {return '7'};
    if slice.contains("thgie")  {return '8'};
    if slice.contains("enin")   {return '9'};
    if slice.contains("orez")   {return '0'};
    return 'n'
}