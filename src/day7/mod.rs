pub fn part1(input: &str) -> u64 {
    let mut total: u64 = 0;

    for line in input.lines() {
        let split_line = line.split(": ").collect::<Vec<&str>>();
        let value = split_line[0].parse::<u64>().unwrap();
        let numbers = split_line[1].split(' ')
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        for i in 0..2i32.pow((numbers.len() - 1) as u32) {
            let mut current: u64 = numbers[0];
            for n in 1..numbers.len() {
                if ((i >> (n - 1)) & 1) == 0 {
                    current += numbers[n];
                } else {
                    current *= numbers[n];
                }
            }

            if current == value {
                total += value;
                break;
            }
        }
    }

    total
}

pub fn part2(input: &str) -> u64 {
    let mut total: u64 = 0;

    for line in input.lines() {
        let split_line = line.split(": ").collect::<Vec<&str>>();
        let value = split_line[0].parse::<u64>().unwrap();
        let numbers = split_line[1].split(' ')
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        for i in 0..3i32.pow((numbers.len() - 1) as u32) {
            let mut current: u64 = numbers[0];
            for n in 1..numbers.len() {
                let x = i / (3i32.pow(n as u32 - 1)) % 3;
                current = match x {
                    0 => current + numbers[n],
                    1 => current * numbers[n],
                    _ => (current.to_string() + &*numbers[n].to_string()).parse::<u64>().unwrap(),
                };
            }

            if current == value {
                total += value;
                break;
            }
        }
    }

    total
}