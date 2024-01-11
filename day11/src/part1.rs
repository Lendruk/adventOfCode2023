use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;
#[derive(Debug)]
struct Space {
    size: usize,
    is_galaxy: bool,
}

#[derive(Debug)]
struct Universe {
    space_time: Vec<Vec<Space>>,
    width: usize,
    height: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

// Example taken from: https://doc.rust-lang.org/std/collections/binary_heap/index.html
// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Universe {
    fn from_snapshot(snapshot: Vec<String>) -> Self {
        let mut space: Vec<Vec<Space>> = vec![];

        for i in 0..snapshot.len() {
            let line = snapshot[i].chars().collect::<Vec<char>>();
            let mut new_row: Vec<Space> = vec![];
            for j in 0..line.len() {
                let point = line[j];
                new_row.push(Space {
                    is_galaxy: point == '#',
                    size: 1,
                });
            }

            space.push(new_row);
        }

        let height = space.len();
        let width = space[0].len();
        Universe {
            space_time: space,
            width: width,
            height: height,
        }
    }

    fn print(&self) -> String {
        let mut printed_universe = String::from("");
        let mut carry_over: HashMap<usize, usize> = HashMap::new();
        for i in 0..self.height {
            for j in 0..self.width {
                let space = &self.space_time[i][j];
                // println!("size at {},{} - {}", i, j, space.size);

                match carry_over.get(&j) {
                    Some(v) => {
                        // printed_universe = format!("{}{}", printed_universe, ".".repeat(*v));

                        for _ in 0..*v {
                            printed_universe.push_str(&".".repeat(self.width + v));
                            printed_universe.push_str("\n");
                        }
                        carry_over.remove(&j);
                    }
                    None => {}
                }

                let new_space: String = if space.is_galaxy {
                    String::from("#")
                } else {
                    ".".repeat(if space.size as usize / 2 > 1 {
                        space.size as usize / 2
                    } else {
                        1
                    })
                };

                if space.size / 2 > 0 {
                    carry_over.insert(j + 1, space.size as usize / 2);
                }
                printed_universe.push_str(&new_space);
            }
            printed_universe.push_str("\n");
        }

        return printed_universe;
    }

    // Expands on empty cols and rows
    fn expand(&mut self) {
        let mut cols_not_to_expand: HashSet<usize> = HashSet::new();
        for i in 0..self.height {
            let mut empty_row = true;
            let mut updated_row: Vec<Space> = vec![];
            for j in 0..self.width {
                let space = &self.space_time[i][j];

                if space.is_galaxy {
                    empty_row = false;
                    cols_not_to_expand.insert(j);
                }
                updated_row.push(Space {
                    is_galaxy: false,
                    size: space.size * 2,
                });
            }

            // Expand row
            if empty_row {
                self.space_time[i] = updated_row;
            }
        }

        for i in 0..self.height {
            for j in 0..self.width {
                if !cols_not_to_expand.contains(&j) {
                    self.space_time[i][j].size *= 2;
                }
            }
        }
    }

    fn find_shortest_paths(&self) -> Vec<usize> {
        let mut paths: Vec<usize> = vec![];
        let mut galaxies: Vec<(usize, usize)> = vec![];

        // Find galaxies first
        for i in 0..self.height {
            for j in 0..self.width {
                let space = &self.space_time[i][j];

                if space.is_galaxy {
                    galaxies.push((i, j));
                }
            }
        }

        // Build the pairs
        let mut pairs: Vec<((usize, usize), Vec<(usize, usize)>)> = vec![];
        for i in 0..galaxies.len() {
            let source = galaxies[i];

            let mut sub_pairs: Vec<(usize, usize)> = vec![];
            for j in i + 1..galaxies.len() {
                let target = galaxies[j];
                // pairs.push((source, target));
                sub_pairs.push(target);
            }
            pairs.push((source, sub_pairs));
        }

        println!("Total pairs: {}", pairs.len());
        for pair in pairs {
            paths.push(self.shortest_path_between(pair.0, pair.1));
        }

        return paths;
    }

    // Dijkstra
    // Using a priority queue in this case a binary heap
    fn shortest_path_between(&self, source: (usize, usize), targets: Vec<(usize, usize)>) -> usize {
        let mut distances: Vec<Vec<usize>> = vec![];
        let mut prev: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();
        // let mut queue: HashSet<(usize, usize)> = HashSet::new();
        let mut queue: BinaryHeap<State> = BinaryHeap::new();

        println!("source {:?}", source);
        println!("target {:?}", targets);
        for i in 0..self.height {
            let mut row: Vec<usize> = vec![];
            for j in 0..self.width {
                row.push(usize::MAX);
                prev.insert((i, j), None);
            }
            distances.push(row);
        }
        queue.push(State {
            cost: 0,
            position: (source.0, source.1),
        });
        distances[source.0][source.1] = 0;

        while queue.len() > 0 {
            let first_in_queue = queue.pop().unwrap();
            // let closest = Universe::get_min_of_grid(&distances, &queue);
            // queue.remove(&closest);

            if first_in_queue.cost > distances[first_in_queue.position.0][first_in_queue.position.1]
            {
                continue;
            }

            let closest = first_in_queue.position;

            let mut directions: Vec<(usize, usize)> = vec![];
            // N x , y - 1
            match closest.0.checked_sub(1) {
                Some(_) => {
                    let north = (closest.0 - 1, closest.1);
                    directions.push(north);
                }
                None => {}
            }
            // S x , y + 1
            if closest.0 + 1 < self.height {
                let south = (closest.0 + 1, closest.1);
                directions.push(south);
            }
            // E x + 1, y
            if closest.1 + 1 < self.width {
                let east = (closest.0, closest.1 + 1);
                directions.push(east);
            }

            match closest.1.checked_sub(1) {
                Some(_) => {
                    // W x - 1, y
                    let west = (closest.0, closest.1 - 1);
                    directions.push(west);
                }
                None => {}
            }

            directions.iter().for_each(|val| {
                // Edge from closest to current
                let v = self.space_time[val.0][val.1].size;

                // println!("{}", v);
                // println!("val {:?}", val);
                let dist = distances[closest.0][closest.1] + v;

                if dist < distances[val.0][val.1] {
                    distances[val.0][val.1] = dist;
                    prev.insert((val.0, val.1), Some(closest));
                    // queue.insert((val.0, val.1));
                    queue.push(State {
                        cost: dist,
                        position: *val,
                    })
                }
            });
        }

        let mut total = 0;
        for target in targets {
            total += distances[target.0][target.1];
        }

        return total;
    }
}

fn main() -> std::io::Result<()> {
    // Open the file for reading
    let file = File::open("./src/input.txt")?;

    // Create a buffered reader to read the file
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Something went wrong"))
        .collect();

    let mut universe = Universe::from_snapshot(lines);
    println!("original universe");
    println!("{}", universe.print());
    // Expand once
    println!("expanded universe");
    universe.expand();

    let shortest_paths = universe.find_shortest_paths();
    println!("Amount of paths {}", shortest_paths.len());
    println!(
        "Shortest path sum: {}",
        shortest_paths.iter().fold(0, |a, b| a + b)
    );

    Ok(())
}
