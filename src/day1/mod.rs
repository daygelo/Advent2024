pub fn part_1(input: &str) -> u32 {
    let mut point = 50;
    let mut password = 0;

    for line in input.lines() {
        let (direction, distance) = line.split_at(1);
        let direction = if direction == "R" { 1 } else { -1 };
        let distance = distance.parse::<i32>().unwrap();
        point = (point + direction * distance) % 100; // I could've also used the rem_euclid() method for positive modulo

        if point == 0 {
            password += 1;
        }
    }

    password
}

pub fn part_2(input: &str) -> i32 {
    let mut point = 50;
    let mut password = 0;

    for line in input.lines() {
        let (direction, distance) = line.split_at(1);
        let direction = if direction == "R" { 1 } else { -1 };
        let distance = distance.parse::<i32>().unwrap();
        
        if direction > 0 || point == 0 {
            password += (point + distance) / 100;
        } else {
            password += (100 - point + distance) / 100;
        }
        
        point = (point + direction * distance).rem_euclid(100); // positive modole necessary
    }

    password
}