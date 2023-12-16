use std::{fs::File, io::{self, BufRead}, path::Path, vec};
use std::io::Write;
use itertools::Itertools;

pub(crate) 
fn day16(){
    println!("This is day 16");
    let data = process_data();
    let visited_places = solve_it(&data,0,0,0);
    let nr_energized_tiles = visited_places.iter().fold(0, |acc,v|
         v.iter().fold(acc, |acc2, dir_arr|
            acc2 + if dir_arr.iter().any(|dir| *dir) {1} else {0}));
    println!("Nr of energized tiles: {}",nr_energized_tiles);
    let _ = write_to_file("tmp",&visited_places.iter().map(|v|
         v.iter().map(|f| if f.iter().any(|f| *f ) {'#'} else {'.'}).collect_vec()).collect_vec());
    // Part 2 Change from where the beam enters
    // left & right side 
    let mut max_energized_tiles = 0;
    let x_l = 0;
    let x_r = data[0].len() - 1;
    for y in 0..data.len(){
        let visited_places_l = solve_it(&data,x_l,y,0);
        let visited_places_r = solve_it(&data,x_r,y,2);
        let nr_energized_tiles_l = visited_places_l.iter().fold(0, |acc,v|
            v.iter().fold(acc, |acc2, dir_arr|
               acc2 + if dir_arr.iter().any(|dir| *dir) {1} else {0}));
        let nr_energized_tiles_r = visited_places_r.iter().fold(0, |acc,v|
            v.iter().fold(acc, |acc2, dir_arr|
                   acc2 + if dir_arr.iter().any(|dir| *dir) {1} else {0}));
        if nr_energized_tiles_l > max_energized_tiles {
            max_energized_tiles = nr_energized_tiles_l;
        }
        if nr_energized_tiles_r > max_energized_tiles {
            max_energized_tiles = nr_energized_tiles_r;
        }
    }
    // Top and bottom sides
    let y_t = 0;
    let y_b = data.len() - 1;
    for x in 0..data[0].len(){
        let visited_places_t = solve_it(&data,x,y_t,3);
        let visited_places_b = solve_it(&data,x,y_b,1);
        let nr_energized_tiles_t = visited_places_t.iter().fold(0, |acc,v|
            v.iter().fold(acc, |acc2, dir_arr|
               acc2 + if dir_arr.iter().any(|dir| *dir) {1} else {0}));
        let nr_energized_tiles_b = visited_places_b.iter().fold(0, |acc,v|
            v.iter().fold(acc, |acc2, dir_arr|
                   acc2 + if dir_arr.iter().any(|dir| *dir) {1} else {0}));
        if nr_energized_tiles_t > max_energized_tiles {
            max_energized_tiles = nr_energized_tiles_t;
        }
        if nr_energized_tiles_b > max_energized_tiles {
            max_energized_tiles = nr_energized_tiles_b;
        }
    }
    
    println!("Max nr of energized tiles: {}",max_energized_tiles);	

}

fn solve_it(data: &Vec<Vec<char>>,x: usize, y: usize, dir: i8) -> Vec<Vec<[bool; 4]>> {
    // Matrix to store if it has visited the cell and from which direction.  0 = was traveling to the right, 1=up, 2=left, 3=down
    let mut visited_places: Vec<Vec<[bool;4]>> = vec![vec![[false;4]; data[0].len()]; data.len()];
    visited_places[y][x][dir as usize] = true;
    let mut light_beams: Vec<(usize,usize,i8)> = vec![(x,y,dir)];  // The current light beams, (x,y,direction)
    while !light_beams.is_empty() {
        let mut new_beams: Vec<(usize,usize,i8)> = vec![];  // The new light beams
        let mut indices_to_remove: Vec<usize> = vec![];
        let mut index: usize = 0;
        for (x,y,dir) in light_beams.iter_mut(){
            // Redirect if necessary
            match data[*y][*x] {
                '\\' => {
                    if *dir == 0 || *dir == 2{  // If horizontal go right
                        *dir = if *dir == 0 {3} else {*dir - 1};
                    }else if *dir == 1 || *dir == 3{  // If moving vertical go left
                        *dir = if *dir == 3 {0} else {*dir + 1};
                    }
                },
                '/' => {
                    if *dir == 0 || *dir == 2{  // If horizontal go left
                        *dir += 1;
                    }else if *dir == 1 || *dir == 3{  // If vertical go right
                        *dir -= 1;
                    }
                },
                '-' => {
                    if *dir == 1 || *dir == 3{  // If vertical, split
                        // Add another light beam
                        let new_dir = if *dir == 3 {0} else {*dir + 1};
                        if let Some(new_beam) = move_beam(x, y, &new_dir,&data[0].len(),&data.len()) {
                            visited_places[new_beam.1][new_beam.0][new_beam.2 as usize] = true;
                            new_beams.push(new_beam);
                        };
                                                
                        // Redirect original beam
                        *dir = *dir - 1;
                    }
                },
                '|' => {
                    if *dir == 0 || *dir == 2{  // If horizontal split
                        // Add another light beam
                        let new_dir =  if *dir == 0 {3} else {*dir - 1};
                        if let Some(new_beam) = move_beam(x, y, &new_dir,&data[0].len(),&data.len()) {
                            visited_places[new_beam.1][new_beam.0][new_beam.2 as usize] = true;
                            new_beams.push(new_beam);
                        }
                                                
                        // Redirect original beam
                        *dir = *dir + 1;
                    }
                },
                _ => {},
            }
            // Move
            if let Some(next_pos) = move_beam(x, y, dir,&data[0].len(),&data.len()) {
                // print!("Moved ({},{}) -> ",*x,*y);
                *x = next_pos.0;
                *y = next_pos.1;
                *dir = next_pos.2;
                // println!("({},{})",*x,*y);
            } else {
                indices_to_remove.push(index);
                index += 1;
                continue;
            }

            // Check if it is in a already visited loop
            if visited_places[*y][*x][*dir as usize] {
                indices_to_remove.push(index);
                index += 1;
                continue;
            }
            // Store this cell as visited
            visited_places[*y][*x][*dir as usize] = true;
            index += 1;
        }
        // Remove light beams
        for index in indices_to_remove.iter().rev() {
            light_beams.remove(*index);
        }

        // Add new light beams
        for new_beam in new_beams{
            light_beams.push(new_beam);
        }
    }
    return visited_places;
}

fn move_beam(x: &usize, y: &usize, dir: &i8, x_max: &usize, y_max: &usize) -> Option<(usize, usize, i8)>{
    let mut x = *x;
    let mut y = *y;
    // Check if out of bounds
    if (x == 0 && *dir == 2) || (y == 0 && *dir == 1) {return None}
    match dir {
        0 => x += 1,
        1 => y -= 1,
        2 => x -= 1,
        3 => y += 1,
        _ => panic!("Movement went wrong, invalid direction was inputted"),
    }
    // Check if out of bounds
    if x >= *x_max || y >= *y_max {return None}
    return Some((x,y,*dir));
}

// Extract the data into rgb arrays per game
fn process_data() -> Vec<Vec<char>> {
    let mut data: Vec<Vec<char>> = vec![];
    // Open file
    let file_path = "inputs/day16.txt";
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