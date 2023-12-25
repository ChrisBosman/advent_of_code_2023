use std::{fs::File, io::{self, BufRead}, path::Path, vec};

pub(crate) 
fn day23(){
    println!("This is day 23");
    let graph = process_data();
    
}

// Extract the data into rgb arrays per game
fn process_data() -> Vec<Node> {
    // Open file
    let file_path = "inputs/day23.txt";
    let mut data: Vec<Vec<char>> = vec![];
    // Collect data to array
    if let Ok(lines) = read_lines(file_path){
        for line in lines{
            if let Ok(line) = line{
                data.push(line.chars().collect::<Vec<char>>());
            }
        }
    }else{
        println!("Error: Could not read the file!:\n{}",file_path);
    }
    return create_graph(data);
}


// # are trees and . is a path and there are some one way directions edges
fn create_graph(data: Vec<Vec<char>>) -> Vec<Node> {
    let mut graph: Vec<Node> = vec![];
    // Start by adding all the nodes to an array
    for i in 0..data.len(){
        for j in 0..data[i].len(){
            if data[i][j] == '#'{ continue; }
            graph.push(Node{row: i, col: j, neighbors: vec![]});
        }
    }

    // Connect the neighbors
    for i in 0..graph.len(){
        // Check above
        if graph[i].row > 0 && data[graph[i].row - 1][graph[i].col] != '#' && data[graph[i].row - 1][graph[i].col] != 'v' {
            if let Some(node_index) = find_cell_backwards(&graph,i,graph[i].row - 1, graph[i].col) {
                graph[i].neighbors.push(node_index);
            } else {panic!("[Up] Cannot find neighbors, [{},{}]", graph[i].row, graph[i].col)}
        }
        // Check down
        if graph[i].row > data.len() && data[graph[i].row + 1][graph[i].col] != '#' && data[graph[i].row + 1][graph[i].col] != '^' {
            if let Some(node_index) = find_cell_forwards(&graph,i,graph[i].row + 1, graph[i].col) {
                graph[i].neighbors.push(node_index);
            } else {panic!("[Down] Cannot find neighbors, [{},{}]",graph[i].row, graph[i].col)}
        }
        // Check Left
        if graph[i].col > 0 && data[graph[i].row][graph[i].col - 1] != '#' && data[graph[i].row][graph[i].col - 1] != '>' {
            graph[i].neighbors.push(i-1);
        }
        // Check Right
        if graph[i].col < data[0].len() && data[graph[i].row][graph[i].col + 1] != '#' && data[graph[i].row][graph[i].col + 1] != '<' {
            graph[i].neighbors.push(i+1);
        }
    }

    return graph
}

fn find_cell_backwards(graph: &[Node], i: usize, row: usize, col: usize) -> Option<usize> {
    for j in (0..i).rev(){
        if graph[j].col == col && graph[j].row == row{
            return Some(j);
        }
    }
    return None
}

fn find_cell_forwards(graph: &[Node], i: usize, row: usize, col: usize) -> Option<usize> {
    for j in i..graph.len() {
        if graph[j].col == col && graph[j].row == row{
            return Some(j);
        }
    }
    return None
}

struct Node{
    row: usize,
    col: usize,
    neighbors: Vec<usize>,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}