#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gear_ratios_01_ex() {
        let input = include_str!("ex_input.txt");
        let lines = input.lines();
        let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let mut nums: Vec<(usize, usize, usize)> = Vec::new();

        for (y, line) in lines.enumerate() {
            let mut chars = line.chars().enumerate();
            while let Some((x, char)) = chars.next() {
                
                if char.is_digit(10) {
                    let num: String = chars.clone().take_while(|c| c.1.is_digit(10)).map(|c| c.1).collect();
                    nums.push((y, x, (format!("{char}") + &num).parse().unwrap()));

                    chars.nth(num.len());
                }
            }
        }
        println!("{nums:?}")
    }
}
