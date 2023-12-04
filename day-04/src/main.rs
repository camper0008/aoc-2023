fn first() {
    let input = include_str!("input.txt");
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
    println!("first = {output}");
}

fn parse_line(lines: &[&str], idx: usize) -> usize {
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

    let matching = matching_numbers
        + (idx + 1..=idx + matching_numbers)
            .map(|v| parse_line(lines, v))
            .sum::<usize>();

    matching
}

fn second() {
    let input = include_str!("input.txt");
    let lines: Vec<&str> = input.lines().collect();
    let output = input
        .lines()
        .enumerate()
        .map(|(i, _)| parse_line(&lines, i))
        .sum::<usize>()
        + lines.len();
    println!("second = {output}");
}

fn main() {
    first();
    second();
}
