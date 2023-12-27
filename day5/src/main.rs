use std::time::Instant;

fn main() {
    let input = include_str!("input.txt");

    let mut seeds: Vec<i64> = Vec::new();
    let mut filters: Vec<Filter> = Vec::new();
    let mut filter: Option<Filter> = None;
    for line in input.lines() {
        // process first line
        if line.starts_with("seeds: ") {
            // part 1
            // seeds = line
            //     .replace("seeds: ", "")
            //     .split(" ")
            //     .map(|x| x.trim())
            //     .filter(|x| !x.is_empty())
            //     .map(|x| x.parse::<i64>().unwrap())
            //     .collect();

            // part 2
            seeds = parse_seeds(line.replace("seeds: ", ""));

            continue;
        }

        if line.is_empty() {
            if filter.is_some() {
                filters.push(filter.unwrap())
            }
            filter = None;
            continue;
        }

        if line.ends_with("map:") {
            filter = Some(Filter {
                name: line.to_string(),
                computations: Vec::new(),
            });
            // could parse the name but perhaps not needed
            continue;
        }

        if filter.as_mut().is_some() {
            let mut numbers = line.split(" ").map(|x| x.trim());
            let dest_range_start = numbers.next().unwrap().parse().unwrap();
            let src_range_start = numbers.next().unwrap().parse().unwrap();
            let range_len = numbers.next().unwrap().parse().unwrap();
            filter
                .as_mut()
                .unwrap()
                .computations
                .push(create_computation(
                    dest_range_start,
                    src_range_start,
                    range_len,
                ));
        }
    }

    // println!("seeds is {:?}", seeds);
    println!("len seeds is {:?}", seeds.len());
    println!("len filters is {:?}", filters.len());

    // let mut results: Vec<i64> = Vec::new();
    let mut min_result = i64::MAX;
    let mut now = Instant::now();
    let start = Instant::now();
    for (i, s) in seeds.iter().enumerate() {
        let mut result = *s;
        for f in &filters {
            result = f.apply(result);
            // println!("filter {} applied, result is now {}", f.name, result);
        }
        // println!("result is {result}");
        if result < min_result {
            min_result = result;
            println!(
                "Min result is now {}, elapsed seeds: {}, elapsed time: {:?}",
                min_result,
                i,
                start.elapsed()
            )
        }
        // results.push(result);

        // if results.len() % 10_000_000 == 0 {
        //     println!(
        //         "Running slow_function() took {} seconds and now has length {}.",
        //         now.elapsed().as_secs(),
        //         results.len()
        //     );
        //     now = Instant::now();
        // }
    }

    // results.sort_by(|a, b| a.cmp(b));
    // println!("lowest result is {}", results[0]);
}

fn parse_seeds(line: String) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    let parts: Vec<i64> = line
        .split(" ")
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();
    for i in (0..parts.len()).step_by(2) {
        let start_range = parts[i];
        let range_len = parts[i + 1];
        for x in start_range..start_range + range_len {
            result.push(x)
        }
    }

    result
}

fn create_computation(dest_range_start: i64, source_range_start: i64, range: i64) -> Computation {
    Computation {
        dest_range_start,
        source_range_start,
        source_range_end: source_range_start + range,
    }
}

struct Filter {
    name: String,
    computations: Vec<Computation>,
}

impl Filter {
    fn apply(&self, input: i64) -> i64 {
        for c in &self.computations {
            if c.is_applicable_for(input) {
                return c.apply(input);
            }
        }

        // fallback to no change
        input
    }
}

struct Computation {
    dest_range_start: i64,
    source_range_start: i64,
    source_range_end: i64,
}

impl Computation {
    fn is_applicable_for(&self, input: i64) -> bool {
        input >= self.source_range_start && input < self.source_range_end
    }

    fn apply(&self, input: i64) -> i64 {
        self.dest_range_start + (input - self.source_range_start)
    }
}
