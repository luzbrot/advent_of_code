use std::ops::{Sub, AddAssign};
use std::{fs, collections::LinkedList};
use itertools::Itertools;


/// Position struct to indicate rope parts positions
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32
}
impl Position {
    /// Check if this position is touching a other position as defined
    fn is_touching(&self, other: &Position) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
}
/// Impl Sub to be able to do a - b
impl Sub for Position {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}
// Impl AddAssign to be able to do a += b
impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

/// Rope struct to track rope state
struct Rope {
    /// The head of the rope
    head: Position,
    /// The parts of the rope (can be 0)
    parts: Vec<Position>,
    /// The tail of the rope
    tail: Position,
    /// List of visited places of the tail of the rope
    visited: LinkedList<Position>
}
impl Rope {
    /// Create a new rope with a defined number of parts.
    /// 
    /// # Arguments
    /// * `parts` - The number of parts
    fn new(parts: u32) -> Self {
        let mut visited = LinkedList::new();
        visited.push_back(Position{x: 0, y: 0});
        let mut rope_parts = Vec::new();
        for _ in 0..parts {
            rope_parts.push(Position{x: 0, y: 0});
        }
        Self {
            head: Position { x: 0, y: 0 },
            parts: rope_parts,
            tail: Position { x: 0, y: 0 },
            visited
        }
    }
    /// Update the tail position if needed and safe the new tail position.
    /// Also update the parts.
    fn update_tail(&mut self) {
        self.update_parts();
        let reference = if !self.parts.is_empty() {*self.parts.last().unwrap()} else {self.head};
        if self.tail.is_touching(&reference) {
            return;
        }
        let mut dif = reference - self.tail;
        dif.x = dif.x.signum();
        dif.y = dif.y.signum();
        self.tail += dif;
        self.visited.push_back(self.tail);
    }
    /// Update all part positions if needed
    fn update_parts(&mut self) {
        let mut reference = self.head;
        for part in self.parts.iter_mut() {
            if part.is_touching(&reference) {
                return;
            }
            let mut dif = reference - *part;
            dif.x = dif.x.signum();
            dif.y = dif.y.signum();
            *part += dif;
            reference = *part;
        }
    }
    /// Move the head up n times and update the tail
    fn move_up(&mut self, n: u32) {
        for _ in 0..n {
            self.head.y += 1;
            self.update_tail();
        }
    }
    /// Move the head down n times and update the tail
    fn move_down(&mut self, n: u32) {
        for _ in 0..n {
            self.head.y -= 1;
            self.update_tail();
        }
    }
    /// Move the head right n times and update the tail
    fn move_right(&mut self, n: u32) {
        for _ in 0..n {
            self.head.x += 1;
            self.update_tail();
        }
    }
    /// Move the head left n times and update the tail
    fn move_left(&mut self, n: u32) {
        for _ in 0..n {
            self.head.x -= 1;
            self.update_tail();
        }
    }
    /// Count the unique positions in the visited places of the tail
    fn count_visited_places(&self) -> usize {
        self.visited.iter().unique().count()
    }
    /// Move the head according to a sequence (puzzle input)
    /// 
    /// # Arguments
    /// * `input` - The sequence to move the head
    fn move_seq(&mut self, input: &str) {
        for line in input.split('\n') {
            let mut parts = line.split(' ');
            match parts.next().unwrap() {
                "U" => self.move_up(parts.next().unwrap().parse().unwrap()),
                "D" => self.move_down(parts.next().unwrap().parse().unwrap()),
                "R" => self.move_right(parts.next().unwrap().parse().unwrap()),
                "L" => self.move_left(parts.next().unwrap().parse().unwrap()),
                _ => panic!("Malformed input")
            }
        }
    }
}

fn main() {

    let input = fs::read_to_string("input").unwrap_or_else(|_| panic!("Unable to read input"));

    let mut rope = Rope::new(0);
    rope.move_seq(&input);
    println!("There were {} places visited.", rope.count_visited_places());
    rope = Rope::new(8);
    rope.move_seq(&input);
    println!("There were {} places visited by the bigger tope.", rope.count_visited_places());
}


#[cfg(test)]
mod tests {
    use crate::Rope;


    #[test]
    fn check_against_example() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let mut rope = Rope::new(0);
        rope.move_seq(input);
        assert_eq!(rope.count_visited_places(), 13);

        rope = Rope::new(8);
        rope.move_seq(input);
        assert_eq!(rope.count_visited_places(), 1);

        rope = Rope::new(8);
        rope.move_seq("R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20");
    assert_eq!(rope.count_visited_places(), 36);
    }
}