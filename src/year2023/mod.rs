mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

pub fn runner(day: &String) -> String {
    if day == "1" {
        format!("{}", day_1::solution())
    } else if day == "2" {
        format!("{}", day_2::solution())
    } else if day == "3" {
        format!("{}", day_3::fast_solution())
    } else if day == "3.5" {
        format!("{}", day_3::struct_solution())
    } else if day == "4" {
        format!("{}", day_4::solution())
    } else if day == "5" {
        format!("{}", day_5::solution())
    } else if day == "5.5" {
        format!("{}", day_5::slow_solution())
    } else if day == "6" {
        format!("{}", day_6::solution())
    } else if day == "7" {
        format!("{}", day_7::solution())
    } else if day == "8" {
        format!("{}", day_8::solution())
    } else if day == "9" {
        format!("{}", day_9::solution())
    } else {
        format!("Unknown day input {day}")
    }
}
