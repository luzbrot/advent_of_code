use std::{fs, collections::LinkedList, ops::{Deref, DerefMut}};
use regex::Regex;


/// Possible operators for the worry modifier
enum WorryModifierOp {
    /// *
    Multi,
    /// +
    Plus
}
impl From<&str> for WorryModifierOp {
    fn from(s: &str) -> Self {
        match s {
            "*" => Self::Multi,
            "+" => Self::Plus,
            _ => panic!("Unknown operator")
        }
    }
}
/// Possible values for the worry modifier
enum WorryModifierValue {
    /// The item to modify the worry for
    Item,
    /// A value
    Value(i64),
}
impl From<&str> for WorryModifierValue {
    fn from(s: &str) -> Self {
        match s {
            "old" => Self::Item,
            _ => Self::Value(s.parse::<i64>().unwrap()),
        }
    }
}
/// A worry modifier to modify the worry level of an item
struct WorryModifier {
    /// Left side of the worry modification
    left_value: WorryModifierValue,
    /// The operation of the worry modification
    op: WorryModifierOp,
    /// The right side of the worry modification
    right_value: WorryModifierValue
}
impl WorryModifier {
    /// Modify the worry level of an item
    /// 
    /// # Arguments
    /// * `item` - The item to modify
    fn modify(&self, item: &mut Item) {
        let left = match self.left_value {
            WorryModifierValue::Item => item.0,
            WorryModifierValue::Value(v) => v
        };
        let right = match self.right_value {
            WorryModifierValue::Item => item.0,
            WorryModifierValue::Value(v) => v
        };
        item.0 = match self.op {
            WorryModifierOp::Multi => left * right,
            WorryModifierOp::Plus => left + right
        };
    }
}
impl From<&str> for WorryModifier {
    fn from(s: &str) -> Self {
        let mut parts = s.split(' '); 
        Self {
            left_value: parts.next().unwrap().into(),
            op: parts.next().unwrap().into(),
            right_value: parts.next().unwrap().into()
        }
    }
}
/// An item containing a worry level
struct Item(i64);
impl Item {
    /// Relief the worry level
    /// 
    /// # Arguments
    /// * `relief` - The relief to divide the level with
    /// * `module` - The module to use on the worry level (solution for part 2, but obvious should not hinder part 1)
    fn relief(&mut self, relief: i64, modulo: i64) {
        self.0 /= relief;
        self.0 %= modulo;
    }
    /// Checks if the worry level is dividable by a divisor
    /// 
    /// # Arguments
    /// - `divisor` - The divisor to check
    fn dividable(&self, divisor: i64) -> bool {
        self.0 % divisor == 0
    }
}
/// Renamed Item list for function implementation
struct Items(LinkedList<Item>);
impl Deref for Items {
    type Target = LinkedList<Item>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Items {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<&str> for Items {
    fn from(s: &str) -> Self {
        let mut ll = LinkedList::new();
        for i in s.split(", ") {
            ll.push_back(Item(i.parse::<i64>().unwrap()));
        }
        Self(ll)
    }
}
/// Implementation of a monkey
struct Monkey {
    /// The items the monkey is holding
    items: Items,
    /// The worry modifier for that monkey
    worry_modifier: WorryModifier,
    /// The divisor to decide where to throw the item
    divisor: i64,
    /// The index of the monkey to throw to if the check succeeds
    true_monkey: usize,
    /// The index of the monkey to throw to if the check does not succeed
    false_monkey: usize,
    /// Counter of how many items the monkey inspected
    counter: usize,
    /// The relief value after the monkey is done inspecting
    relief: i64,
    /// The modulo after the monkey is done inspection (Solution for part 2)
    modulo: i64
}
impl Monkey {
    /// Check if the monkey has items
    fn has_items(&self) -> bool {
        !self.items.is_empty()
    }
    /// Process one item of the monkey
    /// 
    /// # Returns
    /// Tuple of the index of the monkey the item is thrown to and the Item
    fn process_item(&mut self) -> (usize, Item) {
        let mut item = self.items.pop_front().unwrap();
        self.worry_modifier.modify(&mut item);
        item.relief(self.relief, self.modulo);
        self.counter += 1;
        if item.dividable(self.divisor) {
            (self.true_monkey, item)
        }
        else {
            (self.false_monkey, item)
        }
    }
    /// Receive an item
    /// 
    /// # Arguments
    /// * `item` - The item to receive
    fn receive_item(&mut self, item: Item) {
        self.items.push_back(item);
    }
}
impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let re = Regex::new(r"Monkey (\d):\s+Starting items: ([\d, ]+)\s+Operation: new = (.+?)\s+Test: divisible by (\d+)\s+If true: throw to monkey (\d+)\s+If false: throw to monkey (\d+)").unwrap();
        let cap = re.captures(s).unwrap();
        Self {
            items: cap.get(2).unwrap().as_str().into(),
            worry_modifier: cap.get(3).unwrap().as_str().into(),
            divisor: cap.get(4).unwrap().as_str().parse::<i64>().unwrap(),
            true_monkey: cap.get(5).unwrap().as_str().parse::<usize>().unwrap(),
            false_monkey: cap.get(6).unwrap().as_str().parse::<usize>().unwrap(),
            counter: 0,
            relief: 3,
            modulo: i64::MAX
        }
    }
}

/// Rename of Vec with monkey to impl functions
struct Monkeys(Vec<Monkey>);
impl Monkeys {
    /// Do a round of monkey game
    fn do_round(&mut self) {
        for i in 0..self.0.len() {
            while self.0[i].has_items() {
                let (to, item) = self.0[i].process_item();
                self.0[to].receive_item(item);
            }
        }
    }
    /// Calculate the level of monkey business
    fn get_level_of_monkey_business(&self) -> usize {
        let mut counters = self.0.iter().map(|el| el.counter).collect::<Vec<usize>>();
        counters.sort();
        counters.reverse();
        counters[0] * counters[1]
    }
}
impl From<&str> for Monkeys {
    fn from(s: &str) -> Self {
        let mut monkeys: Vec<Monkey> = Vec::new();
        for m in s.split("\n\n") {
            monkeys.push(m.into());
        }
        let modulo: i64 = monkeys.iter().map(|el| el.divisor).product();
        monkeys.iter_mut().for_each(|el| el.modulo = modulo);
        Self(monkeys)
    }
}


fn main() {

    let input = fs::read_to_string("input").unwrap_or_else(|_| panic!("Unable to read input"));

    let mut monkeys: Monkeys = input.as_str().into();
    for _ in 0..20 {
        monkeys.do_round();
    }
    println!("The level of monkey business is after 20 rounds {}", monkeys.get_level_of_monkey_business());

    monkeys = input.as_str().into();
    monkeys.0.iter_mut().for_each(|el| el.relief = 1);
    for _ in 0..10000 {
        monkeys.do_round();
    }
    println!("The level of monkey business is after 10000 rounds without relief is {}", monkeys.get_level_of_monkey_business());
}


#[cfg(test)]
mod tests {
    use crate::Monkeys;

    #[test]
    fn check_against_example() {
        let input = "Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        let mut monkeys: Monkeys = input.into();
        for _ in 0..20 {
            monkeys.do_round();
        }
        assert_eq!(monkeys.0.iter().map(|el| el.counter).collect::<Vec<usize>>(), [101, 95, 7, 105]);
        assert_eq!(monkeys.get_level_of_monkey_business(), 10605);

        monkeys = input.into();
        monkeys.0.iter_mut().for_each(|el| el.relief = 1);
        for i in 0..10000 {
            monkeys.do_round();
            if i + 1 == 1 {
                assert_eq!(monkeys.0.iter().map(|el| el.counter).collect::<Vec<usize>>(), [2, 4, 3, 6]);
            }
            else if i + 1 == 20 {
                assert_eq!(monkeys.0.iter().map(|el| el.counter).collect::<Vec<usize>>(), [99, 97, 8, 103]);
            }
            else if i + 1 == 1000 {
                assert_eq!(monkeys.0.iter().map(|el| el.counter).collect::<Vec<usize>>(), [5204, 4792, 199, 5192]);
            }
            else if i + 1 == 2000 {
                assert_eq!(monkeys.0.iter().map(|el| el.counter).collect::<Vec<usize>>(), [10419, 9577, 392, 10391]);
            }
            else if i + 1 == 3000 {
                assert_eq!(monkeys.0.iter().map(|el| el.counter).collect::<Vec<usize>>(), [15638, 14358, 587, 15593]);
            }
            else if i + 1 == 4000 {
                assert_eq!(monkeys.0.iter().map(|el| el.counter).collect::<Vec<usize>>(), [20858, 19138, 780, 20797]);
            }
            else if i + 1 == 5000 {
                assert_eq!(monkeys.0.iter().map(|el| el.counter).collect::<Vec<usize>>(), [26075, 23921, 974, 26000]);
            }
            else if i + 1 == 6000 {
                assert_eq!(monkeys.0.iter().map(|el| el.counter).collect::<Vec<usize>>(), [31294, 28702, 1165, 31204]);
            }
            else if i + 1 == 7000 {
                assert_eq!(monkeys.0.iter().map(|el| el.counter).collect::<Vec<usize>>(), [36508, 33488, 1360, 36400]);
            }
            else if i + 1 == 8000 {
                assert_eq!(monkeys.0.iter().map(|el| el.counter).collect::<Vec<usize>>(), [41728, 38268, 1553, 41606]);
            }
            else if i + 1 == 9000 {
                assert_eq!(monkeys.0.iter().map(|el| el.counter).collect::<Vec<usize>>(), [46945, 43051, 1746, 46807]);
            }
            else if i + 1 == 10000 {
                assert_eq!(monkeys.0.iter().map(|el| el.counter).collect::<Vec<usize>>(), [52166, 47830, 1938, 52013]);
            }
        }
        assert_eq!(monkeys.get_level_of_monkey_business(), 2713310158);

    }
}