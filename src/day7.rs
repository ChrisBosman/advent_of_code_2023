use std::{fs::File, io::{self, BufRead}, path::Path, vec, cmp::Ordering};
use itertools::Itertools;

pub(crate) 
fn day7(){
    println!("This is day 7");
    let (mut data,mut data_part2) = process_data();
    data.sort_unstable_by(sort_hands);
    data_part2.sort_unstable_by(sort_hands_part_2);
    let sum = data.iter().enumerate().fold(0, |acc,(i,(_,bet,_))| {
        acc + (<usize as TryInto<u32>>::try_into(i).expect("Could not convert usize to u32")+1)**bet
    });
    let sum_part2 = data_part2.iter().enumerate().fold(0, |acc,(i,(_,bet,_))| {
        acc + (<usize as TryInto<u32>>::try_into(i).expect("Could not convert usize to u32")+1)**bet
    });
    println!("Sum {}",sum);
    println!("Part 2\nSum {}",sum_part2);
}

// Extract the data into rgb arrays per game
fn process_data() -> (Vec<(String, u32, u8)>,Vec<(String, u32, u8)>) {
    let mut data: Vec<(String,u32,u8)> = vec![];
    let mut data_part2: Vec<(String,u32,u8)> = vec![];
    // Open file
    let file_path = "inputs/day7.txt";
    // Collect data to array
    if let Ok(lines) = read_lines(file_path){
        for line in lines{
            if let Ok(line) = line{
                // The data looks like:  "hand bet" example: "ATJ82 232"
                let mut parts = line.split_whitespace();
                let hand = parts.nth(0).expect(&format!("[0] Could not split on white spaces {}", line)).to_string();
                let bet = parts.next().expect(&format!("[1] Could not split on white spaces \"{}\"", line)).parse::<u32>().expect(&format!("Could not parse into u32 {:?}", parts.nth(1)));
                let hand_type = eval_type(&hand);
                let hand_type_part2 = eval_type_part2(&hand);
                data.push((hand.clone(),bet,hand_type));
                data_part2.push((hand,bet,hand_type_part2));
            }
        }
    }else{
        println!("Error: Could not read the file!:\n{}",file_path);
    }
    return (data,data_part2);
}

// Sort
fn sort_hands(a:&(String,u32,u8),b:&(String,u32,u8))-> std::cmp::Ordering {
    match a.2.cmp(&b.2) {
        Ordering::Less => return Ordering::Less,
        Ordering::Greater => return Ordering::Greater,
        Ordering::Equal => {
            return compare_equal_hands(&a.0,&b.0);
        }
    }
}

// Compare hands that have the same type
fn compare_equal_hands(a: &String, b: &String) -> Ordering {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    for i in 0..a_chars.len(){
        let value = compare_value(a_chars[i], b_chars[i]);
        if value != Ordering::Equal {
            return value;
        }
    }
    return Ordering::Equal
}

fn compare_value(a_char: char, b_char: char) -> Ordering {
    match a_char {
        'A' | 'K' | 'Q' | 'J' | 'T' => {
            match b_char {
                'A' | 'K' | 'Q' | 'J' | 'T' => {  // Compare the A,K,Q,J,T
                    let match_indices_a: Vec<_> = "TJQKA".match_indices(a_char).collect();
                    let match_indices_b: Vec<_> = "TJQKA".match_indices(b_char).collect();
                    return match_indices_a[0].0.cmp(&match_indices_b[0].0);
                },
                _ => return Ordering::Greater, // A is greater then B
            }
        },
        _ => {
            match b_char {
                'A' | 'K' | 'Q' | 'J' | 'T' => return Ordering::Less, // B is greater then A
                _ => return a_char.cmp(&b_char),  // Compare numbers
            }
        },
    }
}

// Evaulate the type: Five of a kind(6), Four of a kind(5), Full house(4), Three of a kind(3), Two pair(2), One pair(1), High card(0)
fn eval_type(hand: &String) -> u8 {
    // Find the unique elements in the hand
    let chars: Vec<char> = hand.chars().collect();
    let unique: Vec<&char> = chars.iter().unique().collect();
    match unique.len() {
        5 => return 0,  // There are 5 different elements, so it can only be a high card
        4 => return 1,  // There are 4 different elements, so it can only be a pair
        3 => {          // There are 3 different elements, so it can only be two pair or three of a kind
            let nr_matches_0 = hand.matches(*unique[0]).collect::<Vec<&str>>().len();
            let nr_matches_1 = hand.matches(*unique[1]).collect::<Vec<&str>>().len();
            if nr_matches_0 == 2 || nr_matches_1 == 2 { return 2;}  // Two pair [2 2 1]
            return 3;  // Three of a kind [1 1 3]
        },
        2 => {          // There are 2 different elements, so it can only be Full House or Four of a kind
            let nr_matches = hand.matches(*unique[0]).collect::<Vec<&str>>().len();
            if nr_matches == 4 || nr_matches == 1 { return 5;}  // Four of a kind
            return 4;  // Full house 
        }, 
        1 => return 6,  // There is only 1 unique elements, so it can only be a Five of a kind
        _ => {
            println!("Something went wrong\nUnique length is not between 5 and 1: {}",unique.len());
            return 6; //  There where 5 Js that have been filtered out
        },
    }
}

//* Part 2
// Evaulate the type: Five of a kind(6), Four of a kind(5), Full house(4), Three of a kind(3), Two pair(2), One pair(1), High card(0)
fn eval_type_part2(hand: &String) -> u8 {
    // Removes the J since they are 'depended' or duplicates of the best card
    let mut without_j = hand.clone();
    without_j.retain(|c| !"J".contains(c));
    // Put it all in the eval_type function
    return eval_type_part2_2(&without_j, &hand);
}

// TODO FIX THE FUNCTION
// Evaulate the type: Five of a kind(6), Four of a kind(5), Full house(4), Three of a kind(3), Two pair(2), One pair(1), High card(0)
fn eval_type_part2_2(hand_reduced: &String,hand: &String) -> u8 {
    // Find the unique elements in the hand
    let chars: Vec<char> = hand_reduced.chars().collect();
    let unique: Vec<&char> = chars.iter().unique().collect();
    match unique.len() {
        5 => return 0,  // There are 5 different elements, so it can only be a high card
        4 => return 1,  // There are 4 different elements, so it can only be a pair
        3 => {          // There are 3 different elements, so it can only be two pair or three of a kind
            let nr_matches_j = hand.matches('J').collect::<Vec<&str>>().len();
            if nr_matches_j != 0 {return 3;}  // Three of a kind
            let nr_matches_0 = hand.matches(*unique[0]).collect::<Vec<&str>>().len();
            let nr_matches_1 = hand.matches(*unique[1]).collect::<Vec<&str>>().len();
            if nr_matches_0 == 2 || nr_matches_1 == 2 { return 2;}  // Two pair [2 2 1]
            return 3;  // Three of a kind [1 1 3]
        },
        2 => {          // There are 2 different elements, so it can only be Full House or Four of a kind
            let nr_matches_j = hand.matches('J').collect::<Vec<&str>>().len();
            if nr_matches_j >= 2 {return 5;}  // Four of a kind  [ABJJJ] or [[AABJJ]
            let nr_matches = hand.matches(*unique[0]).collect::<Vec<&str>>().len();
            if nr_matches_j == 1 && nr_matches == 2 {return 4;}  // Full House [AABBJ]
            if nr_matches_j == 1 {return 5;}  // Four of a kind [AAABJ]
            if nr_matches == 4 || nr_matches == 1 { return 5;}  // Four of a kind
            return 4;  // Full house 
        }, 
        1 => return 6,  // There is only 1 unique elements, so it can only be a Five of a kind
        _ => {
            println!("Something went wrong\nUnique length is not between 5 and 1: {}",unique.len());
            return 6; //  There where 5 Js that have been filtered out
        },
    }
}

// Sort
fn sort_hands_part_2(a:&(String,u32,u8),b:&(String,u32,u8))-> std::cmp::Ordering {
    match a.2.cmp(&b.2) {
        Ordering::Less => return Ordering::Less,
        Ordering::Greater => return Ordering::Greater,
        Ordering::Equal => {
            return compare_equal_hands_part2(&a.0,&b.0);
        }
    }
}
// Compare hands that have the same type
fn compare_equal_hands_part2(a: &String, b: &String) -> Ordering {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    for i in 0..a_chars.len(){
        let value = compare_value_part2(a_chars[i], b_chars[i]);
        if value != Ordering::Equal {
            return value;
        }
    }
    return Ordering::Equal
}

// Now J is the lowest value
fn compare_value_part2(a_char: char, b_char: char) -> Ordering {
    match a_char {
        'A' | 'K' | 'Q' | 'T' => {
            match b_char {
                'A' | 'K' | 'Q' | 'T' => {  // Compare the A,K,Q,J,T
                    let match_indices_a: Vec<_> = "TQKA".match_indices(a_char).collect();
                    let match_indices_b: Vec<_> = "TQKA".match_indices(b_char).collect();
                    return match_indices_a[0].0.cmp(&match_indices_b[0].0);
                },
                _ => return Ordering::Greater, // A is greater then B    Note: Now also includes B='J'
            }
        },
        'J' => {
            if b_char != 'J' { return Ordering::Less;}
            return Ordering::Equal;
        }
        _ => {
            match b_char {
                'A' | 'K' | 'Q' | 'T' => return Ordering::Less, // B is greater then A
                'J' => return Ordering::Greater,
                _ => return a_char.cmp(&b_char),  // Compare numbers
            }
        },
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}