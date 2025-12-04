pub fn solve(lines: &Vec<String>) -> (i64, i64) {
    // Solution
    let mut solution_one: i64 = 0;
    let mut solution_two: i64 = 0;
    for battery_bank in lines {
        solution_one += get_joltage(battery_bank, 2, 0);
        solution_two += get_joltage(battery_bank, 12, 0);
    }

    (solution_one, solution_two)
}

// fn get_joltage_one(battery_str: &str) -> i64 {
//     let mut left_battery = battery_str[0..1].parse::<i64>().unwrap();
//     let mut left_pos = 0;
//     for i in 1..battery_str.len() - 1 {
//         let next_battery: i64 = battery_str[i..i + 1].parse().unwrap();
//         if next_battery > left_battery {
//             left_battery = next_battery;
//             left_pos = i;
//         }
//     }
//
//     left_pos += 1;
//     let mut right_battery: i64 = battery_str[left_pos..left_pos + 1].parse().unwrap();
//     for i in left_pos + 1..battery_str.len() {
//         let next_battery: i64 = battery_str[i..i + 1].parse().unwrap();
//         if next_battery > right_battery {
//             right_battery = next_battery;
//         }
//     }
//
//     left_battery * 10 + right_battery
// }

fn get_joltage(battery_str: &str, needed: usize, sum: i64) -> i64 {
    let battery_bank = String::from(battery_str);
    let mut max = battery_bank[0..1].parse::<i64>().unwrap();
    let mut max_index: usize = 0;
    let mut counter: usize = 1;
    while battery_bank.len() - counter >= needed {
        let next_battery = battery_bank[counter..counter+1].parse::<i64>().unwrap();
        if next_battery > max {
            max = next_battery;
            max_index = counter;
        }
        counter += 1;
    }
    let max_scaled = (10_i64).pow(needed as u32 - 1) * max;
    if needed == 1 {
        sum + max
    } else {
        sum + max_scaled + get_joltage(&battery_str[max_index + 1..], needed - 1, sum)
    }
}
