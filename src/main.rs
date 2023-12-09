use std::env;
use chrono::offset::Local;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Must supply problem argument");
    }
    let day = &args[1];
    let start_dt = Local::now();
    if day == "1" {
        println!("{}", day_1::solution());
    } else if day == "2" {
        println!("{}", day_2::solution());
    } else if day == "3" {
        println!("{}", day_3::fast_solution());
    } else if day == "3.5" {
        println!("{}", day_3::struct_solution());
    } else if day == "4" {
        println!("{}", day_4::solution());
    } else if day == "5" {
        println!("{}", day_5::solution());
    } else if day == "5.5" {
        println!("{}", day_5::slow_solution());
    } else if day == "6" {
        println!("{}", day_6::solution());
    } else if day == "7" {
        println!("{}", day_7::solution());
    } else if day == "8" {
        println!("{}", day_8::solution());
    } else if day == "9" {
        println!("{}", day_9::solution());
    } else {
        println!("Unknown day input {day}");
    }
    let dur = Local::now() - start_dt;
    println!("Duration: {dur}");
}
