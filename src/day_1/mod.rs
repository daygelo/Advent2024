pub fn part1(input: String) -> i32 {
    let lines = input.lines();

    let mut left = Vec::<i32>::new();
    let mut right = Vec::<i32>::new();

    for line in lines {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        left.push(numbers[0].parse::<i32>().unwrap());
        right.push(numbers[1].parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let mut total: i32 = 0;

    for (l, r) in left.iter().zip(right.iter()) {
        total += (l - r).abs();
    }

    total
}