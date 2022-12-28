use std::{fs, ops::{Add, Sub, Div, AddAssign, Mul}, iter::Sum};
use regex::Regex;
#[macro_use]
extern crate lazy_static;
use itertools::Itertools;

/// A Position
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: i64,
    y: i64
}
impl Position {
    /// Calculate the manhattan distance to an other position
    /// 
    /// #Arguments
    /// * `other` - The other position to calculate to
    fn manhattan_distance(&self, other: &Self) -> i64 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }
    /// Calculate the tuning frequency of a position
    fn determine_tuning_frequency(&self) -> i64 {
        self.x * 4000000 + self.y
    }
}
impl From<&str> for Position {
    fn from(s: &str) -> Self {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"x=([-\d]+), y=([-\d]+)").unwrap();
        }
        let cap = RE.captures(s).unwrap();
        Self {
            x: cap.get(1).unwrap().as_str().parse().unwrap(),
            y: cap.get(2).unwrap().as_str().parse().unwrap()
        }
    }
}
impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}
impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}
impl Div<i64> for Position {
    type Output = Self;
    fn div(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}
impl Mul<i64> for Position {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}
impl Sum for Position {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut res = Position {x: 0, y: 0};
        for i in iter {
            res += i;
        }
        res
    }
}

/// A sensor with its nearest found beacon
struct Sensor {
    /// The position of the beacon
    position: Position,
    /// The closest beacon to the sensor
    closest_beacon: Position
}
impl Sensor {
    /// Get the coverage of the sensor for a specific y coordinate
    /// 
    /// # Arguments
    /// * `y` - The y coordinate to get the coverage for
    /// 
    /// # Returns
    /// A vector with all covered positions
    fn get_coverage_for(&self, y: i64) -> Vec<Position> {
        let mut coverage = Vec::new();

        let dis = self.position.manhattan_distance(&self.closest_beacon);

        if (y - self.position.y).abs() > dis {
            return coverage
        }

        let mut pos = self.position;
        pos.y = y;
        for d in -dis..=dis {
            pos.x = self.position.x + d;
            if self.position.manhattan_distance(&pos) <= dis {
                coverage.push(pos);
            }
        }

        coverage
    }
    /// Check if a position is covered from the sensor
    /// 
    /// # Arguments
    /// * `pos` - The position to check
    fn is_covered(&self, pos: &Position) -> bool {
        self.position.manhattan_distance(pos) <= self.position.manhattan_distance(&self.closest_beacon)
    }
    /// Get all positions directly outside of the coverage border
    fn get_outside_border(&self) -> Vec<Position> {
        let mut res = Vec::new();

        let dis = self.position.manhattan_distance(&self.closest_beacon) + 1;
        for dx in -dis..=dis {
            res.push(Position {
                x: self.position.x + dx,
                y: self.position.y - (dis - dx.abs())
            });
            if dis - dx.abs() != 0 {
                res.push(Position {
                    x: self.position.x + dx,
                    y: self.position.y + (dis - dx.abs())
                });
            }
        }

        res
    }
}
impl From<&str> for Sensor {
    fn from(s: &str) -> Self {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"Sensor at (x=[-\d]+, y=[-\d]+): closest beacon is at (x=[-\d]+, y=[-\d]+)").unwrap();
        }
        let cap = RE.captures(s).unwrap();
        Self {
            position: cap.get(1).unwrap().as_str().into(),
            closest_beacon: cap.get(2).unwrap().as_str().into()
        }
    }
}

/// Renamed sensor array
struct Sensors(Vec<Sensor>);
impl Sensors {
    /// Count the covered areas for all sensor on a specific y coordinate
    /// 
    /// # Arguments
    /// * `y` - The get the coverage for
    fn get_coverage_count_for(&self, y: i64) -> usize {
        let beacons = self.0.iter().map(|el| el.closest_beacon).collect::<Vec<Position>>();
        self.0.iter().flat_map(|el| el.get_coverage_for(y)).unique_by(|el| el.x).filter(|el| !beacons.iter().any(|e| e == el)).count()
    }
    /// Checks if a position is covered from any sensor
    /// 
    /// # Arguments
    /// * `pos` - The position to check
    fn is_covered(&self, pos: &Position) -> bool {
        self.0.iter().any(|el| el.is_covered(pos))
    }
    /// Find a distress signal in an square area inside the specified position
    /// 
    /// # Arguments
    /// * `from` - The start position of the square
    /// * `to` - The inclusive end position of the square
    fn find_distress_from_to(&self, from: &Position, to: &Position) -> Option<Position> {
        self.0.iter().flat_map(|el| el.get_outside_border()).filter(|el| el.x >= from.x && el.x <= to.x && el.y >= from.y && el.y <= to.y).find(|el| !self.is_covered(el))
    }
}
impl From<&str> for Sensors {
    fn from(s: &str) -> Self {
        let mut vec = Vec::new();
        for l in s.split('\n') {
            vec.push(l.into());
        }
        Self(vec)
    }
}


fn main() {
    let input = fs::read_to_string("input").unwrap_or_else(|_| panic!("Unable to read input"));
    
    let sensors: Sensors = input.as_str().into();
    println!("There are {} positions which can not contain a beacon", sensors.get_coverage_count_for(2000000));
    println!("The tuning frequency for the distress signal is {}", sensors.find_distress_from_to(&Position { x: 0, y: 0 }, &Position { x: 4000000, y: 4000000 }).unwrap().determine_tuning_frequency())
}


#[cfg(test)]
mod tests {
    use crate::Sensors;


    #[test]
    fn check_against_example() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        let sensors: Sensors = input.into();
        assert_eq!(sensors.get_coverage_count_for(10), 26);
        assert_eq!(sensors.find_distress_from_to(&crate::Position { x: 0, y: 0 }, &crate::Position { x: 20, y: 20 }).unwrap().determine_tuning_frequency(), 56000011);
    }
}