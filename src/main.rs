use aoc_3;
use std::fs;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let total_prior = aoc_3::process_part_1(&input);
    println!("part1: {}", total_prior);
}

fn part_2() {
    let input2 = fs::read_to_string("input2.txt").unwrap();
    let total_prior = aoc_3::proccess_part_2(&input2);
    println!("part2: {}", total_prior);
}
