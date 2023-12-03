use std::env;
#[path = "day-1/solution.rs"] mod day_1;
#[path = "day-2/solution.rs"] mod day_2;
#[path = "day-3/solution.rs"] mod day_3;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Must supply problem argument");
    }
    let day = &args[1];
    if day == "1" {
        println!("{}", day_1::solution());
    } else if day == "2" {
        println!("{}", day_2::solution());
    } else if day == "3" {
        println!("{}", day_3::solution());
    } else {
        println!("Unknown day input {day}");
    }
}
