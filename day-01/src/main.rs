fn first() {
    let input = include_str!("input.txt");
    let output: usize = input
        .lines()
        .map(|v| -> usize {
            let first = v.chars().find(char::is_ascii_digit).unwrap();
            let last = v.chars().rev().find(char::is_ascii_digit).unwrap();
            (first.to_string() + &last.to_string())
                .parse::<usize>()
                .unwrap()
        })
        .sum();
    println!("first: {output}");
}

fn second() {
    let input = include_str!("input.txt");
    let output: usize = input
        .lines()
        .map(|chars| -> usize {
            let mut first: Option<String> = None;
            let mut last: Option<String> = None;
            let mut idx = 0;
            loop {
                match &chars.get(idx..=idx) {
                Some(v) if v.chars().last().unwrap().is_ascii_digit() => {
                    if first.is_none() {
                        first = Some((*v).to_string());
                    } else {
                        last = Some((*v).to_string());
                    };
                    idx += 1;
                }
                Some("o") /* one */ => {
                    let m = "one";
                    let v = Some("1".to_string());
                    if chars.get(idx..idx+m.len()) == Some(m) {
                        if first.is_none() {
                            first = v;
                        } else {
                            last = v;
                        };
                        idx += m.len() - 1;
                    } else {
                        idx += 1;
                    }
                }
                Some("t") /* two three */ => {
                    let m = "two";
                    let v = Some("2".to_string());
                    if chars.get(idx..idx+m.len()) == Some(m) {
                        if first.is_none() {
                            first = v;
                        } else {
                            last = v;
                        };
                        idx += m.len() - 1;
                        continue;
                    }
                    let m = "three";
                    let v = Some("3".to_string());
                    if chars.get(idx..idx+m.len()) == Some(m) {
                        if first.is_none() {
                            first = v;
                        } else {
                            last = v;
                        };
                        idx += m.len() - 1;
                        continue;
                    }
                    idx += 1;
                }
                Some("f") /* four five */ => {
                    let m = "four";
                    let v = Some("4".to_string());
                    if chars.get(idx..idx+m.len()) == Some(m) {
                        if first.is_none() {
                            first = v;
                        } else {
                            last = v;
                        };
                        idx += m.len() - 1;
                        continue;
                    }
                    let m = "five";
                    let v = Some("5".to_string());
                    if chars.get(idx..idx+m.len()) == Some(m) {
                        if first.is_none() {
                            first = v;
                        } else {
                            last = v;
                        };
                        idx += m.len() - 1;
                        continue;
                    }
                    idx += 1;

                }
                Some("s") /* six seven */ => {
                    let m = "six";
                    let v = Some("6".to_string());
                    if chars.get(idx..idx+m.len()) == Some(m) {
                        if first.is_none() {
                            first = v;
                        } else {
                            last = v;
                        };
                        idx += m.len() - 1;
                        continue;
                    }
                    let m = "seven";
                    let v = Some("7".to_string());
                    if chars.get(idx..idx+m.len()) == Some(m) {
                        if first.is_none() {
                            first = v;
                        } else {
                            last = v;
                        };
                        idx += m.len() - 1;
                        continue;
                    }
                    idx += 1;
                }
                Some("e") /* eight */ => {
                    let m = "eight";
                    let v = Some("8".to_string());
                    if chars.get(idx..idx+m.len()) == Some(m) {
                        if first.is_none() {
                            first = v;
                        } else {
                            last = v;
                        };
                        idx += m.len() - 1;
                    } else {
                        idx += 1;
                    }

                }
                Some("n") /* nine */ => {
                    let m = "nine";
                    let v = Some("9".to_string());
                    if chars.get(idx..idx+m.len()) == Some(m) {
                        if first.is_none() {
                            first = v;
                        } else {
                            last = v;
                        };
                        idx += m.len() - 1;
                    } else {
                        idx += 1;
                    }
                }
                Some(_) => idx += 1,
                None => break,
            };
            }
            let first = first.unwrap();
            let last = last.unwrap_or(first.clone());
            (first + &last).parse().unwrap()
        })
        .sum();
    println!("second: {output}");
}

fn main() {
    first();
    second();
}
