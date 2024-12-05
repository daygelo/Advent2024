mod day4;

fn main() {
    let input = std::fs::read_to_string("src/day4/input.txt").unwrap();
    println!("{} {}", day4::part1(&input), day4::part2(&input));
}
