use std::env;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    run(4);
}

fn run(num: i32){
    match num {
        1 => day1::day1(),
        2 => day2::day2(),
        3 => day3::day3(),
        4 => day4::day4(),
        _ => println!("Error: Day not found!"),
    }
}
