pub fn solve(lines: &Vec<String>) -> (i64, i64) {
    // Prepare inputs
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for ch in line.chars() {
            row.push(ch);
        }
        grid.push(row);
    }

    // Solution
    let mut solution_one: i64 = 0;
    let mut solution_two: i64 = 0;

    (solution_one, solution_two)
}
