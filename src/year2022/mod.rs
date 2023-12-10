mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

pub fn runner(day: &String) -> String {
    if day == "1" {
        format!("{}", day_1::solution())
    } else if day == "2" {
        format!("{}", day_2::solution())
    } else if day == "3" {
        format!("{}", day_3::solution())
    } else if day == "4" {
        format!("{}", day_4::solution())
    } else if day == "5" {
        format!("{}", day_5::solution())
    } else {
        format!("Unknown day input {day}")
    }
}
