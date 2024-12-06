use std::collections::HashMap;

pub fn part1(input: &str) -> u32 {
    let mut split_input = input.split("\n\n");

    let rules: HashMap<u32, Vec<u32>> = split_input.next().unwrap().lines().fold(HashMap::new(), |mut acc, x| {
        let mut split = x.split('|');
        let first = split.next().unwrap().parse().unwrap();
        let second = split.next().unwrap().parse().unwrap();

        if acc.contains_key(&first) {
            acc.get_mut(&first).unwrap().push(second);
        } else {
            acc.insert(first, vec![second]);
        }

        acc
    });

    let mut total: u32 = 0;

    for line in split_input.next().unwrap().lines() {
        let numbers = line.split(',').map(|x| x.parse().unwrap()).collect::<Vec<u32>>();

        let mut valid = true;
        let mut i: usize = 0;
        while i < numbers.len() {
            for j in i..numbers.len() {
                if rules.get(&numbers[j]).unwrap().contains(&numbers[i]) {
                    valid = false;
                }
            }

            i += 1;
        }

        if valid {
            total += numbers[numbers.len() / 2];
        }
    }

    total
}

pub fn part2(input: &str) -> u32 {
    let mut split_input = input.split("\n\n");

    let rules: HashMap<u32, Vec<u32>> = split_input.next().unwrap().lines().fold(HashMap::new(), |mut acc, x| {
        let mut split = x.split('|');
        let first = split.next().unwrap().parse().unwrap();
        let second = split.next().unwrap().parse().unwrap();

        if acc.contains_key(&first) {
            acc.get_mut(&first).unwrap().push(second);
        } else {
            acc.insert(first, vec![second]);
        }

        acc
    });

    let mut total: u32 = 0;

    for line in split_input.next().unwrap().lines() {
        let mut numbers = line.split(',').map(|x| x.parse().unwrap()).collect::<Vec<u32>>();

        let mut valid = true;
        let mut i: usize = 0;
        while i < numbers.len() {
            for j in i..numbers.len() {
                if rules.get(&numbers[j]).unwrap().contains(&numbers[i]) {
                    valid = false;
                    (numbers[i], numbers[j]) = (numbers[j], numbers[i]);
                }
            }

            i += 1;
        }

        if !valid {
            total += numbers[numbers.len() / 2];
        }
    }

    total
}