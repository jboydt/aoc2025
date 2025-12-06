use pad::PadStr;

pub fn solve(lines: &Vec<String>) -> (i64, i64) {
    // Part 1
    let mut equations = Vec::new();
    for line in lines {
        let mut current = 0;
        let parts = line.split(" ").collect::<Vec<&str>>();
        for part in parts {
            if is_number(part) {
                if current == equations.len() {
                    equations.push(Equation::new());
                }

                let input: i64 = part.parse().unwrap();
                equations[current].inputs.push(input);

                current += 1;
            } else if is_operation(part) {
                equations[current].operation = part.chars().next().unwrap();
                current += 1;
            }
        }
    }

    let mut solution_one: i64 = 0;

    for equation in &equations {
        solution_one += equation.solve();
    }

    // Part 2
    let mut longest_line_length = lines[0].len();
    for line in lines[1..].iter() {
        if line.len() > longest_line_length {
            longest_line_length = line.len();
        }
    }
    let mut fixed_lines = Vec::new();
    for line in lines {
        let fixed_line = line.pad_to_width(longest_line_length);
        fixed_lines.push(fixed_line);
    }

    let mut current_equation = 0;

    let guide_line = fixed_lines[0].clone();
    for (index, _) in guide_line.chars().enumerate() {
        if is_horizontal_spacer(index, &fixed_lines) {
            current_equation += 1;
            continue;
        }
        let input = get_vertical_input(index, &fixed_lines);
        equations[current_equation].vertical_inputs.push(input);
    }

    let mut solution_two: i64 = 0;
    for equation in &equations {
        solution_two += equation.solve_vertical();
    }

    (solution_one, solution_two)
}

fn get_vertical_input(index: usize, lines: &Vec<String>) -> i64 {
    let mut input = String::new();
    for line in lines {
        let digit = line.chars().nth(index).unwrap();
        if digit.is_ascii_digit() {
            input.push(digit);
        }
    }
    input.parse::<i64>().unwrap()
}

fn is_horizontal_spacer(index: usize, lines: &Vec<String>) -> bool {
    for line in lines {
        if line.chars().nth(index).unwrap() != ' ' {
            return false;
        }
    }
    true
}

fn is_number(n: &str) -> bool {
    !n.is_empty() && n.chars().all(|c| c.is_ascii_digit())
}

fn is_operation(n: &str) -> bool {
    !n.is_empty() && n.chars().all(|c| c == '+' || c == '*')
}

struct Equation {
    inputs: Vec<i64>,
    vertical_inputs: Vec<i64>,
    operation: char,
}

impl Equation {
    fn new() -> Self {
        Equation {
            inputs: Vec::new(),
            vertical_inputs: Vec::new(),
            operation: '?',
        }
    }

    fn solve(&self) -> i64 {
        let mut solution = self.inputs[0];
        for i in 1..self.inputs.len() {
            match self.operation {
                '+' => solution += self.inputs[i],
                '*' => solution *= self.inputs[i],
                _ => {}
            }
        }
        solution
    }

    fn solve_vertical(&self) -> i64 {
        let mut solution = self.vertical_inputs[0];
        for i in 1..self.vertical_inputs.len() {
            match self.operation {
                '+' => solution += self.vertical_inputs[i],
                '*' => solution *= self.vertical_inputs[i],
                _ => {}
            }
        }
        solution
    }
}
