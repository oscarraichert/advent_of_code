#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gear_ratios_01_ex() {
        let input = include_str!("ex_input.txt");
        let lines = input.lines();
        let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let nums = get_nums(&lines);
        let symbols = get_symbols(lines);

        for num in &nums {
            let num_l = num.2.to_string().len();
            let num_crd = (num.0, num.1, num_l);

            let result = symbol_next_to(num, &symbols);

            if result {
                println!("{num:?}");
            }
            println!("{result}");
        }

        // println!("{nums:?}");
        // println!("{symbols:?}");
    }

    fn symbol_next_to(num: &(usize, usize, usize), symbols: &[(usize, usize, char)]) -> bool {
        let left = symbols
            .iter()
            .any(|s| if num.1 != 0 { s.1 == num.1 - 1 } else { false });

        let right = symbols
            .iter()
            .any(|s| s.1 == num.1 + num.2.to_string().len());

        if left | right {
            return true;
        }

        false
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
}
