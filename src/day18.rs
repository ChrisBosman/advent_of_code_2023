use std::{fs::File, io::{self, BufRead}, path::Path, vec};
use std::io::Write;
use itertools::Itertools;

pub(crate) 
fn day18(){
    println!("This is day 18");
    let data = process_data();
    let (x_size, y_size, x0, y0) = find_max_size(&data);
    let edges = walk_through(&data,(y_size+2,x_size+2),(y0+1,x0+1));  // Add a one pixel padding
    let filled = fill_edges(edges);
    let count = filled.iter().fold(0,  |acc, vec| vec.iter().fold(acc, |acc, val| if *val {acc+1} else {acc} ));
    println!("Count = {count}");
    
    // Part two
    println!("Start Part 2");
    let new_data = color2data(&data);
    let (x_size, y_size, x0, y0) = find_max_size(&new_data);
    let edges = walk_through(&new_data,(y_size+2,x_size+2),(y0+1,x0+1));  // Add a one pixel padding
    println!("Edges found");
    let filled = fill_edges(edges);
    println!("Filled");
    let count = filled.iter().fold(0,  |acc, vec| vec.iter().fold(acc, |acc, val| if *val {acc+1} else {acc} ));
    println!("Count = {count}");
    // let _= write_to_file("tmp", &filled);
}

fn color2data(data: &[(char, i32, String)]) -> Vec<(char,i32,String)> {
    let mut new_data: Vec<(char,i32,String)> = vec![];
    for (_,_,hex_num) in data{
        let mut dist = 0;
        let mut dir = 'E';
        // Go over every char and unwrap it to a number
        for (i, ele) in hex_num.chars().rev().enumerate() {
            let val = hex2num(ele);
            if i == 0 {
                dir = match val {
                    0 => 'R',
                    1 => 'D',
                    2 => 'L',
                    3 => 'U',
                    _ => 'E',
                }; 
                continue;
            }
            dist += 16_i32.pow((i-1) as u32) * val;
        }
        println!("Hex num: {dist}, dir: {dir}");
        new_data.push((dir,dist,"".to_string()));
    }

    return new_data;
}

fn hex2num(ele: char) -> i32 {
    return match ele{
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' => 10,
        'b' => 11,
        'c' => 12,
        'd' => 13,
        'e' => 14,
        'f' => 15,
        _ => panic!("Cannot convert char to num: {ele}")
    }
}

fn fill_edges(edges: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    // let inverse = edges.iter().map(|vec| vec.iter().map(|val| !val).collect_vec()).collect_vec();
    // Flood fill the inverse
    let filled = flood_fill(edges.clone(),false);
    // Add filled and edges 
    return edges.iter().enumerate().map(|(i, vec)| vec.iter().enumerate().map(|(j,edge)|{
        *edge || !filled[i][j]
    }).collect_vec()).collect_vec();
}

// Matrix to fill, value to overwrite
fn flood_fill(matrix: Vec<Vec<bool>>, value: bool) -> Vec<Vec<bool>> {
    let mut matrix = matrix;
    // Find start point by trying every point on the left hand side
    let mut starting_point: [usize;2] = [0,0];
    for y in 0..matrix.len(){
        if matrix[y][0] == value {
            starting_point = [y,0];
            break;
        }
        if y == matrix.len()-1 {println!("[Flood_fill]  Not starting point found");}
    }
    // Fill
    let mut frontier_points: Vec<[usize;2]> = vec![starting_point];
    while !frontier_points.is_empty(){
        let mut new_points: Vec<[usize;2]> = vec![];
        // Loop over all frontier_points
        let mut iter = frontier_points.iter();
        while let Some(point) = iter.next() {
            // Check around the point
            // If neigbour is false, spread
            if point[0] > 0 { check_point(&mut matrix, point, &mut new_points, (-1,0),value); }
            if point[1] > 0 { check_point(&mut matrix, point, &mut new_points, (0,-1),value); }
            if point[0] < matrix.len()-1 { check_point(&mut matrix, point, &mut new_points, (1,0),value); }
            if point[1] < matrix[0].len()-1 { check_point(&mut matrix, point, &mut new_points, (0,1),value); }
        }
        frontier_points = new_points;
    }

    return matrix;
}

fn check_point(matrix: &mut Vec<Vec<bool>>, point: &[usize; 2], new_points: &mut Vec<[usize; 2]>, offset: (i32,i32), value: bool) {
    if !matrix[(point[0] as i32 + offset.0 )as usize][(point[1] as i32 + offset.1) as usize] {
        matrix[(point[0] as i32 + offset.0 )as usize][(point[1] as i32 + offset.1) as usize] = !value;
        new_points.push([(point[0] as i32 + offset.0) as usize, (point[1] as i32 + offset.1) as usize])
    }
}

fn walk_through(data: &[(char, i32, String)], size: (usize, usize), starting_index: (usize, usize)) -> Vec<Vec<bool>> {
    let mut board: Vec<Vec<bool>> = vec![vec![false;size.1];size.0];
    let mut current = starting_index;
    for i in 0..data.len(){
        match data[i].0 {
            'R' => current = add_steps_horizontal(&mut board, 1, data[i].1, current),
            'U' => current = add_steps_vertical(&mut board, -1, data[i].1, current),
            'L' => current = add_steps_horizontal(&mut board, -1, data[i].1, current),
            'D' => current = add_steps_vertical(&mut board, 1, data[i].1, current),
            _ => continue,
        }
    }
    // let _= write_to_file("tmp", &board);
    return board;
}

fn add_steps_horizontal(board: &mut Vec<Vec<bool>>,step: i32,nr_steps: i32, current: (usize, usize))-> (usize, usize) {
    let mut c = current;
    for _step_nr in 0..nr_steps{
        c.1 = (c.1 as i32 + step) as usize;
        board[c.0][c.1] = true;
    }
    return c;
}
fn add_steps_vertical(board: &mut Vec<Vec<bool>>,step: i32,nr_steps: i32, current: (usize, usize))-> (usize, usize) {
    let mut c = current;
    for _step_nr in 0..nr_steps{
        c.0 = (c.0 as i32 + step) as usize;
        board[c.0][c.1] = true;
    }
    return c;
}



// Find minimum array and starting points
// Returns: (x_size, y_size, x0,y0)
fn find_max_size(d: &Vec<(char, i32, String)>) -> (usize,usize,usize,usize) {
    let mut x_min: i32 = 0;
    let mut x_max: i32 = 0;
    let mut y_min: i32 = 0;
    let mut y_max: i32 = 0;
    let mut current_x: i32 = 0;
    let mut current_y: i32 = 0;

    for i in 0..d.len(){
        match d[i].0 {
            'R' => current_x += d[i].1,
            'U' => current_y -= d[i].1,
            'L' => current_x -= d[i].1,
            'D' => current_y += d[i].1,
            _ => {},
        }
        // Update the max and min values
        if current_x > x_max {x_max = current_x;}
        if current_y > y_max {y_max = current_y;}
        if current_x < x_min {x_min = current_x;}
        if current_y < y_min {y_min = current_y;}
    }
    println!("Limits  x: ({},{})\ty: ({},{})",x_min,x_max,y_min,y_max);
    // To get x_min = y_min = 0, remove x_min from x_min and x_max, same for y
    // Starting point = (-x_min,-y_min) 
    return ((x_max - x_min) as usize + 1,(y_max - y_min) as usize + 1,-x_min as usize,-y_min as usize);
}

// Extract the data into rgb arrays per game
fn process_data() -> Vec<(char, i32, String)> {
    let mut data: Vec<(char,i32,String)> = vec![];
    // Open file
    let file_path = "inputs/day18.txt";
    // Collect data to array
    if let Ok(lines) = read_lines(file_path){
        for line in lines{
            if let Ok(line) = line{
                let parts = line.split_whitespace().collect_vec();
                let dir = parts[0].chars().nth(0).expect(&format!("Could not find the direction in: {}", line));
                let dist = parts[1].parse::<i32>().expect(&format!("Could not parse string to i32: {}", parts[1]));
                let color = parts[2].replace(&['(', ')', '#'][..],"");
                data.push((dir,dist,color));
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

fn print_matrix(data: &Vec<Vec<bool>>) {
    for row in data {
        let row_string: Vec<String> = row.iter().map(|item| if *item {'#'.to_string()} else {'.'.to_string()} ).collect();
        println!("{}", row_string.join(" "));
    }
}

fn write_to_file(filename: &str, data: &Vec<Vec<bool>>) -> std::io::Result<()>{
    let mut file = File::create(filename)?;

    for row in data {
        let row_string: Vec<String> = row.iter().map(|item| if *item {'#'.to_string()} else {'.'.to_string()} ).collect();
        writeln!(file, "{}", row_string.join(" "))?;
    }
    Ok(())
}