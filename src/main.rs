use std::fs;

/// Define a forest of 2D Array as tree grid
struct Forest(Vec<Vec<u32>>);
impl Forest {
    /// Create a forest from multiple lines of input
    /// 
    /// # Arguments
    /// * `input` - The puzzle provided input
    fn from_input(input: &str) -> Self {
        let mut rows = Vec::new();
        for line in input.split('\n') {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c.to_digit(10).unwrap());
            }
            rows.push(row);
        }
        Self(rows)
    }
    /// Count the visible trees from the edge.
    /// Only count each tree once.
    fn count_visible_trees(&self) -> usize {
        let mut count = self.0.len() * 2 + (self.0[0].len() - 2) * 2;
        for r in 1..(self.0.len() - 1) {
            for c in 1..(self.0[r].len() - 1) {
                let size = self.0[r][c];
                // visible from left
                if self.0[r].iter().take(c).all(|el| el < &size) {
                    count += 1;
                    continue;
                }
                // visible from right
                if self.0[r].iter().skip(c + 1).rev().all(|el| el < &size) {
                    count += 1;
                    continue;
                }
                let column_iter = self.0.iter().map(|el| el[c]);
                // visible from top
                if column_iter.clone().take(r).all(|el| el < size) {
                    count += 1;
                    continue;
                }
                // visible from bottom
                if column_iter.skip(r + 1).rev().all(|el| el < size) {
                    count += 1;
                    continue;
                }
            }
        }
        count
    }
    /// Search for the highest scenic score in the forest.
    /// The border is ignored as the score will be 0.
    fn find_highest_scenic_score(&self) -> usize {
        let mut heigest_score = 0;
        for r in 1..(self.0.len() - 1) {
            for c in 1..(self.0[r].len() - 1) {
                let size = self.0[r][c];
                let comp = |el: u32, last_ok: &mut bool| -> bool {
                    if !*last_ok {
                        false
                    }
                    else {
                        if el >= size {
                            *last_ok = false;
                        }
                        true
                    }
                };
                let mut last_ok = true;
                let left = self.0[r].iter().take(c).rev().take_while(|el| comp(**el, &mut last_ok)).count();
                last_ok = true;
                let right = self.0[r].iter().skip(c + 1).take_while(|el|  comp(**el, &mut last_ok)).count();
                let column_iter = self.0.iter().map(|el| el[c]);
                last_ok = true;
                let top = column_iter.clone().take(r).rev().take_while(|el|  comp(*el, &mut last_ok)).count();
                last_ok = true;
                let bottom = column_iter.skip(r + 1).take_while(|el|  comp(*el, &mut last_ok)).count();
                let score = left * right * top * bottom;
                if score > heigest_score {
                    heigest_score = score;
                }
            }
        }
        heigest_score
    }
}

fn main() {

    let input = fs::read_to_string("input").unwrap_or_else(|_| panic!("Unable to read input"));

    let forest = Forest::from_input(&input);
    println!("The number of visible from outside visible trees is {}", forest.count_visible_trees());
    println!("The highest scenic score is {}", forest.find_highest_scenic_score());
}


#[cfg(test)]
mod tests {
    use crate::Forest;


    #[test]
    fn check_against_example() {
        let forest = Forest::from_input("30373
25512
65332
33549
35390");
        assert_eq!(forest.count_visible_trees(), 21);
        assert_eq!(forest.find_highest_scenic_score(), 8);
    }
}