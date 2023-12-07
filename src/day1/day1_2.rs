use std::fs;

pub fn day1_2() {
    let input: String = fs::read_to_string("src/day1/input.txt")
        .expect("Should have been able to read the file");

    let map: Vec<(&str, &str)> = Vec::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "th3ee"),
        ("four", "fo4r"),
        ("five", "fi5e"),
        ("six", "s6x"),
        ("seven", "se7en"),
        ("eight", "ei8ht"),
        ("nine", "ni9e"),
    ]);

    let mut lines: Vec<String> = input.split("\r\n").map(String::from).collect();

    for line in lines.iter_mut() {
        for (digit, replacement) in &map {
            while line.contains(digit) {
                let index: usize = line.find(digit).unwrap();
                line.replace_range(index..(index + digit.len()), replacement);
            }
        }
    }

    let digit_lines: Vec<String> = lines
        .iter()
        .map(|x| x.chars().filter(|y| y.is_numeric()).collect())
        .collect();

    let result: u32 = digit_lines
        .iter()
        .map(|x: &String| {
            if let (Some(first_digit), Some(last_digit)) = (x.chars().nth(0), x.chars().last()) {
                if let (Some(first), Some(last)) = (first_digit.to_digit(10), last_digit.to_digit(10)) {
                    return first * 10 + last;
                }
            }
            return 0;
        })
        .sum();

    println!("{}", result);
}