use std::{collections::HashMap, fs};

fn main() {
    println!("Hello, world!");
    solve_part1();
    solve_part2();
}

fn solve_part1() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut lines = file.lines();
    let mut points: u32 = 0;
    while let Some(line) = lines.next() {
        let mut parts = line.split("|");

        let winning_numbers = parts
            .next()
            .unwrap()
            .split(":")
            .last()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();

        let card_numbers = parts
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();

        let count = winning_numbers
            .iter()
            .filter(|&x| card_numbers.contains(x))
            .count();

        if count > 0 {
            points += 2u32.pow(count as u32 - 1);
        }
    }
    println!("Totalt points: {}", points);
}

fn solve_part2() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut lines = file.lines().enumerate();
    let number_of_lines = lines.clone().count();
    let mut card_copies: HashMap<i32, i32> = HashMap::new();

    while let Some((number, line)) = lines.next() {
        let mut parts = line.split("|");
        let card_number: i32 = number as i32 + 1;

        let winning_numbers = parts
            .next()
            .unwrap()
            .split(":")
            .last()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();

        let card_numbers = parts
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();

        let count = winning_numbers
            .iter()
            .filter(|&x| card_numbers.contains(x))
            .count();

        let how_many_am_i: i32 = card_copies.get(&card_number).unwrap_or(&0).clone() + 1;

        for i in 1..count + 1 {
            let won_card_number = card_number + i as i32;
            *card_copies.entry(won_card_number).or_insert(0) += how_many_am_i;
        }
    }

    let total_copies: i32 = card_copies.values().sum();
    println!("Totalt cards: {}", total_copies + number_of_lines as i32);
}
