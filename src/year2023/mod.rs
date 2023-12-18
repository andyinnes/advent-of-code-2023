mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;

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
    } else if day == "10" {
        format!("{}", day_10::solution())
    } else if day == "11" {
        format!("{}", day_11::solution())
    } else if day == "12" {
        format!("{}", day_12::solution())
    } else if day == "13" {
        format!("{}", day_13::solution())
    } else if day == "14" {
        format!("{}", day_14::solution())
    } else if day == "15" {
        format!("{}", day_15::solution())
    } else if day == "16" {
        format!("{}", day_16::solution())
    } else if day == "17" {
        format!("{}", day_17::solution())
    } else if day == "18" {
        format!("{}", day_18::solution())
    } else {
        format!("Unknown day input {day}")
    }
}
