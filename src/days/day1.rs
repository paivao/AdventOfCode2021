use std::collections::VecDeque;

pub fn run(input: &str) -> Result<String, String> {
    let mut window = VecDeque::with_capacity(3);
    let mut lines = input.lines();
    let first = lines
        .next()
        .ok_or(String::from("Empty input!"))?
        .parse::<i32>()
        .map_err(|e| String::from(format!("Parse error: {}", e)))?;
    window.push_back(first);
    let mut count = 0;
    let mut count_sum = 0;
    let mut last_sum = first;
    let mut actual_sum;
    for line in lines {
        let value = match line.parse::<i32>() {
            Err(why) => return Err(String::from(format!("Parse error: {}", why))),
            Ok(val) => val,
        };
        if value > *window.back().unwrap() {
            count += 1
        }
        if window.len() < 3 {
            last_sum += value;
        } else {
            let last = window.pop_front().unwrap();
            actual_sum = last_sum + value - last;
            if actual_sum > last_sum {
                count_sum += 1;
            }
            last_sum = actual_sum;
        }
        window.push_back(value);
    }
    Ok(format!("Normal: {}, Window: {}", count.to_string(), count_sum.to_string()))
}