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


/// Calories of an elf
#[derive(Debug)]
struct Calories {
    /// The calories of the items he is carrying
    values: Vec<u32>,
    sum: u32
}

impl Calories {
    /// Create a new Calories object
    fn new() -> Self {
        Self {
            values: Vec::new(),
            sum: 0
        }
    }
    /// Add calories to the object.
    /// Adds to the vector of values and adds to the sum.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to add
    fn add(&mut self, value: u32) {
        self.values.push(value);
        self.sum += value;
    }
}

fn main() {
    let args = Args::parse();

    let file = if args.test { "test_input" } else { "input" };

    let input = fs::read_to_string(file).unwrap_or_else(|_| panic!("Unable to read {}", file));

    let mut elfs: Vec<Calories> = Vec::new();

    // Split on empty lines which are just two line breaks
    for elf in input.split("\n\n") {
        // Add every line to the calories
        let cs = elf.split('\n');
        let mut cl = Calories::new();
        for c in cs {
            cl.add(c.parse::<u32>().unwrap());
        }
        elfs.push(cl);
    }

    // Sort calories per sum descending
    elfs.sort_by(|a, b| a.sum.cmp(&b.sum).reverse());

    println!("The most calories one elf is carring is: {}", elfs[0].sum);

    // Sum the first three elements of the vec
    let sum_top_three: u32 = elfs[0..3].iter().map(|a| a.sum).sum();
    println!("The calories of the three top elfs is: {}", sum_top_three);

}
