pub fn part1(input: &str) -> u32 {
    let mut total: u32 = 0;
    let mut current: String = "".to_string();

    for char in input.chars() {
        match char {
            'm' if current == ""  => {
                current.push(char);
            }
            'u' if current == "m"  => {
                current.push(char);
            }
            'l' if current == "mu"  => {
                current.push(char);
            }
            '(' if current == "mul"  => {
                current.push(char);
            }
            _ if char.is_digit(10) && current.starts_with("mul(") => {
                current.push(char);
            }
            ',' if current.starts_with("mul(") && current.len() > 4 => {
                current.push(char);
            }
            ')' if current.starts_with("mul(") && current.contains(',') => {
                total += current[4..].split(',').map(|x| x.parse::<u32>().unwrap()).product::<u32>();
                current.clear();
            }

            _ => {
                current.clear();
            }
        }
    }

    total
}

pub fn part2(input: &str) -> u32 {
    let mut total: u32 = 0;
    let mut current: String = "".to_string();
    let mut enabled = true;

    for char in input.chars() {
        match char {
            'm' if current == ""  => {
                current.push(char);
            }
            'u' if enabled && current == "m"  => {
                current.push(char);
            }
            'l' if enabled && current == "mu"  => {
                current.push(char);
            }
            '(' if enabled && current == "mul"  => {
                current.push(char);
            }
            _ if enabled && char.is_digit(10) && current.starts_with("mul(") => {
                current.push(char);
            }
            ',' if enabled && current.starts_with("mul(") && current.len() > 4 => {
                current.push(char);
            }
            ')' if enabled && current.starts_with("mul(") && current.contains(',') => {
                total += current[4..].split(',').map(|x| x.parse::<u32>().unwrap()).product::<u32>();
                current.clear();
            }

            'd' if current == ""  => {
                current.push(char);
            }
            'o' if current == "d"  => {
                current.push(char);
            }
            '(' if current == "do"  => {
                current.push(char);
            }
            ')' if current == "do("  => {
                enabled = true;
                current.clear();
            }

            'n' if current == "do"  => {
                current.push(char);
            }
            '\'' if current == "don"  => {
                current.push(char);
            }
            't' if current == "don'"  => {
                current.push(char);
            }
            '(' if current == "don't"  => {
                current.push(char);
            }
            ')' if current == "don't("  => {
                enabled = false;
                current.clear();
            }

            _ => {
                current.clear();
            }
        }
    }

    total
}