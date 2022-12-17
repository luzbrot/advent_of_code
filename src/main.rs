use std::{fs, collections::{LinkedList, linked_list::Iter}};

/// Possible sub operations.
enum SubOperation {
    /// Add the number
    Addx(i32),
    /// Do nothing
    Noop
}

/// A renamed list of suboperations resulting in one operation.
/// Noop is noop, but addx is basically noop and add.
struct Operation(LinkedList<SubOperation>);
impl Operation {
    /// Create a iterator of the underlining list
    fn iter(&self) -> Iter<'_, SubOperation> {
        self.0.iter()
    }
}
impl From<&str> for Operation {
    /// From input line
    fn from(s: &str) -> Self {
        let mut list = LinkedList::new();
        if s.starts_with("addx") { // addx => noop + addx
            let mut parts = s.split(' ');
            list.push_back(SubOperation::Noop);
            list.push_back(SubOperation::Addx(parts.nth(1).unwrap().parse().unwrap()));
        }
        else if s.starts_with("noop") {
            list.push_back(SubOperation::Noop);
        }
        else {
            panic!("Unknown operation: {}", s)
        }
        Self(list)
    }
}

/// A cpu has multiple operations queued a start value for its register
struct Cpu {
    /// The start value of the register x
    x: i32,
    /// The queued operations
    ops: LinkedList<Operation>
}
impl Cpu {
    /// Create a new cpu with start value x = 1.
    /// The ops queue will be empty and need to be added with `add_ops_from_str`.
    fn new() -> Self {
        Self {
            x: 1,
            ops: LinkedList::new()
        }
    }
    /// Add queued operations to the cpu.
    /// 
    /// # Arguments
    /// * `ops` - Operations in each line (puzzle input)
    fn add_ops_from_str(&mut self, ops: &str) {
        for line in ops.split('\n') {
            self.ops.push_back(line.into());
        }
    }
    /// Calculate the expected signal strength to a given cycle without consuming the operations
    /// 
    /// # Arguments
    /// * `cycle` - The cycle to calculate for
    fn get_signal_strength_for_cycle(&self, cycle: usize) -> i32 {
        (self.x + self.ops.iter().flat_map(|el| el.iter()).take(cycle - 1).map(|el| -> i32 {
            if let SubOperation::Addx(x) = el {
                return *x
            }
            0
        }).sum::<i32>()) * (cycle as i32)
    }
    /// Draw the image resulting from the operations queued with out consuming them.
    /// 
    /// # Returns
    /// * String containing the image with \n for a new line
    fn draw_image(&self) -> String {
        let mut image = "".to_string();
        let iter = self.ops.iter().flat_map(|el| el.iter()).map(|el| -> i32 {
            if let SubOperation::Addx(x) = el {
                return *x
            }
            0
        });
        let mut x = self.x;
        for (i, dx) in iter.enumerate() {
            let pixel = (i % 40) as i32;
            if x == pixel || x + 1 == pixel || x - 1 == pixel {
                image += "#";
            }
            else {
                image += ".";
            }
            if pixel == 39 {
                image += "\n";
            }
            x += dx;
        }
        image
    }
}



fn main() {

    let input = fs::read_to_string("input").unwrap_or_else(|_| panic!("Unable to read input"));

    let mut cpu = Cpu::new();

    cpu.add_ops_from_str(&input);
    let sssum = cpu.get_signal_strength_for_cycle(20) + cpu.get_signal_strength_for_cycle(60) + cpu.get_signal_strength_for_cycle(100) + cpu.get_signal_strength_for_cycle(140) + cpu.get_signal_strength_for_cycle(180) + cpu.get_signal_strength_for_cycle(220);
    println!("The sum of the signal strengths is {}", sssum);
    println!("The image looks like:");
    println!("{}", cpu.draw_image());

}


#[cfg(test)]
mod tests {
    use crate::Cpu;


    #[test]
    fn check_against_example() {
        let mut cpu = Cpu::new();
        cpu.add_ops_from_str("addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop");

        assert_eq!(cpu.get_signal_strength_for_cycle(20), 420);
        assert_eq!(cpu.get_signal_strength_for_cycle(60), 1140);
        assert_eq!(cpu.get_signal_strength_for_cycle(100), 1800);
        assert_eq!(cpu.get_signal_strength_for_cycle(140), 2940);
        assert_eq!(cpu.get_signal_strength_for_cycle(180), 2880);
        assert_eq!(cpu.get_signal_strength_for_cycle(220), 3960);
        assert_eq!(cpu.draw_image(), "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
".to_string());
    }
}