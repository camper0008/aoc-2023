use std::time::Instant;

fn first() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    let output: usize = input
        .lines()
        .map(|v| {
            let mut numbers: Vec<usize> = Vec::new();
            let mut existing_numbers = Vec::new();
            let mut split = v.split_whitespace();
            let _card = split.next();
            let _card_index = split.next();
            loop {
                let next = split.next().unwrap();
                if next == "|" {
                    break;
                };
                let next = next.parse().unwrap();
                numbers.push(next);
            }

            loop {
                let Some(next) = split.next() else {
                    break;
                };
                let next = next.parse().unwrap();
                if numbers.contains(&next) {
                    existing_numbers.push(next);
                };
            }
            if existing_numbers.len() > 0 {
                2_usize.pow(existing_numbers.len() as u32 - 1u32)
            } else {
                0
            }
        })
        .sum();

    let now = Instant::now() - now;
    println!("  first = {output} in {}ms", now.as_millis());
}

fn parse_line_uncached(lines: &[&str], idx: usize) -> usize {
    let line = lines[idx];
    let matching_numbers = {
        let mut numbers: Vec<usize> = Vec::new();
        let mut existing_numbers = Vec::new();
        let mut split = line.split_whitespace();
        let _card = split.next();
        let _card_index = split.next();
        loop {
            let next = split.next().unwrap();
            if next == "|" {
                break;
            };
            let next = next.parse().unwrap();
            numbers.push(next);
        }

        loop {
            let Some(next) = split.next() else {
                break;
            };
            let next = next.parse().unwrap();
            if numbers.contains(&next) {
                existing_numbers.push(next);
            };
        }
        existing_numbers.len()
    };

    let tickets_won = matching_numbers
        + (idx + 1..=idx + matching_numbers)
            .map(|v| parse_line_uncached(lines, v))
            .sum::<usize>();

    tickets_won
}

fn second_uncached() {
    let now = Instant::now();
    let input = include_str!("input.txt");
    let lines: Vec<&str> = input.lines().collect();
    let output = input
        .lines()
        .enumerate()
        .map(|(i, _)| parse_line_uncached(&lines, i))
        .sum::<usize>()
        + lines.len();
    let now = Instant::now() - now;
    println!("  second = {output} in {}ms", now.as_millis());
}

fn parse_line_cached(lines: &[&str], idx: usize, cache: &mut Vec<Option<usize>>) -> usize {
    if let Some(v) = cache[idx] {
        return v;
    };
    let line = lines[idx];
    let matching_numbers = {
        let mut numbers: Vec<usize> = Vec::new();
        let mut existing_numbers = Vec::new();
        let mut split = line.split_whitespace();
        let _card = split.next();
        let _card_index = split.next();
        loop {
            let next = split.next().unwrap();
            if next == "|" {
                break;
            };
            let next = next.parse().unwrap();
            numbers.push(next);
        }

        loop {
            let Some(next) = split.next() else {
                break;
            };
            let next = next.parse().unwrap();
            if numbers.contains(&next) {
                existing_numbers.push(next);
            };
        }
        existing_numbers.len()
    };

    let tickets_won = matching_numbers
        + (idx + 1..=idx + matching_numbers)
            .map(|v| parse_line_cached(lines, v, cache))
            .sum::<usize>();
    cache[idx] = Some(tickets_won);
    tickets_won
}

fn second_cached() {
    let now = Instant::now();
    let input = include_str!("input.txt");
    let lines: Vec<&str> = input.lines().collect();
    let mut cache = vec![None; lines.len()];
    let output = input
        .lines()
        .enumerate()
        .map(|(i, _)| parse_line_cached(&lines, i, &mut cache))
        .sum::<usize>()
        + lines.len();
    let now = Instant::now() - now;
    println!("  second = {output} in {}ms", now.as_millis());
}

fn main() {
    println!("calculating first with cache");
    first();
    println!("calculating second with cache");
    second_cached();
    println!("calculating second without cache (should probably be run with --release)");
    second_uncached();
}
