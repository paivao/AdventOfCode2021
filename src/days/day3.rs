pub fn run(input: &str) -> Result<String, String> {
    let mut lines = input.lines();
    let mut bits: Vec<(usize, usize)> = Vec::new();
    let first_line = lines.next().ok_or(String::from("Blank input!"))?;
    for ch in first_line.chars() {
        match ch {
            '0' => bits.push((1, 0)),
            '1' => bits.push((0, 1)),
            _ => return Err(format!("Weird input: {}", ch))
        }
    }
    for line in lines {
        for (pos, ch) in line.char_indices() {
            match ch {
                '0' => bits[pos].0 += 1,
                '1' => bits[pos].1 += 1,
                _ => return Err(format!("Weird input: {}", ch))
            }
        }
    }
    let mut gamma = 0usize;
    let mut epsilon = 0usize;
    for bit in bits.iter() {
        gamma <<= 1;
        epsilon <<= 1;
        if bit.0 > bit.1 {
            epsilon |= 1;
        } else {
            gamma |= 1;
        }
    }
    Ok(format!("Power consuption: {}", gamma * epsilon))
}