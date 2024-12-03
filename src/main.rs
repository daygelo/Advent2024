mod day2;

fn main() {
    let input = std::fs::read_to_string("src/day2/input.txt").unwrap();
    println!("{} {}", day2::part1(&input), day2::part2(&input));
}
