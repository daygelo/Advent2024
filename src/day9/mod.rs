pub fn part1(input: &str) -> usize {
    let mut files: Vec<i32> = Vec::new();

    let mut i: usize = 0;
    for ch in input.chars() {
        for _ in 0..ch.to_digit(10).unwrap() {
            files.push(
                if i % 2 == 1 {
                    -1
                } else {
                    (i / 2) as i32
                }
            )
        }
        i += 1;
    }

    files.reverse();

    let mut i: usize = 0;
    let mut empty_index: usize = files.len() - 1;
    for n in files.clone() {
        if n >= 0 {
            while empty_index > i && files[empty_index] >= 0 {
                empty_index -= 1;
            }
            if files[empty_index] < 0 {
                files[empty_index] = n;
                files.remove(i);
            }
        } else {
            i += 1;
        }
    }

    files.reverse();
    files.iter()
        .filter(|n| **n >= 0)
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (i * *x as usize))
}

pub fn part2(input: &str) -> usize {
    let mut files: Vec<(i32, u32)> = Vec::new();

    let mut i: usize = 0;
    for ch in input.chars() {
        files.push((
            if i % 2 == 1 {
                -1
            } else {
                (i / 2) as i32
            },
            ch.to_digit(10).unwrap()
        ));
        i += 1;
    }

    files.reverse();

    // let mut i: usize = 0;
    // let mut empty_index: usize = files.len() - 1;
    // for (n, len) in files.clone() {
    //     if n >= 0 {
    //         while empty_index > i && files[empty_index].0 >= 0 {
    //             empty_index -= 1;
    //         }
    //         if files[empty_index] < 0 {
    //             files[empty_index] = n;
    //             files.remove(i);
    //         }
    //     } else {
    //         i += 1;
    //     }
    // }

    0
}