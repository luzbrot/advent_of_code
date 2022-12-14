use clap::Parser;
use std::{fs, collections::LinkedList};
use regex::Regex;


/// CLI Input
#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
struct Args {
    /// If set the test data is loaded
    #[arg(long)]
    test: bool,
}


/// Renamed Vec as Stacks to impl functions on it
#[derive(Debug, Clone)]
struct Stacks(Vec<LinkedList<char>>);

impl Stacks {
    fn new() -> Self {
        Self(Vec::new())
    }

    /// Add crates from the bottom to the stacks.
    /// Add stacks if needed.
    /// 
    /// # Arguments
    /// * `s` - Line from the input defined a row of stacks
    fn add_from_bottom(&mut self, s: &str) {
        let crate_re = Regex::new(r"\[(.)\]").unwrap();

        let count = (s.len() - 3) / 4 + 1;
        while self.0.len() < count {
            self.0.push(LinkedList::new());
        }

        for i in 0..count {
            let crate_ = s.chars().skip(i*4).take(3).collect::<String>();
            if let Some(c) = crate_re.captures(crate_.as_str()) {
                self.0[i].push_back(c.get(1).unwrap().as_str().chars().next().unwrap());
            }
        }
    }
    /// Do a move operation of a CrateMover9000 as defined.
    /// 
    /// # Arguments
    /// * `from` - move from
    /// * `to` - move to
    /// * `count` - the number to move
    fn do_operation_9000(&mut self, from: usize, to: usize, count: usize) {
        // Did not find a fancy iterator operation -.-
        for _ in 0..count {
            let tmp = self.0[from - 1].pop_front().unwrap(); // unwrap as input should not be malformed
            self.0[to - 1].push_front(tmp);
        }
    }
    /// Do a move operation of a CrateMover9001 as defined.
    /// 
    /// # Arguments
    /// * `from` - move from
    /// * `to` - move to
    /// * `count` - the number to move
    fn do_operation_9001(&mut self, from: usize, to: usize, count: usize) {
        // Did not find a fancy iterator operation -.-
        let mut tmp = LinkedList::new();
        for _ in 0..count {
            tmp.push_back(self.0[from - 1].pop_front().unwrap()); // unwrap as input should not be malformed
        }
        for _ in 0..count {
            self.0[to - 1].push_front(tmp.pop_back().unwrap());
        }
    }
    /// Create the message defined by the crate on top of each stack
    fn top_message(&self) -> String {
        self.0.iter().map(|el| el.front().unwrap()).collect() // unwrap as input should not be malformed
    }
}

fn main() {
    let args = Args::parse();

    let file = if args.test { "test_input" } else { "input" };

    let input = fs::read_to_string(file).unwrap_or_else(|_| panic!("Unable to read {}", file));

    let mut stacks = Stacks::new();
    let stack_re = Regex::new(r"((\[.\]|   )(?: |\n))+").unwrap();
    let operation_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for l in input.split('\n') {
        if stack_re.is_match(l) {
            stacks.add_from_bottom(l);
            continue;
        }
    }
    let mut stacks_clone = stacks.clone();
    for l in input.split('\n') {
        if let Some(op) = operation_re.captures(l) {
            let from = op.get(2).unwrap().as_str().parse::<usize>().unwrap(); // unwrap as input should not be malformed
            let to = op.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let count = op.get(1).unwrap().as_str().parse::<usize>().unwrap();
            stacks.do_operation_9000(from, to, count);
            stacks_clone.do_operation_9001(from, to, count);
        }
    }

    println!("After rearrangement the top looks like {}", stacks.top_message());
    println!("After rearrangement the top looks really like {}", stacks_clone.top_message());
}
