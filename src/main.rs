use std::fs;
use regex::Regex;
#[macro_use]
extern crate lazy_static;


/// A Valve
#[derive(Debug, PartialEq, Eq)]
struct Valve {
    /// Id of the valve
    id: String,
    /// The pressure release rate of the valve
    rate: i32
}
impl From<&str> for Valve {
    fn from(s: &str) -> Self {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"Valve (.+) has flow rate=(\d+)").unwrap();
        }
        let cap = RE.captures(s).unwrap();
        Self {
            id: cap.get(1).unwrap().as_str().to_string(),
            rate: cap.get(2).unwrap().as_str().parse().unwrap()
        }
    }
}

/// Renamed vec of valves
#[derive(Debug)]
struct Valves(Vec<Valve>);
impl Valves {
    fn new() -> Self {
        Self(Vec::new())
    }
}
impl From<&str> for Valves {
    /// From &str
    /// 
    /// # Arguments
    /// * `s` - The puzzle input string
    fn from(s: &str) -> Self {
        let mut v = Vec::new();
        for line in s.split('\n') {
            v.push(line.into());
        }
        Self(v)
    }
}

/// The edges in the valve graph (leading tunnels)
#[derive(Debug)]
struct Edge<'a> {
    /// The edge from valve
    from: &'a Valve,
    /// to valve
    to: &'a Valve
}
impl<'a> Edge<'a> {
    /// Create new edges
    /// 
    /// Arguments
    /// * `from` - The id of the valve from
    /// * `to`- The id of the valve to
    /// * `valves` - The valves the ids are in
    fn new(from: &str, to: &str, valves: &'a Valves) -> Self {
        Edge {
            from: valves.0.iter().find(|el| el.id == from).unwrap(),
            to: valves.0.iter().find(|el| el.id == to).unwrap()
        }
    }
}
/// Renamed vector if edges
#[derive(Debug)]
struct Edges<'a>(Vec<Edge<'a>>);
impl<'a> Edges<'a> {
    /// Create a new one
    fn new() -> Self {
        Self(Vec::new())
    }
    /// Populate the edges with edges from the puzzle input
    /// 
    /// # Arguments
    /// * `s` - The puzzle input
    /// * `valves` - The valves also read from the puzzle input
    fn populate(&mut self, s: &str, valves: &'a Valves) {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"Valve (.+) has flow rate=\d+; tunnels* leads* to valves* (.+)$").unwrap();
        }
        for line in s.split('\n') {
            let cap = RE.captures(line).unwrap();
            let id = cap.get(1).unwrap().as_str();
            let cons = cap.get(2).unwrap().as_str();
            for con in cons.split(", ") {
                self.0.push(Edge::new(id, con, valves));
            }
        }
    }
    /// Returns all edges from a specific valve
    /// 
    /// #Arguments
    /// * `valve` - The valve to search for
    fn get_edges_for_valve(&self, valve: &'a Valve) -> Vec<&Edge<'a>> {
        self.0.iter().filter(|el| el.from == valve).collect()
    }
}

/// Release of pressure for a valve and from a time
#[derive(Debug, Clone)]
struct Release<'a> {
    /// The time the release starts
    t: usize,
    /// The valve releasing pressure
    valve: &'a Valve
}
/// Renamed vector of releases
#[derive(Debug, Clone)]
struct Releases<'a>(Vec<Release<'a>>);
impl<'a> Releases<'a> {
    /// New empty releases
    fn new() -> Self {
        Self(Vec::new())
    }
    /// Increment the releases to a specific time
    /// 
    /// # Arguments
    /// * `t` - The time to increment to
    /// 
    /// # Returns
    /// The releases pressure until (inclusive) time t
    fn increment_to_t(&self, t: usize) -> i32 {
        let mut release = 0;
        for dt in 1..=t {
            release += self.0.iter().filter_map(|el| if dt >= el.t { Some(el.valve.rate) } else { None }).sum::<i32>();
        }
        release
    }
}
/// Node for use in the route optimization algorithms
#[derive(Debug, Clone)]
struct Node<'a> {
    /// Valve of the node
    valve: &'a Valve,
    /// Is the valve open?
    open: bool,
    /// The current time
    t: usize,
    /// The releases of the route
    releases: Releases<'a>,
    /// The previous visited valves and if opened
    previous: Vec<(&'a Valve, bool)>, // Valve, open
    /// Indicates if this route is done
    done: bool
}

/// Find an optimized route through the tunnels to release the most pressure.
/// Using Dijkstra algorithm to find the best route but without stopping on a specific target.
/// 
/// # Arguments
/// * `start_id` - The valve to start from
/// * `valves` - The valves to optimize
/// * `edges` - The connections between the valves to optimize over
/// 
/// # Returns
/// The pressure released over the optimized route
fn find_optimized_route<'a>(start_id: &str, valves: &Valves, edges: &Edges<'a>, time: usize) -> i32 {
    let mut nodes: Vec<Node> = valves.0.iter().map(|el| Node {valve: el, releases: Releases::new(), open: false, t: usize::MAX, previous: Vec::new(), done: false}).collect();
    nodes.extend(valves.0.iter().filter(|el| el.rate != 0).map(|el| Node {valve: el, releases: Releases::new(), open: true, t: usize::MAX, previous: Vec::new(), done: false}));
    nodes.iter_mut().find(|el| el.valve.id == start_id).unwrap().t = 0;

    loop {
        let (connections, current_t, current_releases, new_current_previous) = {
            let opt_node = nodes.iter_mut().filter(|el| !el.done).min_by(|a, b| a.releases.increment_to_t(time).cmp(&b.releases.increment_to_t(time)));
            if let Some(node) = opt_node {
                node.done = true;
                if node.t >= time || node.t == usize::MAX { continue; }
                let mut new_current_previous = node.previous.clone();
                new_current_previous.push((node.valve, node.open));
                (edges.get_edges_for_valve(node.valve), node.t, node.releases.clone(), new_current_previous)
            }
            else { break; }
        };
        for edge in connections {
            nodes.iter_mut().filter(|el| el.valve == edge.to).for_each(|el| {
                if el.open && new_current_previous.iter().any(|v| v.0 == el.valve && v.1) {
                    return
                }
                let new_t = current_t + if el.open { 2 } else { 1 };
                let mut new_releases = current_releases.clone();
                if el.open {
                    new_releases.0.push(Release { t: new_t + 1, valve: el.valve });
                }
                let new_incremented_release = new_releases.increment_to_t(new_t + 1);
                let incremented_release = el.releases.increment_to_t(new_t + 1);
                if incremented_release < new_incremented_release || (new_t < el.t && incremented_release <= new_incremented_release) {
                    el.done = false;
                    el.t = new_t;
                    el.releases = new_releases;
                    el.previous = new_current_previous.clone();
                }
            });
        }
    }

    nodes.iter().filter(|el| el.t <= time).map(|el| el.releases.increment_to_t(time)).max().unwrap()
}


fn main() {
    let input = fs::read_to_string("input").unwrap_or_else(|_| panic!("Unable to read input"));

    let valves: Valves = input.as_str().into();
    let mut edges = Edges::new();
    edges.populate(input.as_str(), &valves);

    println!("With the optimized route you can release {} pressure", find_optimized_route("AA", &valves, &edges, 30));
}


#[cfg(test)]
mod tests {
    use crate::{Valves, Edges, find_optimized_route};


    #[test]
    fn check_against_example() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

        let valves: Valves = input.into();
        let mut edges = Edges::new();
        edges.populate(input, &valves);
        
        assert_eq!(find_optimized_route("AA", &valves, &edges, 30), 1651);
    }
}