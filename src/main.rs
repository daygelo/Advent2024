mod day8;

fn main() {
    let input = std::fs::read_to_string("src/day8/input.txt").unwrap();
    println!("{} {}", day8::part1(&input), day8::part2(&input));
}
