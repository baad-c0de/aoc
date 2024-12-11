use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use meaningful_lines::MeaningfulLines;

fn main() {
    part1();
    part2();
}

fn part1() {
    let world = World::new(include_str!("../data.txt"), process_antinodes);

    println!("Solution to part1: {}", world.number_antinodes());
}

fn part2() {
    let world = World::new(include_str!("../data.txt"), process_antinodes_2);

    println!("Solution to part2: {}", world.number_antinodes());
}

struct World {
    map: Vec<char>,
    width: usize,
    height: usize,
    nodes: HashMap<char, Vec<(usize, usize)>>,
    antinodes: HashMap<char, Vec<(usize, usize)>>,
}

impl World {
    fn new(
        input: &str,
        processor: impl Fn(
            &HashMap<char, Vec<(usize, usize)>>,
            usize,
            usize,
        ) -> HashMap<char, Vec<(usize, usize)>>,
    ) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = MeaningfulLines::new(input).count();

        let mut map = Vec::with_capacity(width * height);
        let mut nodes = HashMap::new();

        let lines = MeaningfulLines::new(input);
        for (y, line) in lines.enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c != '.' {
                    let entry: &mut Vec<(usize, usize)> = nodes.entry(c).or_default();
                    entry.push((x, y));
                }
                map.push(c);
            }
        }

        let antinodes = processor(&nodes, width, height);

        // for (_, positions) in &antinodes {
        //     for (x, y) in positions {
        //         if map[y * width + x] != '.' {
        //             map[y * width + x] = '#';
        //         }
        //     }
        // }

        Self {
            map,
            width,
            height,
            nodes,
            antinodes,
        }
    }

    fn number_antinodes(&self) -> usize {
        self.antinodes
            // Process the vectors only
            .values()
            // Flatten the vectors into one big iterator
            .flat_map(|positions| positions.iter())
            // Remove the references by cloning
            .cloned()
            // Collect the positions into a HashSet to remove duplicates
            .collect::<HashSet<(usize, usize)>>()
            // And return the number of antinodes
            .len()
    }
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Output Map
        writeln!(f, "Map:")?;
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.map[y * self.width + x])?;
            }
            writeln!(f)?;
        }

        // Output Nodes
        writeln!(f, "Nodes:")?;
        for (node, positions) in &self.nodes {
            writeln!(f, "{}: {:?}", node, positions)?;
        }

        // Output Antinodes
        writeln!(f, "Antinodes:")?;
        for (node, positions) in &self.antinodes {
            writeln!(f, "{}: {:?}", node, positions)?;
        }
        Ok(())
    }
}

fn process_antinodes(
    nodes: &HashMap<char, Vec<(usize, usize)>>,
    width: usize,
    height: usize,
) -> HashMap<char, Vec<(usize, usize)>> {
    let mut antinodes = HashMap::new();

    // Iterate through each node type
    for (node, positions) in nodes {
        // Iterate through each position of the node type
        for (x, y) in positions {
            // Iterate through the other nodes
            for (other_x, other_y) in positions {
                if x != other_x && y != other_y {
                    // Calculate the distance vector between the two nodes
                    let dx = *other_x as isize - *x as isize;
                    let dy = *other_y as isize - *y as isize;

                    // Calculate the position of the antinode and see if it is valid
                    let antinode_x = *x as isize - dx;
                    let antinode_y = *y as isize - dy;
                    if antinode_x >= 0
                        && antinode_x < width as isize
                        && antinode_y >= 0
                        && antinode_y < height as isize
                    {
                        let entry: &mut Vec<(usize, usize)> = antinodes.entry(*node).or_default();
                        entry.push((antinode_x as usize, antinode_y as usize));
                    }
                }
            }
        }
    }

    antinodes
}

fn process_antinodes_2(
    nodes: &HashMap<char, Vec<(usize, usize)>>,
    width: usize,
    height: usize,
) -> HashMap<char, Vec<(usize, usize)>> {
    let mut antinodes = HashMap::new();

    // Iterate through each node type
    for (node, positions) in nodes {
        // Iterate through each position of the node type
        for (x, y) in positions {
            // Iterate through the other nodes
            for (other_x, other_y) in positions {
                if x != other_x && y != other_y {
                    // Calculate the distance vector between the two nodes
                    let dx = *other_x as isize - *x as isize;
                    let dy = *other_y as isize - *y as isize;

                    fn add_node(
                        antinodes: &mut HashMap<char, Vec<(usize, usize)>>,
                        node: char,
                        x: usize,
                        y: usize,
                    ) {
                        let entry: &mut Vec<(usize, usize)> = antinodes.entry(node).or_default();
                        entry.push((x, y));
                    }

                    // Add antinodes at the position of the starting node
                    add_node(&mut antinodes, *node, *x, *y);

                    // Add antinodes in the direction of the second node
                    let mut nx = *x as isize;
                    let mut ny = *y as isize;

                    while nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                        add_node(&mut antinodes, *node, nx as usize, ny as usize);
                        nx += dx;
                        ny += dy;
                    }

                    // Add antinodes in the opposite direction of the second node
                    nx = *x as isize - dx;
                    ny = *y as isize - dy;

                    while nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                        add_node(&mut antinodes, *node, nx as usize, ny as usize);
                        nx -= dx;
                        ny -= dy;
                    }
                }
            }
        }
    }

    antinodes
}
