mod day1;

fn main() {
    let input = std::fs::read_to_string("src/day1/input.txt").unwrap();
    println!("{} {}", day1::part1(&input), day1::part2(&input));
}
