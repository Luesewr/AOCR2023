use std::fs;

pub fn day1_1() {
    let input: String = fs::read_to_string("src/day1/input.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = input.split("\r\n").collect();

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

    println!("{result}");
}