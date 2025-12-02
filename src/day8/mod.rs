use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    let mut width: i32 = 0;
    let mut height: i32 = 0;
    let mut y: i32 = 0;
    for line in input.lines() {
        let mut x: i32 = 0;
        for ch in line.chars() {
            match ch {
                '.' => (),
                _ if antennas.contains_key(&ch) => {
                    antennas.get_mut(&ch).unwrap().push((x, y));
                }
                _ => {
                    antennas.insert(ch, vec![(x, y)]);
                }
            }

            x += 1;
        }

        width = x;
        y += 1;
    }
    height = y;

    let mut antinodes: Vec<(i32, i32)> = Vec::new();
    for (_, positions) in &antennas {
        let mut i: usize = 0;
        for position_1 in positions {
            for position_2 in &positions[(i + 1)..] {
                let slope = (position_1.0 - position_2.0, position_1.1 - position_2.1);
                let antinode_1 = (position_1.0 + slope.0, position_1.1 + slope.1);
                let antinode_2 = (position_2.0 - slope.0, position_2.1 - slope.1);

                if (0..width).contains(&antinode_1.0) && (0..height).contains(&antinode_1.1) && !antinodes.contains(&antinode_1) {
                    antinodes.push(antinode_1);
                }
                if (0..width).contains(&antinode_2.0) && (0..height).contains(&antinode_2.1) && !antinodes.contains(&antinode_2) {
                    antinodes.push(antinode_2);
                }
            }

            i += 1;
        }
    }

    antinodes.len()
}

pub fn part2(input: &str) -> usize {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    let mut width: i32 = 0;
    let mut height: i32 = 0;
    let mut y: i32 = 0;
    for line in input.lines() {
        let mut x: i32 = 0;
        for ch in line.chars() {
            match ch {
                '.' => (),
                _ if antennas.contains_key(&ch) => {
                    antennas.get_mut(&ch).unwrap().push((x, y));
                }
                _ => {
                    antennas.insert(ch, vec![(x, y)]);
                }
            }

            x += 1;
        }

        width = x;
        y += 1;
    }
    height = y;

    let mut antinodes: Vec<(i32, i32)> = Vec::new();
    for (f, positions) in &antennas {
        let mut i: usize = 0;
        for position_1 in positions {
            for position_2 in &positions[(i + 1)..] {
                let slope = (position_1.0 - position_2.0, position_1.1 - position_2.1);
                let mut antinode = *position_1;
                loop {
                    if !(0..width).contains(&antinode.0) || !(0..height).contains(&antinode.1) {
                        break;
                    }
                    if !antinodes.contains(&antinode) {
                        antinodes.push(antinode);
                    }
                    antinode.0 += slope.0;
                    antinode.1 += slope.1;
                }

                antinode = *position_2;
                loop {
                    if !(0..width).contains(&antinode.0) || !(0..height).contains(&antinode.1) {
                        break;
                    }
                    if !antinodes.contains(&antinode) {
                        antinodes.push(antinode);
                    }
                    antinode.0 -= slope.0;
                    antinode.1 -= slope.1;
                }
            }

            i += 1;
        }
    }

    antinodes.len()
}