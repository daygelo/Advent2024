use std::collections::HashMap;

fn get_columns(input: &str) -> (Vec<i32>, Vec<i32>) {
    let lines = input.lines();
    let mut left = Vec::<i32>::new();
    let mut right = Vec::<i32>::new();

    for line in lines {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        left.push(numbers[0].parse::<i32>().unwrap());
        right.push(numbers[1].parse::<i32>().unwrap());
    }

    (left, right)
}

pub fn part1(input: &str) -> u32 {
    let (mut left, mut right) = get_columns(input);

    left.sort();
    right.sort();

    let mut total: u32 = 0;

    for (l, r) in left.into_iter().zip(right) {
        total += l.abs_diff(r);
    }

    total
}

pub fn part2(input: &str) -> i32 {
    let (left, right) = get_columns(input);
    let mut map: HashMap<i32, usize> = HashMap::new();

    for l in left.iter() {
        if !map.contains_key(l) && right.contains(l) {
            let left_count = left.iter().filter(|x| x == &l).count();
            let right_count = right.iter().filter(|x| x == &l).count();
            map.insert(*l, left_count * right_count);
        }
    }

    let mut score: i32 = 0;

    for (key, value) in map {
        score += key * value as i32;
    }

    score
}