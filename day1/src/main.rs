
fn main() {
    let test_input: &str =
        "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

    let input: &str = include_str!("input.txt");

    // println!("Hello, world: {input}");
    let mut sum = 0;
    for line in input.lines() {
        println!("Processing line: {}", line);

        // get first digit
        let index_first = line.find(char::is_numeric).unwrap();
        let first = line.chars().nth(index_first).unwrap();

        // get last digit
        let index_last = line.rfind(char::is_numeric).unwrap();
        let last = line.chars().nth(index_last).unwrap();

        // combine them into a number
        let number = format!("{first}{last}");

        // add to sum
        sum += number.parse::<i32>().unwrap();
    }

    println!("Sum is {sum}");
}
