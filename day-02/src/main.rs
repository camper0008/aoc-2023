fn first() {
    let input = include_str!("input.txt");
    let output = input
        .lines()
        .filter_map(|v| {
            let sets: Vec<_> = v.split(":").collect();
            let game_id = sets[0].split_whitespace().skip(1).collect::<Vec<_>>()[0];
            let game_id: usize = game_id.parse().unwrap();
            let sets = sets[1];
            let sets = sets.split(";");
            for set in sets {
                let items = set.split(",").map(|v| v.trim());
                for item in items {
                    let count_and_color: Vec<_> =
                        item.split_whitespace().filter(|v| !v.is_empty()).collect();
                    let count = count_and_color[0];
                    let count: usize = count.parse().unwrap();
                    let color = count_and_color[1];
                    match color {
                        "red" => {
                            if count > 12 {
                                return None;
                            }
                        }
                        "green" => {
                            if count > 13 {
                                return None;
                            }
                        }
                        "blue" => {
                            if count > 14 {
                                return None;
                            }
                        }
                        color => unreachable!("unexpected color {color}"),
                    }
                }
            }
            Some(game_id)
        })
        .sum::<usize>();
    println!("{output}");
}

#[derive(Default, Clone)]
struct Count {
    red: usize,
    green: usize,
    blue: usize,
}

impl Count {
    fn set_red(&mut self, value: usize) {
        if value > self.red {
            self.red = value;
        }
    }
    fn set_green(&mut self, value: usize) {
        if value > self.green {
            self.green = value;
        }
    }
    fn set_blue(&mut self, value: usize) {
        if value > self.blue {
            self.blue = value;
        }
    }
    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

fn second() {
    let input = include_str!("input.txt");
    let mut games = vec![Count::default(); input.lines().count()];
    let output = input
        .lines()
        .map(|v| {
            let sets: Vec<_> = v.split(":").collect();
            let game_id = sets[0].split_whitespace().skip(1).collect::<Vec<_>>()[0];
            let game_id: usize = game_id.parse().unwrap();
            let game_id = game_id - 1;
            let sets = sets[1];
            let sets = sets.split(";");
            for set in sets {
                let items = set.split(",").map(|v| v.trim());
                for item in items {
                    let count_and_color: Vec<_> =
                        item.split_whitespace().filter(|v| !v.is_empty()).collect();
                    let count = count_and_color[0];
                    let count: usize = count.parse().unwrap();
                    let color = count_and_color[1];
                    match color {
                        "red" => {
                            games[game_id].set_red(count);
                        }
                        "green" => {
                            games[game_id].set_green(count);
                        }
                        "blue" => {
                            games[game_id].set_blue(count);
                        }
                        color => unreachable!("unexpected color {color}"),
                    }
                }
            }
            games[game_id].power()
        })
        .sum::<usize>();
    println!("{output}");
}

fn main() {
    first();
    second();
}
