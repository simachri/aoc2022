use std::collections::VecDeque;

type GridPos = (u32, u32);
type Path = Vec<GridPos>;
type ShortestPath = Option<Path>;

#[derive(Debug, PartialEq)]
struct GridSquare {
    height: u32,
    visited: bool,
}

impl GridSquare {
    fn set_visited(&mut self) {
        self.visited = true;
    }
}

#[derive(Debug, PartialEq)]
struct Board {
    start: GridPos,
    end: GridPos,
    grid: Vec<Vec<GridSquare>>,
}

impl Board {
    fn get_square_mut_at(&mut self, pos: GridPos) -> Option<&mut GridSquare> {
        let (row, col) = pos;
        self.grid
            .get_mut(row as usize)
            .and_then(|row| row.get_mut(col as usize))
    }

    fn get_square_at(&self, pos: &GridPos) -> Option<&GridSquare> {
        let (row, col) = pos;
        self.grid
            .get(*row as usize)
            .and_then(|row| row.get(*col as usize))
    }

    fn is_reachable(&self, source: &GridSquare, target: &GridSquare) -> bool {
        target.height <= source.height + 1
    }

    fn add_direction_if_reachable_and_not_visited(
        &self,
        source_square: &GridSquare,
        target_pos: GridPos,
        directions: &mut Vec<GridPos>,
    ) -> Result<(), String> {
        self.get_square_at(&target_pos)
            .ok_or_else(|| format!("Invalid position {:?}", target_pos))
            .map(|target_square| {
                if !target_square.visited && self.is_reachable(source_square, target_square) {
                    directions.push(target_pos);
                }
            })
    }

    fn get_reachable_unvisited_neighbours(&self, pos: GridPos) -> Result<Vec<GridPos>, String> {
        let mut directions = Vec::new();
        let mut target_pos: GridPos;
        let source_square = self
            .get_square_at(&pos)
            .ok_or_else(|| format!("Invalid position {:?}", pos))?;
        let (row, col) = pos;

        // Top
        if row > 0 && row < self.grid.len() as u32 {
            target_pos = (row - 1, col);

            self.add_direction_if_reachable_and_not_visited(
                source_square,
                target_pos,
                &mut directions,
            )?;
        }

        // Bottom
        if row < self.grid.len() as u32 - 1 {
            target_pos = (row + 1, col);

            self.add_direction_if_reachable_and_not_visited(
                source_square,
                target_pos,
                &mut directions,
            )?;
        }

        // Left
        if col > 0 && col < self.grid[0].len() as u32 {
            target_pos = (row, col - 1);

            self.add_direction_if_reachable_and_not_visited(
                source_square,
                target_pos,
                &mut directions,
            )?;
        }

        // Right
        if col < self.grid[0].len() as u32 - 1 {
            target_pos = (row, col + 1);

            self.add_direction_if_reachable_and_not_visited(
                source_square,
                target_pos,
                &mut directions,
            )?;
        }

        return Ok(directions);
    }
}

fn main() {
    let input = include_str!("../input.txt");

    println!("Result of part 1: {}", find_shortest_path(input).unwrap());
}

fn find_shortest_path(input: &str) -> Option<u32> {
    let mut shortest_path: ShortestPath = None;

    let mut board = parse_board(input);
    let start_pos = board.start;

    let mut possible_paths: VecDeque<Path> = VecDeque::new();
    possible_paths.push_back(vec![start_pos]);

    board.get_square_mut_at(start_pos).unwrap().set_visited();

    shortest_path = breadth_first(&mut board, &mut possible_paths, shortest_path);

    if shortest_path.is_none() {
        return None;
    }

    return Some(shortest_path.unwrap().len() as u32 - 1);
}

fn breadth_first(
    board: &mut Board,
    possible_paths: &mut VecDeque<Path>,
    mut shortest_path: ShortestPath,
) -> ShortestPath {
    for _ in 0..possible_paths.len() {
        let current_path = possible_paths.pop_front().unwrap();

        let neighbours = board
            .get_reachable_unvisited_neighbours(*current_path.last().unwrap())
            .unwrap();

        if neighbours.is_empty() {
            continue;
        }

        for n in neighbours {
            let mut new_path = current_path.clone();

            new_path.push(n);
            board.get_square_mut_at(n).unwrap().set_visited();

            if n == board.end {
                println!("\nReached end position: {:?}", board.end);
                println!("Steps count: {:?}", new_path.len() - 1);

                if shortest_path.is_none() {
                    shortest_path = Some(new_path.clone());
                } else {
                    if new_path.len() < shortest_path.as_ref().unwrap().len() {
                        println!(
                            "New shortest path found with steps count {}.",
                            new_path.len() - 1
                        );
                        shortest_path = Some(new_path.clone());
                    } else {
                        println!(
                            "Path is longer than the currently shortest path found ({} steps). 
                        Discarding it.",
                            shortest_path.as_ref().unwrap().len() - 1
                        );
                        continue;
                    }
                }
            }

            possible_paths.push_back(new_path.clone());
        }
    }

    if !possible_paths.is_empty() {
        shortest_path = breadth_first(board, possible_paths, shortest_path);
    }

    return shortest_path;
}

fn parse_board(input: &str) -> Board {
    let mut start: GridPos = (0, 0);
    let mut end: GridPos = (0, 0);
    let mut grid: Vec<Vec<GridSquare>> = Vec::new();

    for line in input.lines() {
        let mut grid_line: Vec<GridSquare> = Vec::new();

        for char in line.chars() {
            let mut height = char as u32;

            match char {
                'S' => {
                    start = {
                        height = 'a' as u32;
                        (grid.len() as u32, grid_line.len() as u32)
                    }
                }
                'E' => {
                    end = {
                        height = 'z' as u32;
                        (grid.len() as u32, grid_line.len() as u32)
                    }
                }
                _ => (),
            }

            grid_line.push(GridSquare {
                height,
                visited: false,
            });
        }

        grid.push(grid_line)
    }

    return Board { start, end, grid };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_board() {
        let input = r"aSa
bcm
Edc";

        let want = Board {
            start: (0, 1),
            end: (2, 0),
            grid: vec![
                vec![
                    // (0, 0): a
                    GridSquare {
                        height: 'a' as u32,
                        visited: false,
                    },
                    // (0, 1): S
                    GridSquare {
                        height: 'a' as u32,
                        visited: false,
                    },
                    // (0, 2): a
                    GridSquare {
                        height: 'a' as u32,
                        visited: false,
                    },
                ],
                vec![
                    // (1, 0): b
                    GridSquare {
                        height: 'b' as u32,
                        visited: false,
                    },
                    // (1, 1): c
                    GridSquare {
                        height: 'c' as u32,
                        visited: false,
                    },
                    // (1, 2): m
                    GridSquare {
                        height: 'm' as u32,
                        visited: false,
                    },
                ],
                vec![
                    // (2, 0): E
                    GridSquare {
                        height: 'z' as u32,
                        visited: false,
                    },
                    // (2, 1): d
                    GridSquare {
                        height: 'd' as u32,
                        visited: false,
                    },
                    // (2, 2): c
                    GridSquare {
                        height: 'c' as u32,
                        visited: false,
                    },
                ],
            ],
        };

        assert_eq!(parse_board(input), want);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");

        assert_eq!(find_shortest_path(input).unwrap(), 31);
    }
}
