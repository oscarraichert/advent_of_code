#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn trebuchet_01() {
        let input_lines = include_str!("trebuchet_input.txt").lines();
        let mut calibrations = Vec::new();

        for line in input_lines {
            let chars = line.chars();

            let nums: Vec<_> = chars.flat_map(|x| x.to_digit(10)).collect();
            let first_num = nums.first().expect("no number").to_owned();
            let last_num = nums.last().expect("no number").to_owned();

            calibrations.push(first_num * 10 + last_num);
        }

        let mut result = 0;

        for calibration in calibrations {
            result += calibration;
        }

        println!("{result}");
    }

    #[test]
    fn trebuchet_02() {
        let result = calculate_calibration(include_str!("trebuchet_input.txt"));

        assert_eq!(result, 54980);
    }

    #[test]
    fn trebuchet_02_ex() {
        let result = calculate_calibration(include_str!("trebuchet_input_ex_02.txt"));

        assert_eq!(result, 281);
    }

    #[test]
    fn trebuchet_ex_line_02() {
        let result = calculate_calibration("eightwothree");

        assert_eq!(result, 83);
    }

    fn calculate_calibration(input: &str) -> u32 {
        let input_lines = input.lines();

        let nums: HashMap<_, u32> = HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]);

        let mut result = 0;

        for line in input_lines {
            result += get_line_calibration(line, &nums);
        }
        result
    }

    fn get_line_calibration(line: &str, nums: &HashMap<&str, u32>) -> u32 {
        let mut line_calibrations: Vec<(usize, u32)> = Vec::new();

        let digits_char: Vec<_> = line.chars().filter(|x| x.is_digit(10)).collect();

        if digits_char.len() > 0 {
            let first_digit_char = digits_char.first().expect("no number").to_owned();
            let last_digit_char = digits_char.last().expect("no number").to_owned();

            let mut first_digit: Vec<_> = line
                .match_indices(first_digit_char)
                .flat_map(|x| x.1.parse::<u32>().map(|y| (x.0, y)))
                .collect();

            let mut last_digit: Vec<_> = line
                .match_indices(last_digit_char)
                .flat_map(|x| x.1.parse::<u32>().map(|y| (x.0, y)))
                .collect();

            line_calibrations.append(&mut first_digit);
            line_calibrations.append(&mut last_digit);
        }

        for extensive_num in nums.keys() {
            if line.contains(extensive_num) {
                let mut extensive_num_index: Vec<_> = line
                    .match_indices(extensive_num)
                    .map(|x| (x.0, nums.get(x.1).expect("no conversion").to_owned()))
                    .collect();

                line_calibrations.append(&mut extensive_num_index);
            }
        }

        let first_num = line_calibrations
            .iter()
            .min_by(|a, b| a.0.cmp(&b.0))
            .expect("nonexistent")
            .1;

        let last_num = line_calibrations
            .iter()
            .max_by(|a, b| a.0.cmp(&b.0))
            .expect("nonexistent")
            .1;

        first_num * 10 + last_num
    }
}
