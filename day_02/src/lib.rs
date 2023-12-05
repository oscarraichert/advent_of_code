#[derive(Debug)]
struct Game {
    id: u16,
    sets: Vec<Set>,
}

type Set = (u16, String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cube_conundrum_01() {
        let games = get_games();

        let result: u16 = games
            .iter()
            .filter(|game| {
                game.sets.iter().all(|set| match set.1.as_str() {
                    "red" => set.0 <= 12,
                    "green" => set.0 <= 13,
                    "blue" => set.0 <= 14,
                    _ => unreachable!(),
                })
            })
            .map(|valid_game| valid_game.id)
            .sum();

        println!("{result}")
    }

    #[test]
    fn cube_conundrum_02() {
        let games = get_games();

        
        let result: u16 = games.iter().map(|game| {
            let f = |color| game.sets.iter().filter(|set| set.1 == color).map(|set| set.0).max().unwrap();

            let red = f("red");
            let green = f("green");
            let blue = f("blue");

            red * green * blue
        }).sum();

        println!("{result}")
    }

    fn get_games() -> Vec<Game> {
        let input_lines = include_str!("cube_conundrum_input.txt").lines();

        let mut games = Vec::new();

        for line in input_lines {
            let mut game_line = line.split(':');
            let id = game_line
                .next()
                .unwrap()
                .replace("Game ", "")
                .parse()
                .unwrap();

            let draws = game_line.next().unwrap().replace(";", ",");
            let draws = draws.split(',');

            let mut sets = Vec::new();

            for draw in draws {
                let mut draw_elem = draw.trim().split(' ');

                sets.push((
                    draw_elem.next().unwrap().parse().unwrap(),
                    draw_elem.next().unwrap().to_owned(),
                ));
            }

            let game = Game { id, sets };
            games.push(game);
        }
        games
    }
}
