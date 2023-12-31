use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let test_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let mut cards: Vec<Card> = Vec::new();
    for line in input.lines() {
        let mut parts = line.split(':');
        // let title = parts.nth(0).unwrap();
        // title.r
        // println!("parsing {line}");
        // println!("parsing now {:?}", parts.nth(0).unwrap().split(" ").filter(|x| !x.is_empty()).nth(1));
        let index: i32 = parts
            .next()
            .unwrap()
            .split(' ')
            .filter(|x| !x.is_empty())
            .nth(1)
            .unwrap()
            .trim()
            .parse()
            .unwrap();

        let mut inner_parts = parts.nth(0).unwrap().split("|");
        let mut winning_numbers: Vec<i32> = Vec::new();
        for i in inner_parts.nth(0).unwrap().split(" ") {
            if i.trim() == "" {
                continue;
            }
            let n: i32 = i.trim().parse().unwrap();
            winning_numbers.push(n);
        }
        let mut my_numbers: Vec<i32> = Vec::new();
        for i in inner_parts.nth(0).unwrap().split(" ") {
            if i.trim() == "" {
                continue;
            }
            let n: i32 = i.trim().parse().unwrap();
            my_numbers.push(n);
        }
        cards.push(Card {
            index,
            winning_numbers,
            my_numbers,
        });
    }

    let mut sum = 0;
    for c in &cards {
        sum += c.calculate_points();
    }

    println!("sum is {sum}");

    let mut amounts = HashMap::new();
    for c in &cards {
        amounts.insert(c.index, 1);
    }

    for card in &cards {
        for _ in 0..*amounts.get(&card.index).unwrap() {
            let matches = card.calculate_matching_numbers();
            // println!("card {} has {} matches", card.index, matches);
            for i in card.index + 1..card.index + 1 + matches {
                amounts.entry(i).and_modify(|amount| *amount += 1);
            }
        }
    }

    println!("total: {}", amounts.values().sum::<i32>());
}

struct Card {
    index: i32,
    winning_numbers: Vec<i32>,
    my_numbers: Vec<i32>,
}

impl Card {
    fn calculate_points(&self) -> i32 {
        println!(
            "calculating points for winning_nums {:?} with my_nums {:?}",
            &self.winning_numbers, &self.my_numbers
        );
        let mut count = 0;
        for n in &self.my_numbers {
            if self.winning_numbers.contains(&n) {
                count += 1;
            }
        }
        if count == 0 {
            return 0;
        }

        let two: i32 = 2;
        two.pow(count - 1)
    }

    fn calculate_matching_numbers(&self) -> i32 {
        // println!(
        //     "calculating points for winning_nums {:?} with my_nums {:?}",
        //     &self.winning_numbers, &self.my_numbers
        // );
        let mut count = 0;
        for n in &self.my_numbers {
            if self.winning_numbers.contains(&n) {
                count += 1;
            }
        }
        count
    }
}
