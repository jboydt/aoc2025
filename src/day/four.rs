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

    for (row_idx, row) in grid.iter().enumerate() {
        for (pos_idx, _) in row.iter().enumerate() {
            if can_be_accessed(pos_idx, row_idx, &grid) {
                solution_one += 1;
            }
        }
    }

    loop {
        let test_grid = grid.clone();
        let mut removed = 0;
        for (row_idx, row) in test_grid.iter().enumerate() {
            for (pos_idx, _) in row.iter().enumerate() {
                if can_be_accessed(pos_idx, row_idx, &test_grid) {
                    grid[row_idx][pos_idx] = '.';
                    removed += 1;
                    solution_two += 1;
                }
            }
        }

        if removed == 0 {
            break;
        }
    }

    (solution_one, solution_two)
}

fn can_be_accessed(pos_idx: usize, row_idx: usize, grid: &[Vec<char>]) -> bool {
    if grid[row_idx][pos_idx] == '.' {
        return false;
    }

    let mut blocked = 0;

    let x = pos_idx as i32;
    let y = row_idx as i32;

    if contains_paper_roll(x - 1, y - 1, grid) {
        blocked += 1;
    }

    if contains_paper_roll(x, y - 1, grid) {
        blocked += 1;
    }

    if contains_paper_roll(x + 1, y - 1, grid) {
        blocked += 1;
    }

    if contains_paper_roll(x + 1, y, grid) {
        blocked += 1;
    }

    if contains_paper_roll(x + 1, y + 1, grid) {
        blocked += 1;
    }

    if contains_paper_roll(x, y + 1, grid) {
        blocked += 1;
    }

    if contains_paper_roll(x - 1, y + 1, grid) {
        blocked += 1;
    }

    if contains_paper_roll(x - 1, y, grid) {
        blocked += 1;
    }

    blocked < 4
}

fn contains_paper_roll(x: i32, y: i32, grid: &[Vec<char>]) -> bool {
    if x < 0 || y < 0 || y >= grid.len() as i32 || x >= grid[0].len() as i32 {
        return false;
    } else if grid[y as usize][x as usize] == '@' {
        return true;
    }
    false
}