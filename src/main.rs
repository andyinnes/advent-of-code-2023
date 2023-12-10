use std::env;
use chrono::offset::Local;
mod year2022;
mod year2023;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Must supply problem argument");
    }
    let mut year = &String::from("2023");
    let mut day = &args[1];
    if args.len() == 3 {
        year = &args[1];
        day = &args[2];
    }
    let start_dt = Local::now();
    if year == "2023" {
        println!("{}", year2023::runner(day));
    } else if year == "2022" {
        println!("{}", year2022::runner(day));
    } else {
        println!("Unknown year input {year}");
    }
    let dur = Local::now() - start_dt;
    println!("Duration: {dur}");
}
