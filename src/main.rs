use std::{fs, ops::{Add, AddAssign}};


/// The possible tile types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tiles {
    Air,
    Sand,
    Rock
}
impl Default for Tiles {
    fn default() -> Self {
        Tiles::Air
    }
}
impl Default for &Tiles {
    fn default() -> Self {
        &Tiles::Air
    }
}

/// A vector for the positions
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Vector {
    x: i32, 
    y: i32
}
impl Vector {
    /// Calculate the direction to an other vec under the assumption that directions
    /// are always straight vertical or horizontal (Puzzle description).
    /// 
    /// # Arguments
    /// * `to` - The other vector to calculate the direction to
    fn direction_to(&self, to: &Vector) -> Self {
        Self {
            x: (to.x - self.x).signum(),
            y: (to.y - self.y).signum()
        }
    }
}
impl From<&str> for Vector {
    fn from(s: &str) -> Self {
        let mut parts = s.split(',');
        Self {
            x: parts.next().unwrap().parse().unwrap(),
            y: parts.next().unwrap().parse().unwrap()
        }
    }
}
impl Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}
impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

/// A line with multiple segments. Each segment is from the previous position to the next.
struct Line(Vec<Vector>);
impl From<&str> for Line {
    fn from(s: &str) -> Self {
        let mut vec =  Vec::new();

        for part in s.split(" -> ") {
            vec.push(part.into());
        }

        Self(vec)
    }
}

/// A cave saving the tiles in a cave and the start position of the sand 
#[derive(Debug)]
struct Cave {
    /// The tiles in the cave.
    /// If a position is not set the default is assumed.
    /// This can lead to rows of just default tiles to be an empty Vec.
    tiles: Vec<Vec<Tiles>>,
    /// The start/spawn position of the sand
    sand_entry: Vector
}
impl Cave {
    /// Create a new cave with just default tiles
    fn new() -> Self {
        Self {
            tiles: Vec::new(),
            sand_entry: Vector { x: 500, y: 0 }
        }
    }
    /// Draw a line of rock in the cave
    /// 
    /// # Arguments
    /// * `line` - The line to draw with rock
    fn draw_line_of_rock(&mut self, line: &Line) {
        let lefts = line.0.iter().take(line.0.len() - 1);
        let rights = line.0.iter().skip(1);
        for line_part in lefts.zip(rights) {
            let dir = line_part.0.direction_to(line_part.1);
            let mut pos = *line_part.0;
            while pos != *line_part.1 {
                self.add_tile(Tiles::Rock, pos);
                pos += dir;
            }
            self.add_tile(Tiles::Rock, pos);
        }
    }
    /// Adds a tile in the cave of the specified type.
    /// Only needed positions are populated.
    /// 
    /// # Arguments
    /// * `tile` - The tile to add to the cave
    /// * `pos` - The position to add the tile to
    fn add_tile(&mut self, tile: Tiles, pos: Vector) {
        for _ in self.tiles.len()..(pos.y as usize + 1) {
            self.tiles.push(Vec::new());
        }
        for _ in self.tiles[pos.y as usize].len()..(pos.x as usize + 1) {
            self.tiles[pos.y as usize].push(Tiles::default());
        }
        self.tiles[pos.y as usize][pos.x as usize] = tile;
    }
    /// Spawn sand at the spawn position and find the position where it comes to rest.
    /// Does nothing if the spawn position is blocked.
    /// 
    /// # Returns
    /// Returns true if the sand comes to rest.
    /// Returns false if the sand will come not to rest or if the spawn position is blocked.
    fn spawn_sand(&mut self) -> bool {
        let mut pos = self.sand_entry;
        let sand_can_move = |pos: &mut Vector| -> bool {
            if let Some(col) = self.tiles.get(pos.y as usize + 1) {
                let down = col.get(pos.x as usize).unwrap_or_default();
                if *down == Tiles::Air {
                    pos.y += 1;
                    return true;
                }
                let down_left = col.get(pos.x as usize - 1).unwrap_or_default();
                if *down_left == Tiles::Air {
                    pos.y += 1;
                    pos.x -= 1;
                    return true;
                }
                let down_right = col.get(pos.x as usize + 1).unwrap_or_default();
                if *down_right == Tiles::Air {
                    pos.y += 1;
                    pos.x += 1;
                    return true;
                }
                false
            }
            else {
                pos.y += 1;
                true
            }
        };

        if *self.tiles[pos.y as usize].get(pos.x as usize).unwrap_or_default() == Tiles::Sand {
            // Entry blocked
            return false;
        }
        while sand_can_move(&mut pos) {
            if pos.y as usize >= self.tiles.len() {
                break;
            }
        }
        if !sand_can_move(&mut pos) {
            self.add_tile(Tiles::Sand, pos);
            true
        }
        else {
            false
        }
    }
    /// Add a floor two rows under the lowest rows in the cave.
    fn add_floor(&mut self) {
        let y = self.tiles.len() + 1;

        for x in 0..(self.sand_entry.x * 2) {
            self.add_tile(Tiles::Rock, Vector { x, y: y as i32});
        }
    }
}


fn main() {
    let input = fs::read_to_string("input").unwrap_or_else(|_| panic!("Unable to read input"));

    let mut cave = Cave::new();
    let mut cave_with_flor = Cave::new();

    for line in input.split('\n') {
        let line: Line = line.into();
        cave.draw_line_of_rock(&line);
        cave_with_flor.draw_line_of_rock(&line);
    }
    cave_with_flor.add_floor();

    let mut i = 0;
    while cave.spawn_sand() {
        i += 1;
    }
    println!("There {} units of sand at rest", i);

    i = 0;
    while cave_with_flor.spawn_sand() {
        i += 1;
    }
    println!("There are {} units sand until the entry is blocked", i);
}


#[cfg(test)]
mod tests {
    use crate::{Cave, Line};

    #[test]
    fn check_against_example() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        let mut cave = Cave::new();
        let mut cave_with_floor = Cave::new();

        for line in input.split('\n') {
            let line: Line = line.into();
            cave.draw_line_of_rock(&line);
            cave_with_floor.draw_line_of_rock(&line);
        }

        cave_with_floor.add_floor();

        let mut i = 0;
        while cave.spawn_sand() {
            i += 1;
        }
        assert_eq!(i, 24);

        i = 0;
        while cave_with_floor.spawn_sand() {
            i += 1;
        }
        assert_eq!(i, 93);
    }
}