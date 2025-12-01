fn day_one(lines: &Vec<String>) -> (i32, i32) {
    let mut inputs = Vec::new();
    for line in lines {
        let direction = line.chars().next().unwrap();
        let mut distance: i32 = line[1..].parse().unwrap();
        if direction == 'L' || direction == 'l' {
            distance *= -1;
        }
        inputs.push(distance);
    }

    let mut indicator = 50;
    let mut solution_one = 0;
    let mut solution_two = 0;
    for distance in &inputs {
        let mut ticks = distance.clone();
        if ticks < 0 {
            while ticks != 0 {
                ticks += 1;
                indicator -= 1;
                if indicator == -1 {
                    indicator = 99;
                }
                if indicator == 0 {
                    solution_two += 1;
                }
            }
        } else if ticks > 0 {
            while ticks != 0 {
                ticks -= 1;
                indicator += 1;
                if indicator == 100 {
                    indicator = 0;
                }
                if indicator == 0 {
                    solution_two += 1;
                }
            }
        }

        if indicator == 0 {
            solution_one += 1;
        }
    }

    (solution_one, solution_two)
}
