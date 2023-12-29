use std::cmp::Ordering;

fn main() {
    let input = include_str!("test_input.txt");

    let mut hands: Vec<Hand> = Vec::new();
    for line in input.lines() {
        let mut parts = line.split(" ");
        let cards = parts.next().unwrap().to_string();
        let bid = parts.next().unwrap().parse::<i32>().unwrap();
        hands.push(Hand::new(cards, bid))
    }

    println!("Hello, world! {:?}", hands);

    part1(hands);
}

fn part1(mut hands: Vec<Hand>) {
    let total_ranks = hands.len();

    let mut five_of_a_kind: Vec<Hand> = Vec::new();
    let mut four_of_a_kind: Vec<Hand> = Vec::new();
    let mut full_house: Vec<Hand> = Vec::new();
    let mut three_of_a_kind: Vec<Hand> = Vec::new();
    let mut two_pair: Vec<Hand> = Vec::new();
    let mut one_pair: Vec<Hand> = Vec::new();
    let mut high_card: Vec<Hand> = Vec::new();
    while let Some(hand) = hands.pop() {
        println!("Processing {:?}", hand);
        let existing: Vec<i32> = hand.values.iter().filter(|&x| *x > 0).map(|x| *x).collect();
        if existing.contains(&5) {
            five_of_a_kind.push(hand);
            continue;
        }
        if existing.contains(&4) {
            four_of_a_kind.push(hand);
            continue;
        }
        if existing.contains(&3) {
            if existing.contains(&2) {
                full_house.push(hand)
            } else {
                three_of_a_kind.push(hand)
            }
            continue;
        }
        let pairs: Vec<&i32> = existing.iter().filter(|&x| *x == 2).collect();
        if pairs.len() == 2 {
            two_pair.push(hand);
            continue;
        }
        if pairs.len() == 1 {
            one_pair.push(hand);
            continue;
        }
        high_card.push(hand);
    }
    // println!("five_of_a_kind: {:?}", five_of_a_kind);
    // println!("four_of_a_kind: {:?}", four_of_a_kind);
    // println!("full_house: {:?}", full_house);
    // println!("three_of_a_kind: {:?}", three_of_a_kind);
    // println!("two_pair: {:?}", two_pair);
    // println!("one_pair: {:?}", one_pair);
    // println!("high_card: {:?}", high_card);

    five_of_a_kind.sort_by(hand_ordering);
    four_of_a_kind.sort_by(hand_ordering);
    full_house.sort_by(hand_ordering);
    three_of_a_kind.sort_by(hand_ordering);
    two_pair.sort_by(hand_ordering);
    one_pair.sort_by(hand_ordering);
    high_card.sort_by(hand_ordering);

    // println!("five_of_a_kind: {:?}", five_of_a_kind);
    // println!("four_of_a_kind: {:?}", four_of_a_kind);
    // println!("full_house: {:?}", full_house);
    // println!("three_of_a_kind: {:?}", three_of_a_kind);
    // println!("two_pair: {:?}", two_pair);
    // println!("one_pair: {:?}", one_pair);
    // println!("high_card: {:?}", high_card);

    let mut sum = 0;
    let mut current_rank = 1;
    for hand in high_card {
        sum += hand.bid * current_rank;
        current_rank += 1;
    }
    for hand in one_pair {
        sum += hand.bid * current_rank;
        current_rank += 1;
    }
    for hand in two_pair {
        sum += hand.bid * current_rank;
        current_rank += 1;
    }
    for hand in three_of_a_kind {
        sum += hand.bid * current_rank;
        current_rank += 1;
    }
    for hand in full_house {
        sum += hand.bid * current_rank;
        current_rank += 1;
    }
    for hand in four_of_a_kind {
        sum += hand.bid * current_rank;
        current_rank += 1;
    }
    for hand in five_of_a_kind {
        sum += hand.bid * current_rank;
        current_rank += 1;
    }
    println!("sum is {}", sum)
}

fn hand_ordering(a: &Hand, b: &Hand) -> Ordering {
    for i in 0..5 {
        let a_char = a.card_chars.get(i).unwrap();
        let b_char = b.card_chars.get(i).unwrap();
        if a_char != b_char {
            return card_index_of(*a_char).cmp(&card_index_of(*b_char));
        }
    }
    Ordering::Equal
}

#[derive(Debug)]
struct Hand {
    cards: String,
    values: Vec<i32>,
    bid: i32,
    card_chars: Vec<char>,
}

impl Hand {
    fn new(cards: String, bid: i32) -> Hand {
        let mut values: Vec<i32> = vec![0; 13];
        let mut card_chars: Vec<char> = Vec::new();
        for c in cards.chars() {
            // println!("index of {} is {}", c, card_index_of(c));
            values[card_index_of(c)] += 1;
            card_chars.push(c);
        }
        Hand {
            cards,
            values,
            bid,
            card_chars,
        }
    }
}

// A hand consists of five cards labeled one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2.
// The relative strength of each card follows this order, where A is the highest and 2 is the lowest.
fn card_index_of(c: char) -> usize {
    if c.is_ascii_digit() {
        return c as usize - 50;
    }
    if c == 'T' {
        return 8;
    }
    if c == 'J' {
        return 9;
    }
    if c == 'Q' {
        return 10;
    }
    if c == 'K' {
        return 11;
    }
    if c == 'A' {
        return 12;
    }
    panic!("{} not recognized", c)
}
