mod day1;
mod day2;
mod day3;

pub fn run(day: usize, input: &str) -> Result<String, String> {
    match day {
        1 => day1::run(input),
        2 => day2::run(input),
        3 => day3::run(input),
        _ => Err(String::from("Wrong day")),
    }
}
