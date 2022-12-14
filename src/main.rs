use std::{fs, collections::{LinkedList, HashSet}};


/// A marker with definable needed unique characters
#[derive(Debug)]
struct Marker(LinkedList<char>, usize);
impl Marker {
    /// Create a new marker
    /// 
    /// # Arguments
    /// * `uniques` - The number of unique characters needed to be a full marker
    fn new(uniques: usize) -> Self {
        Self(LinkedList::new(), uniques)
    }
    /// Add a char to the marker.
    /// If there are more characters then there are needed for the check they are removed.
    /// 
    /// # Arguments
    /// * `c` - The char to add
    fn add(&mut self, c: char) {
        self.0.push_back(c);
        if self.0.len() > self.1 {
            self.0.pop_front();
        }
    }
    /// Check if all characters are unique in the marker.
    /// If not all characters are added jet false is returned.
    fn check_all_unique(&self) -> bool {
        if self.0.len() < self.1 {
            return false
        }
        let mut uniq = HashSet::new();
        self.0.iter().all(|el| uniq.insert(el))
    }
}

/// Rename a vec to impl functions for it
struct Message(Vec<char>);
impl Message {
    /// New Message from the puzzle input
    fn new(s: &str) -> Self {
        Self(s.chars().collect())
    }
    /// Return the position of the start marker (4 uniques)
    fn get_position_of_start_marker(&self) -> usize {
        self.get_position_of_marker(4)
    }
    /// Return the position of the message marker (14 uniques)
    fn get_position_of_message_marker(&self) -> usize {
        self.get_position_of_marker(14)
    }
    /// Return the position of a marker with defined uniques.
    /// 
    /// # Arguments
    /// * `uniques` - The number of uniques needed for the marker
    fn get_position_of_marker(&self, uniques: usize) -> usize {
        let mut marker = Marker::new(uniques);
        let mut iter = self.0.iter().enumerate();
        for _ in 0..(uniques-1) {
            marker.add(*iter.next().unwrap().1);
        }
        for (i,c) in iter {
            marker.add(*c);
            if marker.check_all_unique() {
                return i + 1
            }
        }
        0
    }
}

fn main() {

    let input = fs::read_to_string("input").unwrap_or_else(|_| panic!("Unable to read input"));

    let message = Message::new(input.as_str());
    println!("The start marker is at position {}", message.get_position_of_start_marker());
    println!("The message marker is at position {}", message.get_position_of_message_marker());
}


#[cfg(test)]
mod tests {
    use crate::Message;

    #[test]
    fn check_against_example() {
        let m1 = Message::new("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        let m2 = Message::new("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let m3 = Message::new("nppdvjthqldpwncqszvftbrmjlhg");
        let m4 = Message::new("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        let m5 = Message::new("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");

        assert_eq!(m1.get_position_of_start_marker(), 7);
        assert_eq!(m2.get_position_of_start_marker(), 5);
        assert_eq!(m3.get_position_of_start_marker(), 6);
        assert_eq!(m4.get_position_of_start_marker(), 10);
        assert_eq!(m5.get_position_of_start_marker(), 11);

        assert_eq!(m1.get_position_of_message_marker(), 19);
        assert_eq!(m2.get_position_of_message_marker(), 23);
        assert_eq!(m3.get_position_of_message_marker(), 23);
        assert_eq!(m4.get_position_of_message_marker(), 29);
        assert_eq!(m5.get_position_of_message_marker(), 26);
    }
}