mod day9;

fn main() {
    let input = std::fs::read_to_string("src/day9/input.txt").unwrap();
    println!("{} {}", day9::part1(&input), day9::part2(&input));
}
