use std::fs;

#[derive(Debug)]
enum Type {
    Number(u32),
    Space,
    Symbol(char)
}
impl From<char> for Type {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Space,
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Self::Number(value.to_digit(10).unwrap()),
            _ => Self::Symbol(value)
        }
    }
}

#[derive(Debug)]
struct Schematic {
    width: usize,
    plan: Vec<Type>
}
impl Schematic {
    fn new(input: &str) -> Self {
        Self {
            width: input.find('\n').unwrap(),
            plan: input.chars().into_iter().filter(|el| !el.is_ascii_whitespace()).map(|el| el.into()).collect()
        }
    }

    fn get_height(&self) -> usize {
        self.plan.len() / self.width
    }

    fn find_relevant_parts(&self) -> Vec<u32> {
        let mut res = Vec::new();

        for y in 0..self.get_height() {
            let mut number: u32 = 0;
            let mut relevant = false;
            for x in 0..self.width {
                match self.plan[self.width * y + x] {
                    Type::Number(n) => {
                        number = number * 10 + n;
                        if self.is_near_symbol(x, y) {
                            relevant = true;
                        }
                    },
                    Type::Space | Type::Symbol(_) => {
                        if number > 0 {
                            if relevant {
                                res.push(number);
                            }
                            number = 0;
                            relevant = false;
                        }
                    }
                }
            }
            if number > 0 {
                if relevant {
                    res.push(number);
                }
            }
        }

        res
    }

    fn is_near_symbol(&self, x: usize, y: usize) -> bool {
        let ym = if y > 0 { y - 1 } else { 0 };
        let yp = if y >= self.get_height() - 1 { self.get_height() - 1 } else { y + 1 };
        for yi in ym..yp+1 {
            let xm = if x > 0 { x - 1 } else { 0 };
            let xp = if x >= self.width - 1 { self.width - 1 } else { x + 1 };
            for xi in xm..xp+1 {
                if xi == x && yi == y { continue; }
                match self.plan[self.width * yi + xi] {
                    Type::Symbol(_) => return true,
                    _ => continue
                }
            }
        }

        false
    }
}



fn main() {
    let input = fs::read_to_string("input").unwrap_or_else(|_| panic!("Unable to read input"));

    let schema = Schematic::new(input.as_str());
    println!("The sum is {}", schema.find_relevant_parts().iter().sum::<u32>());
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn check_against_example() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let schema = Schematic::new(input);
        assert_eq!(schema.find_relevant_parts().iter().sum::<u32>(), 4361);
    }
}