#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gear_ratios_01_ex() {
        let mut lines: Vec<Vec<char>> = include_str!("ex_input.txt").lines().map(|line| line.chars().collect()).collect();
        
        let x = lines.get(0).unwrap().get(0).unwrap();

        println!("{x}")
    }
}
