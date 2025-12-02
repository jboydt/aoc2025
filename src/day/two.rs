pub fn solve(lines: &Vec<String>) -> (i64, i64) {
    // Prepare inputs
    let mut inputs = Vec::new();
    for line in lines {
        let parts = line.split(",").collect::<Vec<&str>>();
        for part in parts {
            let numbers = part.split("-").collect::<Vec<&str>>();
            if numbers.len() == 2 {
                inputs.push((
                    numbers[0].parse::<i64>().unwrap(),
                    numbers[1].parse::<i64>().unwrap(),
                ));
            }
        }
    }

    // Solution
    let mut solution_one = 0;
    let mut solution_two = 0;

    for input in inputs {
        solution_one += count_illegal_pattern_one(input.0, input.1);
        solution_two += count_illegal_pattern_two(input.0, input.1);
    }

    (solution_one, solution_two)
}

fn count_illegal_pattern_two(start: i64, end: i64) -> i64 {
    let mut sum: i64 = 0;
    for i in start..=end {
        if contains_illegal_pattern_two(i) {
            sum += i;
        }
    }
    sum
}

fn contains_illegal_pattern_two(pattern: i64) -> bool {
    let pattern_str = pattern.to_string();
    let mut take = 1;
    while take < (pattern_str.len() / 2) + 1 {
        let test = pattern_str[0..take].to_string();
        let mut rest = pattern_str[take..].to_string();
        while rest.len() >= take {
            if test == rest[0..take] {
                rest = rest[take..].to_string();
            } else {
                break;
            }
        }
        if rest.len() == 0 {
            return true;
        }
        take += 1;
    }

    false
}

fn count_illegal_pattern_one(start: i64, end: i64) -> i64 {
    let mut sum: i64 = 0;
    for i in start..=end {
        if contains_illegal_pattern_one(i) {
            sum += i;
        }
    }
    sum
}

fn contains_illegal_pattern_one(pattern: i64) -> bool {
    let pattern_str = pattern.to_string();
    let mut take = 1;
    while take < (pattern_str.len() / 2) + 1 {
        if pattern_str[0..take].to_string() == pattern_str[take..] {
            return true;
        }
        take += 1;
    }
    false
}
