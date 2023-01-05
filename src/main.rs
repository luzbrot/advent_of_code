use std::{fs, collections::LinkedList, fmt::Display};


/// Indicates the force a stone can be pushed
#[derive(Debug, Clone, Copy)]
enum Force {
    Left,
    Right
}
impl From<char> for Force {
    fn from(c: char) -> Self {
        match c {
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("Unknown force!")
        }
    }
}

/// Iterator over the forces (Puzzle input).
/// Will repeat for ever.
#[derive(Clone)]
struct ForceIterator {
    /// The forces defined by the input
    forces: Vec<Force>,
    /// The current state of the iterator
    state: usize
}
impl From<&str> for ForceIterator {
    fn from(s: &str) -> Self {
        let mut forces = Vec::new();
        for c in s.chars() {
            forces.push(c.into());
        }
        Self { forces, state: 0 }
    }
}
impl Iterator for ForceIterator {
    type Item = Force;
    
    fn next(&mut self) -> Option<Self::Item> {
        let item = Some(self.forces[self.state]);
        self.state = (self.state + 1) % self.forces.len();
        item
    }
}

/// Enum for a block of space 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Stone,
    Air
}
impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Space::Stone => write!(f, "#"),
            Space::Air => write!(f, ".")
        }
    }
}


#[derive(Clone)]
/// A stone with a shape
struct Stone {
    /// 2D shape of the the stone.
    /// [y][x]
    shape: Vec<Vec<Space>>
}
impl Stone {
    fn get_height(&self) -> usize {
        self.shape.len()
    }
    /// Returns
    /// ####
    fn h_line() -> Self {
        Stone {
            shape: vec![vec![Space::Stone; 4]]
        }
    }
    /// Returns
    /// .#.
    /// ###
    /// .#.
    fn cross() -> Self {
        Stone {
            shape: vec![
                vec![Space::Air, Space::Stone, Space::Air],
                vec![Space::Stone, Space::Stone, Space::Stone],
                vec![Space::Air, Space::Stone, Space::Air]
            ]
        }
    }
    /// Returns
    /// ..#
    /// ..#
    /// ###
    fn reverse_l() -> Self {
        Stone {
            shape: vec![
                vec![Space::Air, Space::Air, Space::Stone],
                vec![Space::Air, Space::Air, Space::Stone],
                vec![Space::Stone, Space::Stone, Space::Stone]
            ]
        }
    }
    /// Returns
    /// #
    /// #
    /// #
    /// #
    fn v_line() -> Self {
        Stone {
            shape: vec![vec![Space::Stone]; 4]
        }
    }
    /// Returns
    /// ##
    /// ##
    fn block() -> Self {
        Stone {
            shape: vec![vec![Space::Stone; 2]; 2]
        }
    }
    /// Returns a iterator over the stones in the correct order indefinitely
    fn stones() -> impl Iterator<Item = Stone> {
        let mut state = 4;
        std::iter::repeat_with(move || {
            state = (state + 1) % 5;
            match state {
                0 => Stone::h_line(),
                1 => Stone::cross(),
                2 => Stone::reverse_l(),
                3 => Stone::v_line(),
                4 => Stone::block(),
                _ => panic!("Should not happen!")
            }
        })
    }
}

/// The world the blocks will land in
struct World {
    /// The spaces in the world.
    /// Empty lines are to be assumed to be air
    /// [y][x]
    spaces: LinkedList<[Space; 7]>
}
impl World {
    /// New World
    fn new() -> Self {
        Self { spaces: LinkedList::new() }
    }
    /// Spawn new stones in the world
    /// 
    /// # Arguments
    /// * `stones` - Stones to spawn. Should at least yield `stone_count` stones
    /// * `forces` - Forces to apply to the falling stones. Should yield indefinitely
    /// * `stone_count`- The number of stones to spawn
    fn spawn_until(&mut self, stones: impl Iterator<Item = Stone>, mut forces: impl Iterator<Item = Force>, stone_count: usize) {
        for (count, stone) in stones.enumerate() {
            if count >= stone_count { break; }

            let mut stone_x = 2;
            let mut stone_y = -(3 + stone.get_height() as i32);

            loop {
                let force = forces.next().unwrap();
                match force {
                    Force::Left => {
                        if !self.does_stone_collide(&stone, stone_x - 1, stone_y) {
                            stone_x -= 1;
                        }
                    },
                    Force::Right => {
                        if !self.does_stone_collide(&stone, stone_x + 1, stone_y) {
                            stone_x += 1;
                        }
                    }
                }
                if !self.does_stone_collide(&stone, stone_x, stone_y + 1) {
                    stone_y += 1;
                }
                else {
                    break;
                }
            }

            self.add_stone(stone, stone_x, stone_y);
        } 
    }
    /// Checks if a Stone collides an a specified position with anything
    /// 
    /// # Arguments
    /// * `stone`- The stone to check
    /// * `x` - The x position of the stone
    /// * `y` - The y position of the stone
    fn does_stone_collide(&self, stone: &Stone, x: i32, y: i32) -> bool {
        for (stone_y, stone_row) in stone.shape.iter().enumerate() {
            let world_y = y + stone_y as i32;
            for (stone_x, stone_space) in stone_row.iter().enumerate() {
                if *stone_space == Space::Air { continue; }
                let world_x = x + stone_x as i32;
                if !(0..7).contains(&world_x) { return true } // outside of bounds

                if world_y < 0 { continue; }
                if let Some(world_row) = self.spaces.iter().nth(world_y as usize) {
                    if world_row[world_x as usize] != Space::Air {
                        return true
                    }
                }
                else {
                    // Below bedrock
                    return true
                }
            }
        }
        false
    }
    /// Add a stone to the world
    /// 
    /// # Arguments
    /// * `stone` - The stone to add
    /// * `x` - The x position of the stone
    /// * `y` - The y position of the stone
    fn add_stone(&mut self, stone: Stone, x: i32, mut y: i32) {
        while y < 0 {
            self.spaces.push_front([Space::Air; 7]);
            y += 1;
        }

        for (stone_y, stone_row) in stone.shape.iter().enumerate() {
            let world_y = y + stone_y as i32;
            for (stone_x, stone_space) in stone_row.iter().enumerate() {
                if *stone_space == Space::Air { continue; }
                let world_x = x + stone_x as i32;
                if !(0..7).contains(&world_x) { continue; } // outside of bounds
                if let Some(world_row) = self.spaces.iter_mut().nth(world_y as usize) {
                    world_row[world_x as usize] = *stone_space;
                }
                else {
                    // Below bedrock
                    continue;
                }
            }
        }
    }
    /// Returns the height of the stone tower in the world
    fn get_height_of_stones(&self) -> usize {
        self.spaces.len()
    }
}
impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = "".to_string();
        for row in self.spaces.iter() {
            for space in row.iter() {
                res += format!("{}", space).as_str();
            }
            res += "\n";
        }
        write!(f, "{}", res)
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap_or_else(|_| panic!("Unable to read input"));

    let forces: ForceIterator = input.as_str().into();
    let mut world = World::new();
    let stones = Stone::stones();
    world.spawn_until(stones, forces, 2022);

    println!("The tower of stones is {} height", world.get_height_of_stones());
}


#[cfg(test)]
mod tests {
    use crate::{ForceIterator, World, Stone};


    #[test]
    fn check_against_example() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        let forces: ForceIterator = input.into();
        let mut world = World::new();
        world.spawn_until(Stone::stones(), forces, 2022);
        assert_eq!(world.get_height_of_stones(), 3068);
    }
}