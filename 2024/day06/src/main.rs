use std::fmt::Display;

use meaningful_lines::MeaningfulLines;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut map = Map::new(include_str!("../data.txt"));
    map.generate_path();

    println!(
        "The answer is {}",
        map.data
            .iter()
            .filter(|&&cell| cell == MapCell::Path)
            .count()
    );
}

fn part2() {
    let map = Map::new(include_str!("../data.txt"));

    let mut count = 0;

    for y in 0..map.height {
        for x in 0..map.width {
            if x == map.guard.0 && y == map.guard.1 {
                continue;
            }
            if map.check_obstacle(x, y) {
                count += 1;
            }
        }
    }

    // map.check_obstacle(3, 6);

    println!("The answer is {}", count);
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
enum MapCell {
    #[default]
    Empty,
    Obstacle,
    Path,
}

#[derive(Debug, Default, Clone, Copy)]
struct Route {
    moved_up: bool,
    moved_down: bool,
    moved_left: bool,
    moved_right: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GuardDirection {
    Up,
    Down,
    Left,
    Right,
}

impl From<GuardDirection> for (isize, isize) {
    fn from(direction: GuardDirection) -> Self {
        match direction {
            GuardDirection::Up => (0, -1),
            GuardDirection::Down => (0, 1),
            GuardDirection::Left => (-1, 0),
            GuardDirection::Right => (1, 0),
        }
    }
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    data: Vec<MapCell>,
    guard: (usize, usize, GuardDirection),
}

impl Display for MapCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            MapCell::Empty => '.',
            MapCell::Obstacle => '#',
            MapCell::Path => 'X',
        };
        write!(f, "{}", c)
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if x == self.guard.0 && y == self.guard.1 {
                    let c = match self.guard.2 {
                        GuardDirection::Up => '^',
                        GuardDirection::Down => 'V',
                        GuardDirection::Left => '<',
                        GuardDirection::Right => '>',
                    };
                    write!(f, "{}", c)?;
                    continue;
                }
                let cell = &self.data[y * self.width + x];
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn new(text: &str) -> Self {
        let width = text.lines().next().unwrap().len();
        let height = MeaningfulLines::new(text).count();

        let mut map = Map {
            width,
            height,
            data: Vec::with_capacity(width * height),
            guard: (0, 0, GuardDirection::Up),
        };
        map.data.resize_with(width * height, || MapCell::Empty);

        let lines = MeaningfulLines::new(text);
        lines.enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let cell = match c {
                    '.' => MapCell::Empty,
                    '#' => MapCell::Obstacle,
                    '^' => {
                        map.guard = (x, y, GuardDirection::Up);
                        MapCell::Empty
                    }
                    'V' => {
                        map.guard = (x, y, GuardDirection::Down);
                        MapCell::Empty
                    }
                    '<' => {
                        map.guard = (x, y, GuardDirection::Left);
                        MapCell::Empty
                    }
                    '>' => {
                        map.guard = (x, y, GuardDirection::Right);
                        MapCell::Empty
                    }
                    _ => panic!("Invalid character in map: {}", c),
                };
                map.data[y * width + x] = cell;
            });
        });

        map
    }

    fn generate_path(&mut self) {
        loop {
            let (x, y, direction) = self.guard;

            self.data[y * self.width + x] = MapCell::Path;

            let (dx, dy) = direction.into();
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            if nx < 0 || nx >= self.width as isize || ny < 0 || ny >= self.height as isize {
                break;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            let cell = &mut self.data[ny * self.width + nx];
            if *cell == MapCell::Obstacle {
                // Turn right
                let direction = match direction {
                    GuardDirection::Up => GuardDirection::Right,
                    GuardDirection::Down => GuardDirection::Left,
                    GuardDirection::Left => GuardDirection::Up,
                    GuardDirection::Right => GuardDirection::Down,
                };
                self.guard = (x, y, direction);
            } else {
                self.guard = (nx, ny, direction);
            }
        }
    }

    fn check_obstacle(&self, obstacle_x: usize, obstacle_y: usize) -> bool {
        let mut routes = Vec::with_capacity(self.width * self.height);
        routes.resize_with(self.width * self.height, Route::default);

        // Get position and direction of guard.
        let (mut x, mut y, mut direction) = self.guard;

        // Repeat movements until we leave the map or the guard moves in a loop.
        loop {
            // Find the direction for the guard to move in.
            let (mut dx, mut dy) = direction.into();
            let (mut nx, mut ny) = (x as isize + dx, y as isize + dy);

            loop {
                // If the new position is outside the map, we have found a bad obstacle.
                if nx < 0 || nx >= self.width as isize || ny < 0 || ny >= self.height as isize {
                    return false;
                }

                // Otherwise, if the new position is not an obstacle, we have
                // found our good direction.
                if self.data[ny as usize * self.width + nx as usize] != MapCell::Obstacle
                    && (nx as usize != obstacle_x || ny as usize != obstacle_y)
                {
                    break;
                }

                // Otherwise, turn right.
                direction = match direction {
                    GuardDirection::Up => GuardDirection::Right,
                    GuardDirection::Down => GuardDirection::Left,
                    GuardDirection::Left => GuardDirection::Up,
                    GuardDirection::Right => GuardDirection::Down,
                };

                (dx, dy) = direction.into();
                (nx, ny) = (x as isize + dx, y as isize + dy);
            }

            // If the guard has already walked from this cell in this direction,
            // we have found a good obstacle.
            if match direction {
                GuardDirection::Up => routes[y * self.width + x].moved_up,
                GuardDirection::Down => routes[y * self.width + x].moved_down,
                GuardDirection::Left => routes[y * self.width + x].moved_left,
                GuardDirection::Right => routes[y * self.width + x].moved_right,
            } {
                return true;
            }

            // Otherwise, let's mark the cell as visited and move the guard.
            match direction {
                GuardDirection::Up => routes[y * self.width + x].moved_up = true,
                GuardDirection::Down => routes[y * self.width + x].moved_down = true,
                GuardDirection::Left => routes[y * self.width + x].moved_left = true,
                GuardDirection::Right => routes[y * self.width + x].moved_right = true,
            }

            x = nx as usize;
            y = ny as usize;
        }
    }
}
