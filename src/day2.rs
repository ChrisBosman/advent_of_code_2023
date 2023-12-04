use std::{fs::File, io::{self, BufRead}, path::Path, vec};

pub(crate) 
fn day2(){
    println!("This is day 2");
    let (data, game_numbers) = process_data();

    let mut sum = 0;
    let mut total_power = 0;
    // Check which games are possible with //*[12 red, 13 green, and 14 blue]
    for i in 0..game_numbers.len(){
        if data[i][0] <= 12 && data[i][1] <= 13 && data[i][2] <= 14 {
            sum += game_numbers[i];
        }
        total_power += data[i][0]*data[i][1]*data[i][2]
    }
    println!("Sum = {}",sum);
    println!("Total Power = {}",total_power);
    // Tmp, print out the text
    // for i in 0..game_numbers.len(){
    //     print!("{}: ",game_numbers[i]);
    //     println!("r: {}, g: {}, b: {}",data[i][0],data[i][1],data[i][2]);
    // }
}

// Extract the data into rgb arrays per game
fn process_data()-> (Vec<[i32; 3]>, Vec<i32>) {
    let mut game_numbers: Vec<i32> = vec![];
    let mut data: Vec<[i32; 3]> = vec![];
    // Open file
    let file_path = "inputs/day2.txt";
    // Collect data to array
    if let Ok(lines) = read_lines(file_path){
        for line in lines{
            if let Ok(line) = line{
                let mut parts: Vec<String> = line.split(&[':',';']).map(|f| {f.to_string()}).collect();  // Split of the Game i:
                game_numbers.push(parts[0].split_off(5).parse::<i32>().expect("[Process data] Game_nr could not unwrap"));
                
                let mut current_data: [i32; 3] = [0, 0, 0];
                // Go over each round in a game
                for i in 1..parts.len(){
                    // Go over all the colors
                    for ele in parts[i].split(',') {
                        let ele: Vec<&str> = ele.split_whitespace().collect();
                        match ele[1] {
                            "red" => {
                                let v = ele[0].parse::<i32>().expect(&format!("[Process_data] Could not unwrap red color to number: {}", ele[0]));
                                if current_data[0] < v {current_data[0] = v}
                            },
                            "green" => {
                                let v = ele[0].parse::<i32>().expect(&format!("[Process_data] Could not unwrap red color to number: {}", ele[0]));
                                if current_data[1] < v {current_data[1] = v}
                            },
                            "blue" => {
                                let v = ele[0].parse::<i32>().expect(&format!("[Process_data] Could not unwrap red color to number: {}", ele[0]));
                                if current_data[2] < v {current_data[2] = v}
                            },
                            _ => {println!("No color found in substring")},
                        }
                    };
                }
                data.push(current_data);
            }
        }
    }else{
        println!("Error: Could not read the file!:\n{}",file_path);
    }
    return (data,game_numbers)
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}