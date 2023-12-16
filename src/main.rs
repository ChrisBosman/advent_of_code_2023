use std::env;

// pub mod day1;
// pub mod day2;
// pub mod day3;
// pub mod day4;
// pub mod day5;
// pub mod day6;
// pub mod day7;
// pub mod day8;
pub mod day9;
pub mod day10;
pub mod day12;
pub mod day15;
pub mod day16;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    run(16);
}

fn run(num: i32){
    match num {
        // 1 => day1::day1(),
        // 2 => day2::day2(),
        // 3 => day3::day3(),
        // 4 => day4::day4(),
        // 5 => day5::day5(),
        // 6 => day6::day6(),
        // 7 => day7::day7(),
        // 8 => day8::day8(),
        9 => day9::day9(),
        10 => day10::day10(),
        12 => day12::day12(),
        15 => day15::day15(),
        16 => day16::day16(),
        _ => println!("Error: Day not found!"),
    }
}
