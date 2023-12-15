use std::{fs::File, io::{self, BufRead}, path::Path, vec, collections::VecDeque};

pub(crate) 
fn day12(){
    let (data, lengths) = process_data();
    let (simp_data,simp_length) = simplify(&data, &lengths, false);
    calc_options(simp_data,simp_length);

}

// Calculate all the possible options
fn calc_options(simp_data: Vec<Vec<&[char]>>, simp_length: Vec<VecDeque<usize>>) {
    let mut sum = 0;
    for (i,row) in simp_data.iter().enumerate(){
        let mut sum_at_start = sum;
        if row.is_empty() {sum += 1; println!("[{i}] Sum middle: {}",sum-sum_at_start); continue;}  // There is only one option possible
        let spaces = row.iter().fold(0, |acc,v| acc+v.len());
        // Try out every solution
        let mut nr_s: Vec<usize> = vec![0;simp_length[i].len()];  // The offset between each part (to tryout more solutions)
        'SolutionsLoop: loop{
            let mut current_zone = 0;
            let mut used_space = 0;
            // Go over each element and see if it fits
            let mut i_el = 0;
            while i_el < simp_length[i].len() && current_zone < row.len() {
                // Check if the element can be added to this line  (if there already is a element on this zone, insert a "." in between the elements)
                if used_space + nr_s[i_el] + simp_length[i][i_el] + (if used_space != 0 {1} else {0}) <= row[current_zone].len() {
                    let add_used_space = nr_s[i_el] + simp_length[i][i_el] + (if used_space != 0 {1} else {0});
                    // It can be added by just looking at the space, but also check if there is no # between the elements
                    if used_space+add_used_space < row[current_zone].len() && row[current_zone][used_space+add_used_space] == '#' {  // Cannot place it here
                        // Increment nr_s at this point
                        nr_s = nr_s.iter().enumerate().map(|(i,&v)| if i > i_el {0}else{v}).collect();
                        nr_s[i_el] += 1;
                        continue 'SolutionsLoop; 
                    }
                    // Check if it has a # before it, cause then it also cannot be placed
                    if used_space+add_used_space-simp_length[i][i_el] > 0 && row[current_zone][used_space+add_used_space-simp_length[i][i_el]-1] == '#' {
                        nr_s = nr_s.iter().enumerate().map(|(i,&v)| if i > i_el {0}else{v}).collect();
                        nr_s[i_el] += 1;
                        continue 'SolutionsLoop; 
                    }
                    // Insert the element
                    used_space += add_used_space;
                    // If it just added the last element, than this is a solution
                    if i_el == simp_length[i].len()-1 {
                        // println!("{:?}",nr_s);
                        sum += 1;
                        // Increment the nr_s values 
                        let last_index = nr_s.len()-1;
                        nr_s[last_index] += 1;
                        continue 'SolutionsLoop; 
                    }
                    i_el += 1;
                    continue;
                }
                // It doesn't fit
                current_zone += 1;
                used_space = 0;
            }
            // The last solution was not valid, so we have reached a max of one (or more) of the nr_s
            // So set the last non-zero value to zero and increment the value before it
            if let Some(rev_index) = nr_s.iter().rev().position(|&v| v != 0) {
                let index = nr_s.len()-rev_index-1;
                nr_s[index] = 0;
                if index == 0 {break;}  // Reached the end of all possible solutions
                nr_s[index-1] += 1;
                // println!("{:?}",nr_s);
            }else {
                // No elements were found
                break;
            }
        }
        println!("[{i}] Sum middle: {}",sum-sum_at_start);
    }
    println!("Sum at end: {}",sum);
    
}

// ?.?.?.?.?#?.?.. 1,1,1,3,1 gives an sim_length[i] = [] and slices = [[?]]

// Simplify the data by substituting ? that can only be one value. a.k.a. remove from the front and back parts that are fixed
fn simplify<'a>(data: &'a Vec<Vec<char>>,lengths: &'a Vec<VecDeque<usize>>, do_anything: bool)-> (Vec<Vec<&'a [char]>>,Vec<VecDeque<usize>> ) {
    let mut simplified: Vec<Vec<&[char]>> = vec![];
    let mut sim_length: Vec<VecDeque<usize>> = lengths.clone();
    // println!("(56) {:?}",sim_length[56]);  
    // Over each row
    for (i,row) in data.iter().enumerate(){
        // Split in the #? sections
        let mut slices: VecDeque<&[char]> = row.split(|v| *v == '.').collect();
        slices.retain(|&x| !x.is_empty());
        let mut something_changed = false;
        while something_changed{
            something_changed = do_anything;
            // Check the first section of # and ?
            // println!("({i}) {:?}",sim_length[i]);
            // println!("({i}) {:?}",slices);
            if sim_length[i][0] == slices[0].len() { 
                // Remove the first element
                slices.pop_front();
                sim_length[i].pop_front();
                something_changed = true;
            }
            // // Also check if the next element after it can fit perfectly in the left over space
            // if slices.len() > 1 && sim_length[i][0]+1 == slices[0].len() + slices[1].len(){
            //     slices.pop_front();
            //     sim_length[i].pop_front();
            //     something_changed = true;
            // }
            // // Remove the first zone if the first element doesn't fit in it
            // if !sim_length.is_empty() && !slices.is_empty() && slices[0].len() > sim_length[i][0] {
            //     slices.pop_front();
            //     sim_length[i].pop_front();
            //     something_changed = true;
            // }
            // Check the last section
            if let (Some(last_len),Some(last_slice)) = (sim_length[i].back(), slices.back()){
                if *last_len == last_slice.len() {
                    // Remove the last element
                    slices.pop_back();
                    sim_length[i].pop_back();
                    something_changed = true;
                }
                //  else if last_slice.len() > *last_len {  // Remove the last zone if the last element doesn't fit in it
                //     slices.pop_back();
                //     sim_length[i].pop_back();
                //     something_changed = true;
                // }
            }
            if slices.is_empty() {break;}
        }
        simplified.push(slices.into());
    }
    return (simplified,sim_length)
}

// Extract the data into rgb arrays per game
fn process_data() -> (Vec<Vec<char>>, Vec<VecDeque<usize>>) {
    let mut data: Vec<Vec<char>> = vec![];
    let mut lengths: Vec<VecDeque<usize>> = vec![].into();
    // Open file
    let file_path = "inputs/day12.txt";
    // Collect data to array
    if let Ok(lines) = read_lines(file_path){
        for line in lines{
            if let Ok(line) = line{
                // Data looks like ?###??...??.???? 3,2,1
                let parts = line.split_whitespace().collect::<Vec<&str>>();
                data.push(parts[0].chars().collect::<Vec<char>>());
                lengths.push(parts[1].split(",").map(|f| f.parse::<usize>().expect(&format!("Could not parse char to usize: {f}"))).collect::<VecDeque<usize>>());
            }
        }
    }else{
    }
    return (data, lengths)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}