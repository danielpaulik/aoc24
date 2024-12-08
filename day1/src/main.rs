use std::{collections::HashMap, iter::zip};

fn input() -> &'static str {
    include_str!("../inputs/1.txt")
}

fn day1_1() {
    let (mut left_elements, mut right_elements): (Vec<_>, Vec<_>) = input().lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap()
            )
        })
        .unzip();

    left_elements.sort_unstable();
    right_elements.sort_unstable();

    let sum: i32 = zip(left_elements, right_elements)
        .map(|(left, right)| { (left - right).abs() })
        .sum();

    println!("Answer to puzzle 1: {}", sum);
}

fn day1_2() {
    let (left_elements, right_elements): (Vec<_>, Vec<_>) = input().lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap()
            )
        })
        .unzip();

    let counts = right_elements.iter()
        .fold(HashMap::new(), |mut accumulator, &element| {
            *accumulator.entry(element).or_insert(0) += 1;
            accumulator
        });

    let sum: i32 = left_elements.iter()
        .map(|element| { counts.get(element).unwrap_or(&0) * element })
        .sum();

    println!("Answer to puzzle 2: {}", sum);
}

fn main() {
    day1_1();
    day1_2();
}
