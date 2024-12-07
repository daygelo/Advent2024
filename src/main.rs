mod day6;

fn main() {
    let input = std::fs::read_to_string("src/day6/input.txt").unwrap();
    println!("{} {}", day6::part1(&input), day6::part2(&input));
}
