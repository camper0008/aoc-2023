use std::error::Error;

fn is_surrounded_by_symbol(chars: &Vec<Vec<char>>, x_idx: usize, y_idx: usize) -> bool {
    let offsets: [(isize, isize); 8] = [
        (-1, -1),
        (0_, -1),
        (1_, -1),
        (-1, 0_),
        (1_, 0_),
        (-1, 1_),
        (0_, 1_),
        (1_, 1_),
    ];
    for (x, y) in offsets {
        if x_idx as isize + x < 0 || y_idx as isize + y < 0 {
            continue;
        }
        if x_idx as isize + x >= chars[y_idx].len() as isize
            || y_idx as isize + y >= chars.len() as isize
        {
            continue;
        }
        let x = (x_idx as isize + x) as usize;
        let y = (y_idx as isize + y) as usize;
        match chars[y][x] {
            '0'..='9' | '.' => {}
            '*' | '#' | '$' | '&' | '=' | '+' | '-' | '%' | '@' | '/' => return true,
            ch => unreachable!("unhandled char {ch}"),
        }
    }
    false
}

fn take_number(chars: &Vec<Vec<char>>, x_idx: &mut usize, y_idx: usize) -> (bool, usize) {
    let mut number = String::new();
    let mut has_number = false;
    loop {
        if *x_idx == chars[y_idx].len() {
            break;
        }
        if !has_number && is_surrounded_by_symbol(chars, *x_idx, y_idx) {
            has_number = true;
        }
        match chars[y_idx][*x_idx] {
            c @ '0'..='9' => {
                number.push(c);
            }
            '*' | '#' | '$' | '&' | '=' | '+' | '-' | '%' | '@' | '/' | '.' => break,
            ch => unreachable!("unhandled char {ch}"),
        }
        *x_idx += 1;
    }
    (has_number, number.parse().unwrap())
}

fn first() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.txt");
    let map = input
        .lines()
        .map(|v| v.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;
    let mut x_idx = 0;
    let mut y_idx = 0;

    loop {
        if x_idx >= map[y_idx].len() {
            x_idx = 0;
            y_idx += 1;
        }
        if y_idx >= map.len() {
            break;
        }

        match map[y_idx][x_idx] {
            '0'..='9' => {
                let (has_symbol, number) = take_number(&map, &mut x_idx, y_idx);
                if has_symbol {
                    println!("{number}");
                    count += number;
                }
            }
            '.' | '*' | '#' | '$' | '&' | '=' | '+' | '-' | '%' | '@' | '/' => {
                x_idx += 1;
                continue;
            }
            ch => unreachable!("unhandled char {ch}"),
        }

        x_idx += 1;
    }

    println!("first: {count}");

    Ok(())
}

#[allow(unused)]
fn second() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.txt");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    first()?;
    second()?;
    Ok(())
}
