/* old solutions that didn't work but I'm proud of 🥲

fn count_horizontal(input: &str) -> u32 {
    let mut total: u32 = 0;

    let mut i: usize = 0;
    while i < (input.len() - 4) {
        if &input[i..(i + 4)] == "XMAS" || &input[i..(i + 4)] == "SAMX" {
            total += 1;
            i += 4;
        } else {
            i += 1;
        }
    }

    total
}

fn flip_rotation(input: &str) -> String {
    let width: usize = input.lines().next().unwrap().len();
    let height: usize = input.len() / width;

    input.chars()
        .enumerate()
        .fold(vec![' '; (width + 1) * height], |mut acc, (i, c)| {
            let i = if c == '\n' { i } else {
                let y = i / (width + 1);
                let x = i % (width + 1);
                x * (width + 1) + y // switch x and y
            };

            acc[i] = c;
            acc
        })
        .into_iter()
        .fold("".to_string(), |mut acc, x| {
            acc.push(x);
            acc
        })
}

fn count_vertical(input: &str) -> u32 {
    count_horizontal(&flip_rotation(input))
}

fn count_diagonal(input: &str) -> u32 {
    let width: usize = input.lines().next().unwrap().len();
    let height: usize = input.len() / width;
    let input = input.chars().filter(|c| *c != '\n').collect::<Vec<char>>();
    let mut total: u32 = 0;

    let mut x: usize = 0;
    let mut y: usize = 0;
    while y <= height - 4 {
        while x < width {
            if x <= width - 4 {
                let mut diagonal: String = String::new();
                diagonal.push(input[(y + 0) * width + (x + 0)]);
                diagonal.push(input[(y + 1) * width + (x + 1)]);
                diagonal.push(input[(y + 2) * width + (x + 2)]);
                diagonal.push(input[(y + 3) * width + (x + 3)]);

                if diagonal == "XMAS" || diagonal == "SAMX" {
                    total += 1;
                }
            }

            if x >= 3 {
                let mut diagonal: String = String::new();
                diagonal.push(input[(y + 0) * width + (x - 0)]);
                diagonal.push(input[(y + 1) * width + (x - 1)]);
                diagonal.push(input[(y + 2) * width + (x - 2)]);
                diagonal.push(input[(y + 3) * width + (x - 3)]);

                if diagonal == "XMAS" || diagonal == "SAMX" {
                    total += 1;
                }
            }

            x += 1;
        }

        x = 0;
        y += 1;
    }

    total
}
*/

pub fn part1(input: &str) -> u32 {
    let width: usize = input.lines().next().unwrap().len();
    let height: usize = input.len() / width;
    let input = input.chars().filter(|c| *c != '\n').collect::<Vec<char>>();
    let mut total: u32 = 0;

    for y in 0..height {
        for x in 0..width {
            if x <= width - 4 {
                let mut horizontal: String = String::new();
                horizontal.push(input[y * width + (x + 0)]);
                horizontal.push(input[y * width + (x + 1)]);
                horizontal.push(input[y * width + (x + 2)]);
                horizontal.push(input[y * width + (x + 3)]);

                if horizontal == "XMAS" || horizontal == "SAMX" {
                    total += 1;
                }
            }

            if y <= height - 4 {
                let mut vertical: String = String::new();
                vertical.push(input[(y + 0) * width + x]);
                vertical.push(input[(y + 1) * width + x]);
                vertical.push(input[(y + 2) * width + x]);
                vertical.push(input[(y + 3) * width + x]);

                if vertical == "XMAS" || vertical == "SAMX" {
                    total += 1;
                }
            }

            if x <= width - 4 && y <= height - 4 {
                let mut diagonal: String = String::new();
                diagonal.push(input[(y + 0) * width + (x + 0)]);
                diagonal.push(input[(y + 1) * width + (x + 1)]);
                diagonal.push(input[(y + 2) * width + (x + 2)]);
                diagonal.push(input[(y + 3) * width + (x + 3)]);

                if diagonal == "XMAS" || diagonal == "SAMX" {
                    total += 1;
                }
            }

            if x >= 3 && y <= height - 4 {
                let mut diagonal: String = String::new();
                diagonal.push(input[(y + 0) * width + (x - 0)]);
                diagonal.push(input[(y + 1) * width + (x - 1)]);
                diagonal.push(input[(y + 2) * width + (x - 2)]);
                diagonal.push(input[(y + 3) * width + (x - 3)]);

                if diagonal == "XMAS" || diagonal == "SAMX" {
                    total += 1;
                }
            }
        }
    }

    total
}

pub fn part2(input: &str) -> u32 {
    let width: usize = input.lines().next().unwrap().len();
    let height: usize = input.len() / width;
    let input = input.chars().filter(|c| *c != '\n').collect::<Vec<char>>();
    let mut total: u32 = 0;

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            if input[y * width + x] != 'A' {
                continue;
            }

            let tl = input[(y - 1) * width + (x - 1)];
            let tr = input[(y - 1) * width + (x + 1)];
            let bl = input[(y + 1) * width + (x - 1)];
            let br = input[(y + 1) * width + (x + 1)];

            if [(tl, tr, br, bl), (tr, br, bl, tl), (br, bl, tl, tr), (bl, tl, tr, br)].contains(&('M', 'M', 'S', 'S')) {
                total += 1;
            }
        }
    }

    total
}