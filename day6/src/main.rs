fn main() {
    let mut input = include_str!("test_input.txt").lines();

    let times: Vec<i32> = input
        .next()
        .unwrap()
        .replace("Time:", "")
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    let distances: Vec<i32> = input
        .next()
        .unwrap()
        .replace("Distance:", "")
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    let zip = times.iter().zip(distances.iter());
    let races = zip.map(|(time, distance)| Race {
        time: *time,
        distance: *distance,
    });

    println!("Hello, world! {:?}", races);

    let mut results: Vec<i32> = Vec::new();
    for race in races {
        let mut count = 0;
        for i in 0..race.time {
            let d = calculate_distance(i, race.time);
            if d > race.distance {
                count += 1;
            }
        }
        results.push(count);
    }

    println!("got results {:?}", results);
    println!(
        "multiplied this is {}",
        results.iter().fold(1, |acc, x| acc * x)
    );
}

fn calculate_distance(hold_time: i32, total_time: i32) -> i32 {
    (total_time - hold_time) * hold_time
}

struct Race {
    time: i32,
    distance: i32,
}
