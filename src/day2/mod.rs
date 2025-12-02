pub fn part1(input: &str) -> u32 {
    let lines = input.lines();
    let mut total: u32 = 0;

    for line in lines {
        let mut report: Vec<i32> = line.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if report[0] == report[1] {
            continue
        } else if report[0] < report[1] {
            report.reverse();
        }

        let mut is_safe = true;

        for i in 0..(report.len() - 1) {
            if !(1..=3).contains(&(report[i] - report[i + 1])) {
                is_safe = false;
                break;
            }
        }

        if is_safe {
            total += 1;
        }
    }

    total
}

pub fn part2(input: &str) -> u32 {
    let lines = input.lines();
    let mut total: u32 = 0;

    for line in lines {
        let mut report: Vec<i32> = line.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut last_life = false;
        let mut indexes_to_be_removed: (usize, usize) = (0, 1);

        let mut direction = 0;
        for i in 0..(report.len() - 1) {
            direction += (report[i + 1] - report[i]).signum();
        }
        direction = direction.signum();

        if direction == 1 {
            report.reverse();
        }

        let mut is_safe = true;

        'big: loop {
            if last_life {
                let mut report1 = report.clone();
                let mut report2 = report;

                report1.remove(indexes_to_be_removed.0);
                report2.remove(indexes_to_be_removed.1);

                for i in 0..(report1.len() - 1) {
                    // println!("{} -> {}", report1[i], report1[i + 1]);
                    if !(1..=3).contains(&(report1[i] - report1[i + 1])) {
                        is_safe = false;
                        // println!("break");
                        break;
                    }
                }

                if !is_safe {
                    is_safe = true;
                    for i in 0..(report2.len() - 1) {
                        // println!("{} -> {}", report2[i], report2[i + 1]);
                        if !(1..=3).contains(&(report2[i] - report2[i + 1])) {
                            is_safe = false;
                            break 'big;
                        }
                    }
                }
            } else {
                for i in 0..(report.len() - 1) {
                    // println!("{} -> {}", report[i], report[i + 1]);
                    if !(1..=3).contains(&(report[i] - report[i + 1])) {
                        if last_life {
                            is_safe = false;
                            break 'big;
                        }

                        indexes_to_be_removed = (i + 1, i);
                        last_life = true;
                        // println!("plan2");
                        continue 'big;
                    }
                }
            }

            break 'big;
        }

        // println!("{} d{}", is_safe, direction);
        // println!("");

        if is_safe {
            total += 1;
        }
    }

    total
}