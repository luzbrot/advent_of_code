use std::{fs, cmp::Ordering};


/// A Packet can be a value or a list of Packets
#[derive(Debug, Clone)]
enum Packet {
    Value(u32),
    List(Vec<Packet>)
}
impl Packet {
    /// Promote A Value to a List containing this Value.
    /// A List will be returned as is
    fn promote(&self) -> Self {
        match self {
            Self::List(_) => self.clone(),
            Self::Value(_) => Self::List(vec![self.clone()])
        }
    }
}
impl From<&str> for Packet {
    fn from(s: &str) -> Self {
        if s.starts_with('[') {
            let mut vec = Vec::new();
            let mut stripped = s.strip_prefix('[').unwrap().strip_suffix(']').unwrap();
            while !stripped.is_empty() {
                let iter = stripped.chars().enumerate();
                let mut parsing_list = 0;
                let mut done_something = false;
                for (i, c) in iter {
                    if c == ',' {
                        if parsing_list > 0 {continue;}
                        vec.push(stripped[0..i].into());
                        if i + 1 < stripped.len() {
                            stripped = &stripped[(i+1)..];
                        }
                        else {
                            stripped = "";
                        }
                        done_something = true;
                        break;
                    }
                    else if c == '[' {
                        parsing_list += 1;
                    }
                    else if c == ']' {
                        parsing_list -= 1;
                        if parsing_list == 0 {
                            vec.push(stripped[0..(i+1)].into());
                            if i + 2 < stripped.len() {
                                stripped = &stripped[(i+2)..];
                            }
                            else {
                                stripped = "";
                            }
                            done_something = true;
                            break;
                        }
                    }
                }
                if !done_something {
                    vec.push(stripped.into());
                    break;
                }
            }
            Packet::List(vec)
        }
        else {
            Packet::Value(s.parse().unwrap())
        }
    }
}

impl Eq for Packet {}
impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Packet::List(list) => {
                match other {
                    Packet::List(other_list) => {
                        let iter = list.iter().zip(other_list.iter());
                        let mut skipped_iter = iter.skip_while(|el| el.0 == el.1);
                        if let Some(el) = skipped_iter.next() {
                            el.0.eq(el.1)
                        }
                        else {
                            list.len() == other_list.len()
                        }
                        
                    },
                    Packet::Value(_) => {
                        self.eq(&other.promote())
                    }
                }
            },
            Packet::Value(val) => {
                match other {
                    Packet::List(_) => {
                        self.promote().eq(other)
                    },
                    Packet::Value(other_val) => {
                        val.eq(other_val)
                    }
                }
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Packet::List(list) => {
                match other {
                    Packet::List(other_list) => {
                        let iter = list.iter().zip(other_list.iter());
                        let mut skipped_iter = iter.skip_while(|el| el.0 == el.1);
                        if let Some(el) = skipped_iter.next() {
                            el.0.cmp(el.1)
                        }
                        else if list.len() < other_list.len() {
                            Ordering::Less
                        }
                        else if list.len() > other_list.len() {
                            Ordering::Greater
                        }
                        else {
                            Ordering::Equal
                        }
                        
                    },
                    Packet::Value(_) => {
                        self.cmp(&other.promote())
                    }
                }
            },
            Packet::Value(val) => {
                match other {
                    Packet::List(_) => {
                        self.promote().cmp(other)
                    },
                    Packet::Value(other_val) => {
                        val.cmp(other_val)
                    }
                }
            }
        }
    }
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A pair of two Packets
#[derive(Debug)]
struct Pair(Packet, Packet);
impl Pair {
    /// Check if the two packets a correctly ordered (first < second)
    fn is_correctly_ordered(&self) -> bool {
        self.0 < self.1
    }
}
impl From<&str> for Pair {
    fn from(s: &str) -> Self {
        let mut parts = s.split('\n');
        Self(
            parts.next().unwrap().into(),
            parts.next().unwrap().into()
        )
    }
}

/// Pairs a renamed vec of pairs
#[derive(Debug)]
struct Pairs(Vec<Pair>);
impl Pairs {
    /// Part 1
    fn get_sum_of_index_of_correctly_ordered_packets(&self) -> usize {
        self.0.iter().enumerate().filter(|el| el.1.is_correctly_ordered()).map(|el| el.0 + 1).sum()
    }
    /// Part 2
    /// Adds a Pair to the vec.
    fn get_decoder_key(&mut self) -> usize {
        let first_divider: Packet = "[[2]]".into();
        let second_divider: Packet = "[[6]]".into();
        self.0.push(Pair(first_divider.clone(), second_divider.clone()));

        let mut packets: Vec<Packet> = self.0.iter().flat_map(|el| vec![el.0.clone(), el.1.clone()]).collect();
        packets.sort();

        packets.iter().enumerate().filter(|el| el.1 == &first_divider || el.1 == &second_divider).map(|el| el.0 + 1).product()
    }
}
impl From<&str> for Pairs {
    fn from(s: &str) -> Self {
        let mut vec = Vec::new();
        let parts = s.split("\n\n");
        for p in parts {
            vec.push(p.into());
        }

        Self(vec)
    }
}


fn main() {
    let input = fs::read_to_string("input").unwrap_or_else(|_| panic!("Unable to read input"));

    let mut pairs: Pairs = input.as_str().into();
    println!("The sum of indexes is {}", pairs.get_sum_of_index_of_correctly_ordered_packets());
    println!("The decoder key is {}", pairs.get_decoder_key());
}


#[cfg(test)]
mod tests {
    use crate::{Pairs, Pair};


    #[test]
    fn check_against_example() {
        let p1: Pair = "[[1]]
[1]".into();
        assert!(p1.0 == p1.1);
        let p2: Pair = "[[1],4]
[1,[2,[3,[4,[5,6,0]]]],8,9]".into();
        assert!(!p2.is_correctly_ordered());


        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        let mut pairs: Pairs = input.into();
        assert_eq!(pairs.get_sum_of_index_of_correctly_ordered_packets(), 13);
        assert_eq!(pairs.get_decoder_key(), 140);
    }
}