pub fn part1(input: &str) -> u32 {
    let chars = input.chars().collect::<Vec<char>>();
    let mut current: String = "".to_string();
    let mut total: u32 = 0;

    let mut i: usize = 0;
    while i < input.len() {
        if i + 4 < input.len() && &input[i..(i + 4)] == "mul(" {
            current += "mul(";
            i += 4;
        } else if current.starts_with("mul(") {
            let char = chars[i];
            if char.is_digit(10) {
                current.push(char);
            } else if char == ',' && current.len() > 4 {
                current.push(char);
            } else if char == ')' && current.starts_with("mul(") && current.contains(',') {
                total += current[4..].split(',').map(|x| x.parse::<u32>().unwrap()).product::<u32>();
                current.clear();
            } else {
                current.clear();
            }
            i += 1;
        } else {
            current.clear();
            i += 1;
        }
    }

    total
}

pub fn part2(input: &str) -> u32 {
    let chars = input.chars().collect::<Vec<char>>();
    let mut current: String = "".to_string();
    let mut total: u32 = 0;
    let mut enabled: bool = true;

    let mut i: usize = 0;
    while i < input.len() {
        if enabled && i + 4 < input.len() && &input[i..(i + 4)] == "mul(" {
            current += "mul(";
            i += 4;
        } else if i + 4 < input.len() && &input[i..(i + 4)] == "do()" {
            enabled = true;
            current.clear();
            i += 4;
        } else if i + 7 < input.len() && &input[i..(i + 7)] == "don't()" {
            enabled = false;
            current.clear();
            i += 7;
        } else if current.starts_with("mul(") {
            let char = chars[i];
            if char.is_digit(10) {
                current.push(char);
            } else if char == ',' && current.len() > 4 {
                current.push(char);
            } else if char == ')' && current.starts_with("mul(") && current.contains(',') {
                total += current[4..].split(',').map(|x| x.parse::<u32>().unwrap()).product::<u32>();
                current.clear();
            } else {
                current.clear();
            }
            i += 1;
        } else {
            current.clear();
            i += 1;
        }
    }

    total
}