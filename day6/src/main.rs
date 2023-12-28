use std::time::Instant;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut input = include_str!("input.txt").lines();

    let times: Vec<i64> = input
        .next()
        .unwrap()
        .replace("Time:", "")
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();

    let distances: Vec<i64> = input
        .next()
        .unwrap()
        .replace("Distance:", "")
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();

    let zip = times.iter().zip(distances.iter());
    let races: Vec<Race> = zip
        .map(|(time, distance)| Race {
            time: *time,
            distance: *distance,
        })
        .collect();

    println!("Hello, world! {:?}", races);

    // naive implementation
    let start = Instant::now();
    let mut results: Vec<i32> = Vec::new();
    for race in &races {
        let mut count = 0;
        for i in 1..race.time - 1 {
            let d = calculate_distance(i, race.time);
            if d > race.distance {
                count += 1;
            }
        }
        results.push(count);
    }

    println!("got results {:?} in {:?}", results, start.elapsed());
    println!(
        "multiplied this is {}",
        results.iter().fold(1, |acc, x| acc * x)
    );

    // optimized implementation
    let start2 = Instant::now();
    let mut results2: Vec<i64> = Vec::new();
    for race in &races {
        // println!("{:?}", race);
        let (x1, x2) = quadratic_solution(race);
        // println!("x1: {}, x2: {}", x1, x2);
        results2.push(x2 - x1 + 1)
    }
    println!("got results {:?} in {:?}", results2, start2.elapsed());
    println!(
        "multiplied this is {}",
        results2.iter().fold(1, |acc, x| acc * x)
    );
}

fn part2() {
    let mut input = include_str!("input.txt").lines();

    let times: i64 = input
        .next()
        .unwrap()
        .replace("Time:", "")
        .replace(" ", "")
        .parse::<i64>()
        .unwrap();

    let distances: i64 = input
        .next()
        .unwrap()
        .replace("Distance:", "")
        .replace(" ", "")
        .parse::<i64>()
        .unwrap();

    let races = vec![Race {
        time: times,
        distance: distances,
    }];

    println!("Hello, world! {:?}", races);

    // naive implementation
    let start = Instant::now();
    let mut results: Vec<i32> = Vec::new();
    for race in &races {
        let mut count = 0;
        for i in 1..race.time - 1 {
            let d = calculate_distance(i, race.time);
            if d > race.distance {
                count += 1;
            }
        }
        results.push(count);
    }

    println!("got results {:?} in {:?}", results, start.elapsed());
    println!(
        "multiplied this is {}",
        results.iter().fold(1, |acc, x| acc * x)
    );

    // optimized implementation
    let start2 = Instant::now();
    let mut results2: Vec<i64> = Vec::new();
    for race in &races {
        // println!("{:?}", race);
        let (x1, x2) = quadratic_solution(race);
        // println!("x1: {}, x2: {}", x1, x2);
        results2.push(x2 - x1 + 1)
    }
    println!("got results {:?} in {:?}", results2, start2.elapsed());
    println!(
        "multiplied this is {}",
        results2.iter().fold(1, |acc, x| acc * x)
    );
}

fn quadratic_solution(race: &Race) -> (i64, i64) {
    /**
        distance = (total_time - hold_time) * hold_time
        200 = (30 - x) * x
        200 = (30 + -x) * x
        0 = x * (-x + 30) - 200
        0 = -x2 + 30x - 200
        b2 - 4ac = 900 - 800 = 100
        x1,x2 = (-b +/- wortel(b2 - 4ac)) / 2a
        x1 = (-30 + wortel(100)) / -2 = (-30 + 10) / -2 = -20 / -2 = 10
        x2 = (-30 - wortel(100)) / -2 = (-30 - 10) / -2 = -40 / -2 = 20
        10 - 20
        See https://en.wikipedia.org/wiki/Quadratic_equation
    **/
    let total_time = race.time as f64;
    let distance = (race.distance + 1) as f64;

    let d = f64::sqrt(total_time * total_time - (4.0 * -1.0 * (0.0 - distance)));
    let x1 = ((0.0 - total_time) + d) / -2.0;
    let x2 = ((0.0 - total_time) - d) / -2.0;

    (x1.ceil() as i64, x2.floor() as i64)
}

/**
distance = (total_time - hold_time) * hold_time
9 = (7 - x) * x
9 = x2 - 7x
0 = x2 - 7x - 9
b2 -4ac
49 - 36 = 13 --> D --> 2 oplossingen
(-b +/- wortel(b2 - 4ac)) / 2a
(-7 +/- wortel(13)) / 2
{\displaystyle ax^{2}+bx+c=a(x-x_{1})(x-x_{2})},
1x2 + -7x + -9 = 1*(x-x1)(x-x2)
x1 * x2 == -9
x1 + x2 == -7

9 = (7 - x) * x
9 = (7 + -x) * x
0 = x * (-x + 7) - 9
0 = -x2 + 7x - 9
b2 - 4ac = 49 - 36 = 13
(-b +/- wortel(b2 - 4ac)) / 2a
(-7 +/- wortel(13)) / -2
x1,x2 = (-7 +/- wortel(13)) / -2
x1 = (-7 + 3,6) / -2 = -3,4/-2 = 1,7
x2 = (-7 - 3,6) / -2 = -10,6/-2 = 5,3
(x+1,7)(x+5,3) = 0
x = 1,7
x = 5,3
2 - 5

distance = (total_time - hold_time) * hold_time
40 = (15 - x) * x
40 = (15 + -x) * x
0 = x * (-x + 15) - 40
0 = -x2 + 15x - 40
b2 - 4ac = 225 - 160 = 65
x1,x2 = (-b +/- wortel(b2 - 4ac)) / 2a
x1 = (-15 + wortel(65)) / -2 = (-15 + 8,06) / -2 = -6,94 / -2 = 3,47
x2 = (-15 - wortel(65)) / -2 = (-15 - 8,06) / -2 = -23,06 / -2 = 11,53
4 - 11

**/

fn calculate_distance(hold_time: i64, total_time: i64) -> i64 {
    (total_time - hold_time) * hold_time
}

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}
