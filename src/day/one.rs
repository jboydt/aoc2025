pub fn solve(lines: &Vec<String>) -> (i64, i64) {
    // Prepare inputs
    let mut inputs = Vec::new();
    for line in lines {
        let direction = line.chars().next().unwrap();
        let mut distance: i64 = line[1..].parse().unwrap();
        if direction == 'L' || direction == 'l' {
            distance *= -1;
        }
        inputs.push(distance);
    }

    // Solution
    let mut indicator: i64 = 50;
    let mut solution_one: i64 = 0;
    let mut solution_two: i64 = 0;
    for distance in &inputs {
        solution_two += (distance / 100).abs();
        let remaining = distance % 100;

        let last_indicator = indicator;
        indicator += remaining;
        if indicator < 0 {
            indicator += 100;
            if last_indicator > 0 {
                solution_two += 1;
            }
        } else if indicator > 99 {
            indicator -= 100;
            if indicator != 0 {
                solution_two += 1;
            }
        }

        if indicator == 0 {
            solution_one += 1;
        }
    }

    (solution_one, solution_one + solution_two)
}
