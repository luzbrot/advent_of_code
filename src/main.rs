use std::fs;
use regex::Regex;


#[derive(Debug, Clone)]
struct Set {
    red: i32,
    green: i32,
    blue: i32,
}
impl Set {
    fn new(line: &str) -> Self {
        let re_red = Regex::new(r"(\d+) red").unwrap();
        let re_green = Regex::new(r"(\d+) green").unwrap();
        let re_blue = Regex::new(r"(\d+) blue").unwrap();

        let red = re_red.captures(line).map_or(0, |el| el.get(1).unwrap().as_str().parse::<i32>().unwrap());
        let green = re_green.captures(line).map_or(0, |el| el.get(1).unwrap().as_str().parse::<i32>().unwrap());
        let blue = re_blue.captures(line).map_or(0, |el| el.get(1).unwrap().as_str().parse::<i32>().unwrap());

        Self {
            red,
            green,
            blue
        }
    }
    fn power(&self) -> i32 {
        return self.red * self.green * self.blue;
    }
    fn reduce(a: Self, b: Self) -> Self {
        Self {
            red: a.red.max(b.red),
            green: a.green.max(b.green),
            blue: a.blue.max(b.blue)
        }
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<Set>
}
impl Game {
    fn new(line: &str) -> Self {
        let re_game = Regex::new(r"Game (\d+): ").unwrap();

        let id = re_game.captures(line).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
        let line = re_game.replace(line, "").to_string();

        Self {
            id: id,
            sets: line.split(";").map(|el| Set::new(el)).collect()
        }
    }

    fn is_possible(&self, red: i32, green: i32, blue: i32) -> bool {
        self.sets.iter().all(|el| el.red <= red && el.green <= green && el.blue <= blue)
    }

    fn get_minimal_set(&self) -> Set {
        self.sets.iter().cloned().reduce(Set::reduce).unwrap()
    }
}



fn main() {
    let input = fs::read_to_string("input").unwrap_or_else(|_| panic!("Unable to read input"));

    let games = input.split("\n").map(|el| Game::new(el));
    let sum: i32 = games.clone().filter(|el| el.is_possible(12, 13, 14)).map(|el| el.id).sum();

    println!("The sum is {}", sum);

    let sum2: i32 = games.map(|el| el.get_minimal_set()).map(|el| el.power()).sum();
    println!("The sum for the minimal games is {}", sum2);
}


#[cfg(test)]
mod tests {
    use crate::Game;

    #[test]
    fn check_against_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let games = input.split("\n").map(|el| Game::new(el));
        let sum: i32 = games.clone().filter(|el| el.is_possible(12, 13, 14)).map(|el| el.id).sum();
        assert_eq!(sum, 8);

        let sum2: i32 = games.map(|el| el.get_minimal_set()).map(|el| el.power()).sum();
        assert_eq!(sum2, 2286);
    }
}