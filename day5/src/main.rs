use std::time::Instant;

fn main() {
    let input = include_str!("input.txt");

    let mut seeds = Seeds::new(
        input
            .lines()
            .next()
            .unwrap()
            .to_string()
            .replace("seeds: ", ""),
    );

    let mut filters: Vec<Filter> = Vec::new();
    let mut filter: Option<Filter> = None;
    for line in input.lines() {
        // skip first line, already processed
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
                _name: line.to_string(),
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
    // println!("len seeds is {:?}", seeds.len());
    println!("len filters is {:?}", filters.len());

    // let mut results: Vec<i64> = Vec::new();
    let mut min_result = i64::MAX;
    let start = Instant::now();
    let mut index: i64 = 0;
    while let Some(item) = seeds.next() {
        // println!("item is {:?}", item);

        let mut result = item;
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
                index,
                start.elapsed()
            )
        }
        index += 1;
    }

    // results.sort_by(|a, b| a.cmp(b));
    println!("lowest result is {}", min_result);
}

struct Seeds {
    parts: Vec<i64>,
    index: usize,
    current_range: Option<SeedsRange>,
}

#[derive(Debug)]
struct SeedsRange {
    index: i64,
    start: i64, // inclusive
    end: i64,   // exclusive
}

impl Iterator for SeedsRange {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        // println!("calling seedsRange.next(), its index is {}", self.index);
        let result = self.start + self.index;
        // println!("result is {}", result);
        if result >= self.end {
            return None;
        }

        self.index += 1;
        Some(result)
    }
}

impl Seeds {
    fn new(line: String) -> Seeds {
        let parts: Vec<i64> = line
            .split(" ")
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect();

        // println!("Creating seeds from parts: {:?}", parts);
        Seeds {
            parts,
            index: 0,
            current_range: None,
        }
    }
}
impl Iterator for Seeds {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        // println!("seeds.next() called, index is {}", self.index);

        if self.current_range.is_some() {
            let result = self.current_range.as_mut().unwrap().next();
            if result.is_some() {
                // println!("returning result of current range: {:?}", result);
                return result;
            }

            // end of range, move on to the next one
            // println!("end of range, move on to the next one");
            self.current_range = None;
            self.index += 2;
        }

        // end of all the seeds
        if self.index >= self.parts.len() - 1 {
            // println!("end of all seeds ranges");
            return None;
        }

        self.current_range = Some(SeedsRange {
            index: 0,
            start: self.parts[self.index],
            end: self.parts[self.index] + self.parts[self.index + 1],
        });
        // println!("created new current range: {:?}", self.current_range);

        self.current_range.as_mut().unwrap().next()
    }
}
fn create_computation(dest_range_start: i64, source_range_start: i64, range: i64) -> Computation {
    Computation {
        dest_range_start,
        source_range_start,
        source_range_end: source_range_start + range,
    }
}

struct Filter {
    _name: String,
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
