use std::{fs::File, io::{self, BufRead}, path::Path, vec};

use itertools::Itertools;

const MIN_RANGE: i64 = 200000000000000;
const MAX_RANGE: i64 = 400000000000000;

pub(crate) 
fn day24(){
    println!("This is day 24");
    let (pos,vel) = process_data();
    part1(&pos,&vel);
    part2(&pos, &vel);
}

struct Vec3d<T>{
    x: T,
    y: T,
    z: T,
}

impl<T: Clone + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T>> Vec3d<T>{
    fn new(x: T, y: T, z: T) -> Vec3d<T>{
        Vec3d{x,y,z}
    }
    fn new_from_vec(vec: Vec<T>) -> Vec3d<T>{
        Vec3d{x: vec[0].clone(), y: vec[1].clone(), z: vec[2].clone()}
    }
    fn get_vec(&self) -> Vec<T>{
        vec![self.x.clone(),self.y.clone(),self.z.clone()]
    }
    fn add(&self, other: &Vec3d<T>) -> Vec3d<T>{
        Vec3d::new(self.x.clone()+other.x.clone(),self.y.clone()+other.y.clone(),self.z.clone()+other.z.clone())
    }
    fn sub(&self, other: &Vec3d<T>) -> Vec3d<T>{
        Vec3d::new(self.x.clone()-other.x.clone(),self.y.clone()-other.y.clone(),self.z.clone()-other.z.clone())
    }
    fn mul_const(&self, other: &T) -> Vec3d<T>{
        Vec3d::new(self.x.clone()*other.clone(),self.y.clone()*other.clone(),self.z.clone()*other.clone())
    }
    fn cross(&self, other: &Vec3d<T>) -> Vec3d<T>{
        Vec3d::new(self.y.clone()*other.z.clone()-self.z.clone()*other.y.clone(),
                   self.z.clone()*other.x.clone()-self.x.clone()*other.z.clone(),
                   self.x.clone()*other.y.clone()-self.y.clone()*other.x.clone())
    }
}

fn part2(pos: &Vec<[i64; 3]>, vel: &Vec<[i64; 3]>) {
    // change i64 to i128
    let mut pos: Vec<[i128; 3]> = pos.iter().map(|f| [f[0] as i128, f[1] as i128, f[2] as i128]).collect();
    let mut vel: Vec<[i128; 3]> = vel.iter().map(|f| [f[0] as i128, f[1] as i128, f[2] as i128]).collect();
    // Shift everything so that line 1 is at the origin, and then calculate a plane normal through the origin and the shifted second line
    let x1 = Vec3d::new_from_vec(pos[0].to_vec());
    let v1 = Vec3d::new_from_vec(vel[0].to_vec());
    let x2 = Vec3d::new_from_vec(pos[1].to_vec());
    let v2 = Vec3d::new_from_vec(vel[1].to_vec());
    let x3 = Vec3d::new_from_vec(pos[2].to_vec());
    let v3 = Vec3d::new_from_vec(vel[2].to_vec());
    let x4 = Vec3d::new_from_vec(pos[3].to_vec());
    let v4 = Vec3d::new_from_vec(vel[3].to_vec());
    let normal = (x2.sub(&x1)).cross(&(v2.sub(&v1)));
    let x_3_shifted = x3.sub(&x1);
    let v_3_shifted = v3.sub(&v1);
    let x_4_shifted = x4.sub(&x1);
    let v_4_shifted = v4.sub(&v1);
    // Find the intersection of the plane and the shifted lines
    let t3 = (-normal.x*x_3_shifted.x -normal.y*x_3_shifted.y -normal.z*x_3_shifted.z) /
                  (normal.x*v_3_shifted.x + normal.y*v_3_shifted.y + normal.z*v_3_shifted.z);
    let t4 = (-normal.x*x_4_shifted.x -normal.y*x_4_shifted.y -normal.z*x_4_shifted.z) /
                  (normal.x*v_4_shifted.x + normal.y*v_4_shifted.y + normal.z*v_4_shifted.z);
    
    println!("t3: {}, t4: {}",t3,t4);

    // Now find the initial position for the trow by drawing a line though t3 and t4
    let x3_t3 = x3.add(&v3.mul_const(&t3));
    let x4_t4 = x4.add(&v4.mul_const(&t4));
    let x0 = x3_t3.mul_const(&t4).sub(&x4_t4.mul_const(&t3)).get_vec().iter().map(|f| *f as f64 /(t4 as f64-t3 as f64)).collect_vec();

    println!("x0: {:?}",x0);
    println!("sum: {}",x0.iter().sum::<f64>());
}

fn part1(pos: &Vec<[i64; 3]>, vel: &Vec<[i64; 3]>) {
    let mut sum = 0;
    for i in 0..pos.len(){
        for j in i..pos.len(){
            if let Some(t2) = calc_inter_t2(pos[i],vel[i],pos[j],vel[j]){
                // Check time
                if t2 < 0 {continue;}
                if calc_t1(pos[i],vel[i],pos[j],vel[j],t2) < 0 {continue;}

                // Find intersection point
                let (x,y) = get_xy_with_t2(pos[j],vel[j],t2);
                if x > MIN_RANGE && x < MAX_RANGE && y > MIN_RANGE && y < MAX_RANGE{
                    sum += 1;
                }
            }
        }
    }
    println!("Intersections: {sum}");
}

fn calc_t1(p1: [i64; 3], v1: [i64; 3], p2: [i64; 3], v2: [i64; 3], t2: i64) -> i64 {
    // t1 = (x2 + Vx2*t2 - x1) / (Vx1)
    return (p2[0] + v2[0]*t2 - p1[0]) / (v1[0]);
}

fn get_xy_with_t2(p2: [i64; 3], v2: [i64; 3], t2: i64) -> (i64,i64) {
    let x = p2[0] + v2[0] *t2;
    let y = p2[1] + v2[1] *t2;  
    return (x,y)
}

fn calc_inter_t2(p1: [i64; 3], v1: [i64; 3],p2: [i64; 3], v2: [i64; 3]) -> Option<i64> {
    // t2 = ( Vx1*(y2-y1) + Vy1*(x1-x2) ) / (Vy1*Vx2-vy2*Vx1)
    if v1[1]*v2[0]-v2[1]*v1[0] == 0 { return None}
    return  Some(( v1[0]*(p2[1]-p1[1]) + v1[1]*(p1[0]-p2[0]) ) / (v1[1]*v2[0]-v2[1]*v1[0]))
}

// Extract the data into rgb arrays per game
fn process_data() -> (Vec<[i64; 3]>, Vec<[i64; 3]>) {
    let mut pos: Vec<[i64;3]> = vec![];
    let mut vel: Vec<[i64;3]> = vec![];
    // Open file
    let file_path = "inputs/day24.txt";
    // Collect data to array
    if let Ok(lines) = read_lines(file_path){
        for line in lines{
            if let Ok(line) = line{
                let parts = readf::<i64>(vec![", ",", "," @ ",", ",", "],line);
                pos.push(parts.get(0..3).expect("Could not get the position").try_into().expect("Could not put in [i64;3]"));
                vel.push(parts.get(3..6).expect("Could not get the velocity").try_into().expect("Could not put in [i64;3]"));
            }
        }
    }else{
        println!("Error: Could not read the file!:\n{}",file_path);
    }
    return (pos,vel);
}

/// ### Read formated string,
/// Format, the text around the values to extract, i.e. "Game 1, red 4, green 5"
/// -> format = vec!["Game ",", red ",", green "]
fn readf<T: std::str::FromStr + std::fmt::Display>(format: Vec<&str>, input: String)-> Vec<T> {
    let mut output: Vec<T> = vec![];
    let mut next_string = input;
    for ele in format {
        let tmp = next_string.splitn(2,&ele).map(|f| f.to_string()).collect::<Vec<String>>();
        next_string = tmp[1].to_string();
        if let Ok(val) = tmp[0].trim().to_string().parse::<T>(){
            output.push(val);
        }
    }
    if let Ok(val) = next_string.trim().parse::<T>(){
        output.push(val);
    }
    return output
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}