use clap::Parser;
use std::fs;


/// CLI Input
#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
struct Args {
    /// If set the test data is loaded
    #[arg(long)]
    test: bool,
}

/// A Item in a rucksack.
/// Just a renamed char.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Item(char);
/// To be able to convert from char to Item
impl From<char> for Item {
    fn from(c: char) -> Self {
        Self(c)
    }
}
/// Convert to priority number for an item
impl From<Item> for u32 {
    fn from(i: Item) -> Self {
        let uc = i.0 as u32;
        if uc >= 97 {
            uc - 97 + 1
        }
        else {
            uc - 65 + 27
        }
    }
}

/// A rucksack with both compartments
#[derive(Debug)]
struct Rucksack {
    /// First compartment
    first: Vec<Item>,
    /// Second compartment
    second: Vec<Item>
}
impl Rucksack {
    /// Create a new rucksack from a input line.
    /// The input gets cut in the middle and put into `first` and `second` and converted to items.
    /// 
    /// # Arguments
    /// * `s` - The input line from the puzzle input
    fn new(s: &str) -> Self {
        let len = s.len();
        let items: Vec<Item> = s.chars().into_iter().map(Item).collect();
        Self {
            first: items[0..len/2].to_vec(),
            second: items[len/2..].to_vec()
        }
    }
    /// Find the duplicate in both compartments.
    /// There should only be one so only one is searched.
    fn get_duplicate(&self) -> Item {
        // If unwarp does not work the input is malformed
        *self.first.iter().find(|el| self.second.iter().any(|e| &e == el)).unwrap()
    }
    /// Find the duplicate and convert it to priority.
    fn get_duplicate_priority(&self) -> u32 {
        self.get_duplicate().into()
    }
    /// Find the token shared between a group of three.
    /// 
    /// # Arguments
    /// * `second` - The second rucksack in the group
    /// * `third` - The third rucksack in the group
    fn find_token(&self, second: &Rucksack, third: &Rucksack) -> Item {
        let mut iter_first = self.first.iter().chain(self.second.iter());
        let iter_second = second.first.iter().chain(second.second.iter());
        let iter_third = third.first.iter().chain(third.second.iter());
        // Unwrap is ok as the input is malformed otherwise
        *iter_first.find(|el| iter_second.clone().any(|e| &e == el) && iter_third.clone().any(|e| &e == el)).unwrap()
    }
    /// Find the token and convert it to priority
    fn get_token_priority(&self, second: &Rucksack, third: &Rucksack) -> u32 {
        self.find_token(second, third).into()
    }
}

fn main() {
    let args = Args::parse();

    let file = if args.test { "test_input" } else { "input" };

    let input = fs::read_to_string(file).unwrap_or_else(|_| panic!("Unable to read {}", file));

    let mut rucksacks: Vec<Rucksack> = Vec::new();

    for r in input.split('\n') {
        rucksacks.push(Rucksack::new(r));
    }

    let priority_sum: u32 = rucksacks.iter().map(|el| el.get_duplicate_priority()).sum();
    println!("The sum of the priorities of the duplicate items is: {:?}", priority_sum);

    let mut iter = rucksacks.iter().peekable();
    let mut tokens: Vec<u32> = Vec::new();
    loop {
        if iter.peek().is_none() {
            break;
        }
        // Unwrap ok as otherwise the input data is malformed
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();
        let third = iter.next().unwrap();
        tokens.push(first.get_token_priority(second, third));
    }

    println!("The sum of the priorities of the badges is: {:?}", tokens.iter().sum::<u32>());
}
