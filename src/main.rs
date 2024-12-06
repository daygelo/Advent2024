mod day5;

fn main() {
    let input = std::fs::read_to_string("src/day5/input.txt").unwrap();
    println!("{} {}", day5::part1(&input), day5::part2(&input));
}
