use std::{fs::File, io::{self, BufRead}, path::Path};

pub(crate) 
fn day4(){
    println!("This is day 4");
    process_data();
}

// Extract the data into rgb arrays per game
fn process_data() {
    let mut sum: i32 = 0;
    let mut num_cards: [i32;218] = [1;218];  // Start of with one of each card
    let mut card_nr = 0;
    // Open file
    let file_path = "inputs/day4.txt";
    // Collect data to array
    if let Ok(lines) = read_lines(file_path){
        for line in lines{
            if let Ok(line) = line{
                use_data(line, &mut sum, &mut num_cards, &mut card_nr);
            }
        }
    }else{
        println!("Error: Could not read the file!:\n{}",file_path);
    }
    println!("Sum: {}",sum);
    println!("Total number of cards: {}",num_cards.iter().sum::<i32>());
}

fn use_data(line: String, sum: &mut i32, num_cards: &mut [i32;218], card_nr: &mut usize) {
    let parts:Vec<String> = line.split(&[':','|']).map(|f| {f.to_string()}).collect();
    let winning_nums: Vec<String> = parts[1].split_whitespace().map(|f| {f.to_string()}).collect();
    if parts.len() == 3{
        let my_nums  = &parts[2];
        // Find how many overlapping number
        let (val,nr_matches) = check_score(winning_nums, my_nums);
        *sum += val;
        // Add the number of obtained copies to num_cards
        for offset in 1..nr_matches+1{
            if *card_nr+offset == num_cards.len() {break;}
            num_cards[*card_nr+offset] += num_cards[*card_nr];
        }
        *card_nr += 1;
    }else{
        println!("No my_number found in string {}",line);
    }
}

fn check_score(winning_str: Vec<String>, my_str: &String) -> (i32,usize){
    let mut output = 0;
    let mut nr_matches = 0;
    let winning_nums = winning_str.iter().map(|f| f.parse::<i32>().expect(&format!("Could not parse to i32: {}", f)));
    let my_nums: Vec<i32> = my_str.split_whitespace().map(|f| f.parse::<i32>().expect(&format!("Could not parse to i32: {}", f))).collect();
    for ele in winning_nums {
        for i in 0..my_nums.len(){
            if ele == my_nums[i]{
                nr_matches += 1;
                if output == 0{ output = 1;}
                else {output *= 2;}
            }
        }
    }
    return (output,nr_matches);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}