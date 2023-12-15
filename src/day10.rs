use std::{fs::File, io::{self, BufRead}, path::Path, vec};
use std::io::Write;
use itertools::Itertools;

pub(crate) 
fn day10(){
    println!("This is day 10");
    let data = process_data();
    walk_through_loop(data);
}

fn walk_through_loop(data: Vec<Vec<char>>){
    let mut walked_map: Vec<Vec<i32>> = vec![vec![0; data[0].len()]; data.len()];
    let mut current_cells: [[usize;3];2] = [[0;3];2];
    // Find the starting index
    if let Some(row_index) = data.iter().position(|v| v.contains(&'S')){
        if let Some(column_index) = data[row_index].iter().position(|v| *v=='S'){
            current_cells[0][0] = row_index;
            current_cells[0][1] = column_index;
        } else {println!("Cannot find starting column index")}
    } else {println!("Cannot find starting row index")}
    // Find the next two cells
    (current_cells[0],current_cells[1]) = find_start_cells(&data, current_cells[0]);
    println!("next cell: {:?}",current_cells);
    println!("next cell: {}",data[current_cells[0][0]][current_cells[0][1]]);
    println!("other next cell: {}",data[current_cells[1][0]][current_cells[1][1]]);
    let mut step_counter = 1;
    walked_map[current_cells[0][0]][current_cells[0][1]] = step_counter;
    walked_map[current_cells[1][0]][current_cells[1][1]] = step_counter;
    // Loop through the loop
    while current_cells[0] != current_cells[1]{
        let next_cell1 = find_next_cells(&data, current_cells[0]);
        let next_cell2 = find_next_cells(&data, current_cells[1]);
        current_cells = [next_cell1,next_cell2];
        step_counter += 1;
        walked_map[current_cells[0][0]][current_cells[0][1]] = step_counter;
        walked_map[current_cells[1][0]][current_cells[1][1]] = step_counter;
        if next_cell1[0] == next_cell2[0] && next_cell1[1] == next_cell2[1] {break;}
    }
    println!("Steps {step_counter}");

    // Save to file
    _ = write_to_file("tmp",&walked_map);
}

// Find the next cell, assuming there is only one possibility
fn find_next_cells(data: &Vec<Vec<char>>, this_cell: [usize; 3])-> [usize; 3] {
    // See from which direction it arrived
    // The this_cell means the dir that the cell was placed from that direction i.e. 1 means that te this_cell was placed from a cell below
    //   1
    //   _
    //2 |_|  0
    //   3
    // transpose it for ease of understanding
    //   3
    //   _
    //0 |_|  2
    //   1
    let dir = this_cell[2];
    let newdir = match data[this_cell[0]][this_cell[1]] {
        '|' => (dir+2) % 4,
        '-' => (dir+2) % 4,
        'J' => if dir == 0 {3} else {0},
        '7' => if dir == 0 {1} else {0},
        'F' => if dir == 1 {2} else {1},
        'L' => if dir == 2 {3} else {2},
        _ => panic!("Current cell is not [|-J7FL], it is: {}",data[this_cell[0]][this_cell[1]]),
    };
    match newdir{
        0 => return [this_cell[0], this_cell[1]-1, 2],
        1 => return [this_cell[0]+1, this_cell[1], 3],
        2 => return [this_cell[0], this_cell[1]+1, 0],
        3 => return [this_cell[0]-1, this_cell[1], 1],
        _ => panic!("New direction is not 0-3, it is: {}",newdir)
    }
}

// Find the two start cells
fn find_start_cells(data: &Vec<Vec<char>>, indices: [usize;3])-> ([usize; 3],[usize; 3]){
    return (find_neighbor_cell(data, indices, 0),find_neighbor_cell(data, indices, 1));
}

// data, indices [row, col,prev_dir]: current/center indices, nth: returns the nth connecting cell, 0 = first, 1 = second, ...
fn find_neighbor_cell(data: &Vec<Vec<char>>, indices: [usize;3], init_nth: i32)-> [usize; 3] {
    let mut nth = init_nth;
    let possible_connections = [['J','7','-'],['F','7','|'],['F','L','-'],['J','L','|']];  // Right,Up,Left,Down
    // Check right
    if indices[1]+1 < data[0].len() && possible_connections[0].contains(&data[indices[0]][indices[1]+1]) {
        if nth == 0 {return [indices[0],indices[1]+1,0];}
        nth -= 1;
    }
    // Check up
    if indices[0] > 0 && possible_connections[1].contains(&data[indices[0]-1][indices[1]]){
        if nth == 0 {return [indices[0]-1,indices[1],1];}
        nth -= 1;
    }
    // Check left
    if indices[1] > 0 && possible_connections[2].contains(&data[indices[0]][indices[1]-1]){
        if nth == 0 {return [indices[0],indices[1]-1,2];}
        nth -= 1;
    }
    // Check down
    if indices[0]+1 < data.len() && possible_connections[3].contains(&data[indices[0]+1][indices[1]]){
        if nth == 0 {return [indices[0]+1,indices[1],3];}
    }
    println!("Could not find an cell, nth: {init_nth}\tindices{:?}",indices);
    return  [0,0,0];
}

// Extract the data into rgb arrays per game
fn process_data() -> Vec<Vec<char>> {
    let mut data: Vec<Vec<char>> = vec![];
    // Open file
    let file_path = "inputs/day10.txt";
    // Collect data to array
    if let Ok(lines) = read_lines(file_path){
        for line in lines{
            if let Ok(line) = line{
                data.push(line.chars().collect_vec());
            }
        }
    }else{
        println!("Error: Could not read the file!:\n{}",file_path);
    }
    return data;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn write_to_file<T: std::fmt::Display>(filename: &str, data: &Vec<Vec<T>>) -> std::io::Result<()>{
    let mut file = File::create(filename)?;

    for row in data {
        let row_string: Vec<String> = row.iter().map(|item| item.to_string()).collect();
        writeln!(file, "{}", row_string.join(" "))?;
    }
    Ok(())
}