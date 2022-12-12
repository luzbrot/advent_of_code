use clap::Parser;
use enum_iterator::{all, Sequence};
use std::{
    fs,
    cmp::Ordering,
    convert::Into
};


/// CLI Input
#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
struct Args {
    /// If set the test data is loaded
    #[arg(long)]
    test: bool,
}


/// Enum of the selection on can make in Rock Paper Scissors
#[derive(Debug, Clone, Copy, Eq, PartialEq, Sequence)]
enum Selection {
    Rock,
    Paper,
    Scissors
}
/// Implementation to convert Selection to numbers like defined
impl From<Selection> for u32 {
    fn from(s: Selection) -> Self {
        match s {
            Selection::Rock => 1,
            Selection::Paper => 2,
            Selection::Scissors => 3
        }
    }
}
/// Convert to Selection from string for Part one and two
impl From<&str> for Selection {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("No conversion from {}", s)
        }
    }
}
/// Needed for Ord
impl PartialOrd for Selection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
/// Implement ordering depending on who wins seen from Self
/// Less => Lose
/// Equal => Draw
/// Greater => Win
impl Ord for Selection {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::Rock => {
                match other {
                    Self::Rock => Ordering::Equal,
                    Self::Paper => Ordering::Less,
                    Self::Scissors => Ordering::Greater
                }
            },
            Self::Paper =>  {
                match other {
                    Self::Rock => Ordering::Greater,
                    Self::Paper => Ordering::Equal,
                    Self::Scissors => Ordering::Less
                }
            },
            Self::Scissors => {
                match other {
                    Self::Rock => Ordering::Less,
                    Self::Paper => Ordering::Greater,
                    Self::Scissors => Ordering::Equal
                }
            }
        }
    }
}

/// Wanted result for the second part of the challenge
#[derive(Debug)]
enum ExpectedResult {
    Lose,
    Draw,
    Win
}
/// Convert ExpectedResult from string
impl From<&str> for ExpectedResult {
    fn from(s: &str) -> Self {
        match s {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("No conversion from {}", s)
        }
    }
}

// Save information of a math of part one and part two
#[derive(Debug)]
struct Match {
    // Selection for the opponent
    opponent: Selection,
    // Selection of the reader
    you: Selection,
    // The wanted result for the second part of the challenge
    wanted_result: ExpectedResult
}
impl Match {
    /// Create a new Match from a line from the input
    /// 
    /// # Arguments
    /// * `s` - The line of a match parted with ' ' (e.g.: 'A X')
    fn new(s: &str) -> Self {
        let mut parts = s.split(' ');
        // We do expect 2 inputs otherwise the data is malformed
        let first = parts.next().unwrap();
        let second = parts.next().unwrap();
        Self {
            opponent: first.into(),
            you: second.into(),
            wanted_result: second.into()
        }
    }
    /// Checks if the match would be won
    /// 
    /// # Returns
    /// True if match would be won
    fn do_you_win(&self) -> bool {
        self.you > self.opponent
    }
    /// Sets `self.you` depending on the wanted result.
    /// 
    /// This works by iterating through all possible selections finding the one fulfilling the win requirement.
    fn correct_your_selection(&mut self) {
        self.you = match self.wanted_result {
            ExpectedResult::Lose => {
                all::<Selection>().find(|el| el < &self.opponent).unwrap()
            },
            ExpectedResult::Draw => {
                all::<Selection>().find(|el| el == &self.opponent).unwrap()
            },
            ExpectedResult::Win => {
                all::<Selection>().find(|el| el > &self.opponent).unwrap()
            }
        }
    }
}
/// Implementation to convert a Match to numbers like defined
impl From<&Match> for u32 {
    fn from(m: &Match) -> Self {
        let mut points: u32 = m.you.into();
        points += if m.do_you_win() { 6 } else if m.opponent == m.you { 3 } else { 0 };
        points
    }
}

fn main() {
    let args = Args::parse();

    let file = if args.test { "test_input" } else { "input" };

    let input = fs::read_to_string(file).unwrap_or_else(|_| panic!("Unable to read {}", file));

    let mut matches: Vec<Match> = Vec::new();

    for m in input.split('\n') {
        matches.push(Match::new(m));
    }

    let mut points: u32 = matches.iter().map(|el| -> u32 {el.into()}).sum();
    println!("The total score is: {}", points);

    matches.iter_mut().for_each(|el| el.correct_your_selection());
    points = matches.iter().map(|el| -> u32 {el.into()}).sum();
    println!("The corrected total score is: {}", points);
}
