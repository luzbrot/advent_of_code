use std::fs;

/// A Hill holding the terrain (input)
struct Hill {
    /// The terrain of the hill (Height from 0 to 27)
    terrain: Vec<Vec<usize>>,
    /// The start x, y position
    start: (usize, usize),
    /// The end x, y position
    end: (usize, usize)
}
impl Hill {
    /// Get the number of steps needed for the shortest route from start to end
    fn get_number_of_steps_for_shortest_route_from_start(&self) -> usize {
        self.get_number_of_steps_for_shortest_route(self.start)
    }
    /// Get the most scenic route from height <=1 to end.
    /// This could be optimized, but it is just a puzzle^^
    fn get_number_of_steps_for_shortest_scenic_route(&self) -> usize {
        let mut start_indexes = Vec::new();
        for (y, row) in self.terrain.iter().enumerate() {
            for (x, field) in row.iter().enumerate() {
                if *field <= 1 {
                    start_indexes.push((x, y));
                }
            }
        }
        start_indexes.iter().map(|el| self.get_number_of_steps_for_shortest_route(*el)).min().unwrap()
    }
    /// Get the shortest number of steps from a defined start to the end.
    /// Uses Dijkstra algorithm to calculate the route.
    /// 
    /// # Arguments
    /// * `start` - The x, y position  of the start
    fn get_number_of_steps_for_shortest_route(&self, start: (usize, usize)) -> usize {
        let rows = self.terrain.len();
        let columns = self.terrain[0].len();

        let nodes = rows * columns;
        let start_index = start.0 + start.1 * columns;
        let end_index = self.end.0 + self.end.1 * columns;
        let mut distances: Vec<(usize, usize, usize, bool)> = Vec::new(); // (index, distance, prev node, done)
        for i in 0..nodes {
            distances.push((i, if i == start_index {
                0
            }
            else {
                usize::MAX
            }, i, false));
        }

        loop {
            let opt_node = distances.iter().filter(|el| !el.3).min_by(|a, b| a.1.cmp(&b.1)).copied();
            if let Some(node) = opt_node {
                if node.0 == end_index { break; }
                distances[node.0].3 = true;
                let x = node.0 % columns;
                let y = node.0 / columns;
                let current_height = self.terrain[y][x];
                let next_distance = node.1 + if node.1 == usize::MAX { 0 } else { 1 };
                if let Some(right_height) = self.terrain[y].get(x + 1) {
                    if *right_height <= current_height + 1 {
                        let i = x + 1 + y * columns;
                        if distances[i].1 > next_distance {
                            distances[i].1 = next_distance;
                            distances[i].2 = node.0;
                        }
                    }
                }
                if x > 0 {
                    if let Some(left_height) = self.terrain[y].get(x - 1) {
                        if *left_height <= current_height + 1 {
                            let i = x - 1 + y * columns;
                            if distances[i].1 > next_distance {
                                distances[i].1 = next_distance;
                                distances[i].2 = node.0;
                            }
                        }
                    }
                }
                if y > 0 {
                    if let Some(up_row) = self.terrain.get(y - 1) {
                        let up_height = up_row[x];
                        if up_height <= current_height + 1 {
                            let i = x + (y - 1) * columns;
                            if distances[i].1 > next_distance {
                                distances[i].1 = next_distance;
                                distances[i].2 = node.0;
                            }
                        }
                    }
                }
                if let Some(down_row) = self.terrain.get(y + 1) {
                    let down_height = down_row[x];
                    if down_height <= current_height + 1 {
                        let i = x + (y + 1) * columns;
                        if distances[i].1 > next_distance {
                            distances[i].1 = next_distance;
                            distances[i].2 = node.0;
                        }
                    }
                }
            }
            else { break; }
        }

        distances[end_index].1
    }
}
impl From<&str> for Hill {
    fn from(s: &str) -> Self {
        let mut terrain: Vec<Vec<usize>> = Vec::new();
        let mut start = (0, 0);
        let mut end = (0, 0);
        for (y, line) in s.split('\n').enumerate() {
            let mut vec = Vec::new();
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    vec.push(0);
                    start = (x, y);
                }
                else if c == 'E' {
                    vec.push(27);
                    end = (x, y);
                }
                else {
                    vec.push(c as usize - 96);
                }
            }
            terrain.push(vec);
        }
        Self { terrain, start, end }
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap_or_else(|_| panic!("Unable to read input"));

    let hill: Hill = input.as_str().into();
    println!("The shortest route to the end does take {} steps", hill.get_number_of_steps_for_shortest_route_from_start());
    println!("The shortest scenic route to the end does take {} steps", hill.get_number_of_steps_for_shortest_scenic_route());
}


#[cfg(test)]
mod tests {
    use crate::Hill;

    #[test]
    fn check_against_example() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        let hill: Hill = input.into();

        assert_eq!(hill.get_number_of_steps_for_shortest_route_from_start(), 31);
        assert_eq!(hill.get_number_of_steps_for_shortest_scenic_route(), 29);
    }
}