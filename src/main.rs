use std::fs;
use regex::Regex;


#[derive(Debug)]
struct CalibrationLine(Vec<u32>);
impl CalibrationLine {
    fn new(line: &str, replace: bool) -> Self {
        let mut numbers = Vec::new();
        let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
        let mut start = 0;
        while let Some(mat) = re.find_at(line, start) {
            start += 1;
            if mat.len() > 1 {
                if replace {
                    numbers.push(match mat.as_str() {
                        "one" => 1,
                        "two" => 2,
                        "three" => 3,
                        "four" => 4,
                        "five" => 5,
                        "six" => 6,
                        "seven" => 7,
                        "eight" => 8,
                        "nine" => 9,
                        _ => panic!("{}", mat.as_str())
                    });
                }
            }
            else {
                //println!("{}", mat.as_str());
                numbers.push(mat.as_str().parse::<u32>().unwrap());
            }
        };
        
        CalibrationLine(numbers)
    }
    fn get_first_digit(&self) -> u32 {
        *self.0.first().unwrap()
    }
    fn get_last_digit(&self) -> u32 {
        *self.0.last().unwrap()
        //self.0.chars().filter(char::is_ascii_digit).next_back().unwrap().to_digit(10).unwrap()
    }
    fn get_value(&self) -> u32 {
        self.get_first_digit()*10 + self.get_last_digit()
    }
}



fn main() {
    let input = fs::read_to_string("input").unwrap_or_else(|_| panic!("Unable to read input"));

    let res = input.split('\n').map(|el| CalibrationLine::new(el, false)).map(|el| el.get_value()).sum::<u32>();
    println!("The calibration value is {}", res);

    let res2 = input.split('\n').map(|el| CalibrationLine::new(el, true)).map(|el| el.get_value()).sum::<u32>();
    println!("The correct calibration value is {}", res2);
}


#[cfg(test)]
mod tests {
    use crate::CalibrationLine;


    #[test]
    fn check_against_example() {
        let mut input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(input.split('\n').map(|el| CalibrationLine::new(el, false)).map(|el| el.get_value()).sum::<u32>(), 142);

        input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(input.split('\n').map(|el| CalibrationLine::new(el, true)).map(|el| el.get_value()).sum::<u32>(), 281);
    }
}