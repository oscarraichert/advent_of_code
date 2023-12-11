#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gear_ratios_01() {
        let input = include_str!("input.txt");
        let lines = input.lines();
        let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let line_count = matrix.len();
        let line_len = matrix.get(0).unwrap().len();
        let nums = get_nums(&lines);
        let symbols = get_symbols(lines);

        let mut parts_sum = 0;

        for num in &nums {
            let symbol_next_to = check_adjacent_symbol(num, &symbols, line_count, line_len);

            if symbol_next_to {
                parts_sum += num.2;
            }
        }

        println!("{parts_sum}")
    }

    #[test]
    fn gear_ratios_01_ex() {
        let input = include_str!("ex_input.txt");
        let lines = input.lines();
        let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let line_count = matrix.len();
        let line_len = matrix.get(0).unwrap().len();
        let nums = get_nums(&lines);
        let symbols = get_symbols(lines);

        let mut parts_sum = 0;

        for num in &nums {
            let symbol_next_to = check_adjacent_symbol(num, &symbols, line_count, line_len);

            if symbol_next_to {
                parts_sum += num.2;
            }
        }
        
        assert_eq!(parts_sum, 4361);
    }
}

fn check_adjacent_symbol(
    num: &(usize, usize, usize),
    symbols: &[(usize, usize, char)],
    last_line: usize,
    line_len: usize,
) -> bool {
    let mut syms = Vec::new();

    let mut sym_over: Vec<_> = symbols
        .iter()
        .filter(|x| if num.0 > 0 { x.0 == num.0 - 1 } else { false })
        .collect();

    let mut sym_under: Vec<_> = symbols
        .iter()
        .filter(|x| {
            if num.0 < last_line {
                x.0 == num.0 + 1
            } else {
                false
            }
        })
        .collect();

    let mut sym_same_line: Vec<_> = symbols.iter().filter(|x| x.0 == num.0).collect();

    syms.append(&mut sym_same_line);
    syms.append(&mut sym_over);
    syms.append(&mut sym_under);    

    let left: Vec<_> = syms
        .iter()
        .filter(|s| {
            if num.1 != 0 {
                s.1 >= num.1 - 1 && s.1 <= num.1 + num.2.to_string().len() - 1
            } else {
                false
            }
        })
        .collect();

    let right: Vec<_> = syms
        .iter()
        .filter(|s| {
            if num.1 < line_len {
                s.1 >= num.1 && s.1 <= num.1 + num.2.to_string().len()
            } else {
                false
            }
        })
        .collect();

    !left.is_empty() || !right.is_empty()
}

fn get_symbols(lines: std::str::Lines<'_>) -> Vec<(usize, usize, char)> {
    let mut symbols = Vec::new();

    for (y, line) in lines.enumerate() {
        let mut chars = line.chars().enumerate();
        while let Some((x, char)) = chars.next() {
            if !char.is_digit(10) && char != '.' {
                symbols.push((y, x, char));
            }
        }
    }
    symbols
}

fn get_nums(lines: &std::str::Lines<'_>) -> Vec<(usize, usize, usize)> {
    let mut nums: Vec<(usize, usize, usize)> = Vec::new();

    for (y, line) in lines.clone().enumerate() {
        let mut chars = line.chars().enumerate();
        while let Some((x, char)) = chars.next() {
            if char.is_digit(10) {
                let num: String = chars
                    .clone()
                    .take_while(|c| c.1.is_digit(10))
                    .map(|c| c.1)
                    .collect();
                nums.push((y, x, (format!("{char}") + &num).parse().unwrap()));

                chars.nth(num.len());
            }
        }
    }
    nums
}
