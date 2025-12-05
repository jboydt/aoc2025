pub fn solve(lines: &Vec<String>) -> (i64, i64) {
    // Prepare inputs
    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();

    let mut parse_ranges = true;
    for line in lines {
        if line.is_empty() {
            parse_ranges = false;
            continue;
        }

        if parse_ranges {
            let range = line.split('-').collect::<Vec<&str>>();
            let range_start = range[0].parse::<i64>().unwrap();
            let range_end = range[1].parse::<i64>().unwrap();
            ranges.push((range_start, range_end));
        } else {
            let id = line.parse::<i64>().unwrap();
            ingredients.push(id);
        }
    }

    // Solution
    let initial_length = ingredients.len();
    ingredients.retain(|id| !is_in_a_range(id, &ranges));
    let solution_one = (initial_length - ingredients.len()) as i64;

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut adjusted = true;
    while adjusted {
        adjusted = false;

        for i in 0..ranges.len() - 1 {
            if ranges[i + 1].0 >= ranges[i].0
                && ranges[i + 1].0 <= ranges[i].1
                && ranges[i + 1].1 > ranges[i].1
            {
                ranges[i].1 = ranges[i + 1].1;
                ranges.remove(i + 1);
                adjusted = true;
                break;
            } else if ranges[i + 1].0 >= ranges[i].0 && ranges[i + 1].1 <= ranges[i].1 {
                ranges.remove(i + 1);
                adjusted = true;
                break;
            }
        }
    }

    let mut solution_two: i64 = 0;
    for range in ranges {
        solution_two += range.1 - range.0 + 1;
    }

    (solution_one, solution_two)
}

fn is_in_a_range(id: &i64, ranges: &Vec<(i64, i64)>) -> bool {
    for range in ranges {
        if is_in_range(id, &range.0, &range.1) {
            return true;
        }
    }
    false
}

fn is_in_range(id: &i64, start: &i64, end: &i64) -> bool {
    id >= start && id <= end
}
