mod day_1;

fn main() {
    let input = std::fs::read_to_string("src/day_1/input.txt").unwrap();
    println!("{} {}", day_1::part_1(input.clone()), day_1::part_2(input));
}
