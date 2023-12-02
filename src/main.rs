use std::env;
#[path = "day-1/solution.rs"] mod day_1;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Must supply problem argument");
    }
    let day = &args[1];
    if day == "1" {
        println!("{}", day_1::solution());
    } else {
        println!("Unknown day input {day}");
    }
}
