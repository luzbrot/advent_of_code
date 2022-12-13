use clap::Parser;
use std::{fs, ops::{Deref, DerefMut}};


/// CLI Input
#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
struct Args {
    /// If set the test data is loaded
    #[arg(long)]
    test: bool,
}


/// Rename std::ops::Range<u32> to impl traits and new functions for it
#[derive(Debug)]
struct Range(std::ops::Range<u32>);
impl Range {
    /// Create a new Range from the input part.
    /// Gets split on '-' and used as start and end.
    /// 
    /// # Arguments
    /// * `s` - The part of the input string
    fn new(s: &str) -> Self {
        let mut parts = s.split('-');
        Self(std::ops::Range{
            start: parts.next().unwrap().parse::<u32>().unwrap(), // Just unwrap as there is an error the input is malformed
            end: parts.next().unwrap().parse::<u32>().unwrap() + 1 // We need to add one as the range is defined as exclusive
        })
    }
}
impl Range {
    /// Checks if the other range is contained in this range
    /// 
    /// # Arguments
    /// * `other` - The other range to check for
    fn contains_range(&self, other: &Self) -> bool {
        let end = other.end - 1; // -1 as we added +1 because exclusive end
        self.contains(&other.start) && self.contains(&end)
    }
    /// Checks if this other range is overlapped by the other range
    /// 
    /// # Arguments
    /// * `other` - The other range to check for
    fn partially_contains_range(&self, other: &Self) -> bool {
        let end = other.end - 1; // -1 as we added +1 because exclusive end
        self.contains(&other.start) || self.contains(&end)
    }
}
/// Impl Deref to use Range as std::ops::Range
impl Deref for Range {
    type Target = std::ops::Range<u32>;
    fn deref(&self) -> &std::ops::Range<u32> { &self.0 }
}
impl DerefMut for Range {
    fn deref_mut(&mut self) -> &mut std::ops::Range<u32> { &mut self.0 }
}

/// A Pair of two ranges from the assignment.
#[derive(Debug)]
struct Pair(Range, Range);
impl Pair {
    /// Create a new assignment pair from the input.
    /// 
    /// # Arguments
    /// * `s` - A line from the input
    fn new(s: &str) -> Self {
        let mut parts = s.split(',');
        Self(Range::new(parts.next().unwrap()), Range::new(parts.next().unwrap()))
    }
    /// Check if the ranges of the assignment overlap completely
    fn assignment_overlaps_completely(&self) -> bool {
        self.0.contains_range(&self.1) || self.1.contains_range(&self.0)
    }
    /// Check if the ranges of the assignment overlap partially
    fn assignment_overlaps_partially(&self) -> bool {
        self.0.partially_contains_range(&self.1) || self.1.partially_contains_range(&self.0)
    }
}

fn main() {
    let args = Args::parse();

    let file = if args.test { "test_input" } else { "input" };

    let input = fs::read_to_string(file).unwrap_or_else(|_| panic!("Unable to read {}", file));

    let mut pairs: Vec<Pair> = Vec::new();

    for p in input.split('\n') {
        pairs.push(Pair::new(p));
    }

    let overlaps = pairs.iter().filter(|el| el.assignment_overlaps_completely()).count();
    println!("There a {} fully overlaps", overlaps);

    let partly_overlaps = pairs.iter().filter(|el| el.assignment_overlaps_partially()).count();
    println!("There a {} partly overlaps", partly_overlaps);
}
