#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gear_ratios_01_ex() {
        let input = include_str!("ex_input.txt");
        let lines = input.lines();
        let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let x_size = matrix.get(0).unwrap().len();
        let y_size = lines.clone().collect::<Vec<_>>().len();

        let mut digits: Vec<_> = Vec::new();
        let mut symbols: Vec<_> = Vec::new();

        for line in lines.enumerate() {
            let chars = line.1.chars();
            let char_enum = chars.enumerate();

            let mut line_digits: Vec<_> = char_enum
                .to_owned()
                .filter_map(|c| c.1.to_digit(10).map(|v| (line.0, c.0, v)))
                .collect();

            let mut line_symbols: Vec<_> = char_enum
                .filter(|c| !c.1.is_digit(10))
                .filter(|c| c.1 != '.')
                .map(|s| (line.0, s.0, s.1))
                .collect();

            digits.append(&mut line_digits);
            symbols.append(&mut line_symbols);
        }

        println!("{digits:?}");
        println!("{symbols:?}")
    }
}
