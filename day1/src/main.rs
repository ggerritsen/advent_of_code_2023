
fn main() {
    let digits = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let input_test: &str =
        "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

    let input: &str = include_str!("input.txt");

    // println!("Hello, world: {input}");
    let mut sum = 0;
    for line in input.lines() {
        println!("Processing line: {}", line);

        // get first digit
        let mut index_first = line.find(char::is_numeric).or(Some(999999999)).unwrap();
        let mut first = line.chars().nth(index_first).or(Some('0')).unwrap().to_string();
        for count in 0..9 {
            let digit = digits[count];
            let index = line.find(digit).or(Some(999999999)).unwrap();
            if index < index_first {
                index_first = index;
                first = (count + 1).to_string();
            }
        }
        println!("First digit is: {}", first);

        // get last digit
        let mut index_last = line.rfind(char::is_numeric).or(Some(0)).unwrap();
        let mut last = line.chars().nth(index_last).unwrap_or('0').to_string();
        for count in 0..9 {
            let digit = digits[count];
            let index = line.rfind(digit).or(Some(0)).unwrap();
            if index > index_last {
                index_last = index;
                last = (count + 1).to_string();
            }
        }
        println!("Last digit is: {}", first);

        // combine them into a number
        let number = format!("{first}{last}");

        // add to sum
        sum += number.parse::<i32>().unwrap();
    }

    println!("Sum is {sum}");
}
