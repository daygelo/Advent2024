use std::collections::HashMap;

#[derive(PartialEq, Eq)]
enum Tile {
    Empty,
    Visited,
    Obstacle,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub fn part1(input: &str) -> usize {
    let mut map: HashMap<(i32, i32), Tile> = HashMap::new();
    let mut guard: (i32, i32) = (0, 0);

    let mut y: i32 = 0;
    for line in input.lines() {
        let mut x: i32 = 0;
        for ch in line.chars() {
            let pos = (x, y);
            map.insert(pos, Tile::Empty);

            match ch {
                '^' => { guard = pos; },
                '#' => { map.insert(pos, Tile::Obstacle); }
                _ => ()
            }

            x += 1;
        }

        y += 1;
    }

    let mut direction: Direction = Direction::Up;
    while map.contains_key(&guard) {
        if map[&guard] == Tile::Obstacle {
            direction = match direction {
                Direction::Up => {
                    guard.1 += 1;
                    Direction::Right
                },
                Direction::Right => {
                    guard.0 -= 1;
                    Direction::Down
                },
                Direction::Down => {
                    guard.1 -= 1;
                    Direction::Left
                },
                Direction::Left => {
                    guard.0 += 1;
                    Direction::Up
                }
            }
        } else {
            map.insert(guard, Tile::Visited);
            match direction {
                Direction::Up => { guard.1 -= 1 }
                Direction::Right => { guard.0 += 1 }
                Direction::Down => { guard.1 += 1 }
                Direction::Left => { guard.0 -= 1 }
            }
        }
    }

    map.iter().filter(|x| (*x).1 == &Tile::Visited).count()
}

fn simulate(map: &HashMap<(i32, i32), bool>, mut pos: (i32, i32), mut direction: Direction) -> bool {
    let mut collisions: Vec<(i32, i32, Direction)> = vec![];
    while map.contains_key(&pos) {
        if map[&pos] {
            direction = match direction {
                Direction::Up => {
                    pos.1 += 1;
                    Direction::Right
                },
                Direction::Right => {
                    pos.0 -= 1;
                    Direction::Down
                },
                Direction::Down => {
                    pos.1 -= 1;
                    Direction::Left
                },
                Direction::Left => {
                    pos.0 += 1;
                    Direction::Up
                }
            };

            let collision = (pos.0, pos.1, direction);
            if collisions.contains(&collision) {
                // println!("LOOP FOUND: {} {:?}", collisions.len(), collisions);
                return true;
            }
            collisions.push(collision);
        } else {
            match direction {
                Direction::Up => { pos.1 -= 1 }
                Direction::Right => { pos.0 += 1 }
                Direction::Down => { pos.1 += 1 }
                Direction::Left => { pos.0 -= 1 }
            }
        }
    }

    false
}

pub fn part2(input: &str) -> usize {
    let mut map: HashMap<(i32, i32), bool> = HashMap::new();
    let mut start: (i32, i32) = (0, 0);

    let mut y: i32 = 0;
    for line in input.lines() {
        let mut x: i32 = 0;
        for ch in line.chars() {
            let pos = (x, y);
            map.insert(pos, false);

            match ch {
                '^' => { start = pos; },
                '#' => { map.insert(pos, true); }
                _ => ()
            }

            x += 1;
        }

        y += 1;
    }

    let mut possibilities: Vec<(i32, i32)> = Vec::new();
    let mut pos: (i32, i32) = start;
    let mut direction: Direction = Direction::Up;
    while map.contains_key(&pos) {
        if map[&pos] {
            direction = match direction {
                Direction::Up => {
                    pos.1 += 1;
                    Direction::Right
                },
                Direction::Right => {
                    pos.0 -= 1;
                    Direction::Down
                },
                Direction::Down => {
                    pos.1 -= 1;
                    Direction::Left
                },
                Direction::Left => {
                    pos.0 += 1;
                    Direction::Up
                }
            }
        } else {
            let mut next: (i32, i32) = pos;
            match direction {
                Direction::Up => { next.1 -= 1 }
                Direction::Right => { next.0 += 1 }
                Direction::Down => { next.1 += 1 }
                Direction::Left => { next.0 -= 1 }
            }

            if map.contains_key(&next) && !map[&next] && !possibilities.contains(&next) {
                map.insert(next, true);
                if simulate(&map, start, Direction::Up) {
                    // println!("pos: {:?}, next: {:?}, dir: {:?}\n", pos, next, direction);
                    possibilities.push(next);
                }
                map.insert(next, false);
            }

            pos = next;
        }
    }

    possibilities.len()
}