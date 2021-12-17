#[derive (Clone)]
struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32
}

pub fn run(input: &str) -> Result<String, String> {
    let mut old_position = Position { depth: 0, horizontal: 0, aim: 0 };
    let mut position = old_position.clone();
    for line in input.lines() {
        let (direction, quantity) = line.split_once(' ').ok_or(String::from("Could not parse line!"))?;
        let quantity = quantity.parse::<i32>().map_err(|e| format!("Failed parting quantity: {}", e))?;
        match direction {
            "forward" => {
                old_position.horizontal += quantity;
                position.horizontal += quantity;
                position.depth += position.aim * quantity;
            },
            "down" => {
                old_position.depth += quantity;
                position.aim += quantity;
            },
            "up" => {
                old_position.depth -= quantity;
                position.aim -= quantity;
            },
            &_ => return Err(format!("Unknown direction: {}", direction))
        }
    }
    Ok(format!("OLD: {}\nNEW: {}", old_position.horizontal * old_position.depth, position.depth * position.horizontal))
}