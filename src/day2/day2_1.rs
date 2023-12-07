use std::collections::HashMap;
use std::fs;
use itertools::Itertools;

pub fn day2_1() {
    let input: String = fs::read_to_string("src/day2/input.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<String> = input.split("\r\n").map(String::from).collect();

    let map: HashMap<&str, u32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    let games: Vec<(u32, Vec<Vec<(u32, String)>>)> = lines
        .iter()
        .map(|x: &String| {
            let (game_id, data): (&str, &str) = x.split(": ")
                .collect_tuple()
                .unwrap();
            let bag: Vec<Vec<(u32, String)>> = data
                .split("; ")
                .map(|x| x
                    .split(", ")
                    .map(|y| {
                        let mut iter = y.split_whitespace();
                        let amount: u32 = iter
                            .next()
                            .expect("Expected cube amount")
                            .parse()
                            .expect("Failed to parse cube amount");
                        let color: String = iter.collect();
                        return (amount, color);
                    })
                    .collect())
                .collect();
            let id = game_id
                .split("Game ")
                .nth(1)
                .expect("Failed to parse game id");
            return (id.parse::<u32>().unwrap(), bag);
        })
        .collect();

    let possible_games_id_sum: u32 = games
        .iter()
        .filter(|x| {
            let (_game_id, data) = x;
            return data
                .iter()
                .all(|y| y
                    .iter()
                    .all(|z| {
                        let (amount, color) = z;
                        return amount <= map.get(color.as_str()).unwrap();
                    }));
        })
        .map(|x| {
            let (game_id, _data) = x;
            return game_id;
        })
        .sum();

    println!("{}", possible_games_id_sum);
}