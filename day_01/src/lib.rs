#[cfg(test)]
mod tests {

    #[test]
    fn trebuchet_01() {
        let input_lines = include_str!("trebuchet_input.txt").lines();
        let mut coordinates = Vec::new();

        for line in input_lines {
            let chars = line.chars();

            let nums: Vec<_> = chars.flat_map(|x| x.to_digit(10)).collect();
            let first_num = nums.first().expect("no number").to_owned();
            let last_num = nums.last().expect("no number").to_owned();

            coordinates.push(first_num * 10 + last_num);
        }

        let mut result = 0;

        for coordinate in coordinates {
            result += coordinate;
        }

        println!("{result}");
    }
}
