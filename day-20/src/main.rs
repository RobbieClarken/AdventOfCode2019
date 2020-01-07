use std::collections::{HashSet, VecDeque};

fn main() {
    challenge_1();
}

fn challenge_1() {
    let input = std::fs::read_to_string("input").unwrap();
    let maze = Maze::new(&input);
    println!(
        "Challenge 1: Minimum number of steps through maze = {}",
        maze.min_steps()
    );
}

struct Maze {
    tiles: Vec<Vec<Tile>>,
}

impl Maze {
    fn new(s: &str) -> Self {
        let tiles: Vec<Vec<_>> = s
            .lines()
            .map(|l| l.chars().map(|c| c.into()).collect())
            .collect();
        let mut maze = Self { tiles };
        for y in 0..=maze.max_y() {
            for x in 0..=maze.max_x() {
                let p = Pos::new(x, y);
                let label = maze.get_label(p);
                if let Some(('A', 'A')) = label {
                    maze.tiles[y][x] = Tile::Entrance;
                } else if let Some(('Z', 'Z')) = label {
                    maze.tiles[y][x] = Tile::Exit;
                } else if let Some((l1, l2)) = label {
                    maze.tiles[y][x] = Tile::Portal(l1, l2);
                }
            }
        }
        maze
    }

    fn max_x(&self) -> usize {
        self.tiles[0].len() - 1
    }

    fn max_y(&self) -> usize {
        self.tiles.len() - 1
    }

    fn get_label(&self, p: Pos) -> Option<(char, char)> {
        if self.get(p) != Tile::Passage {
            return None;
        }
        let checks = [
            ((-2, 0), (-1, 0)),
            ((1, 0), (2, 0)),
            ((0, 1), (0, 2)),
            ((0, -2), (0, -1)),
        ];
        for &(d1, d2) in checks.iter() {
            if let Tile::External(l1) = self.get(p + d1) {
                if let Tile::External(l2) = self.get(p + d2) {
                    return Some((l1, l2));
                }
            }
        }
        None
    }

    fn portal_exit(&self, p: Pos) -> Pos {
        let matches = self.find_tiles(self.get(p));
        assert_eq!(matches.len(), 2);
        if matches[0] == p {
            matches[1]
        } else {
            matches[0]
        }
    }

    fn find_tiles(&self, tile: Tile) -> Vec<Pos> {
        let mut out = vec![];
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, &t) in row.iter().enumerate() {
                if t == tile {
                    out.push(Pos::new(x, y));
                }
            }
        }
        out
    }

    fn get(&self, p: Pos) -> Tile {
        if p.x <= self.max_x() || p.y <= self.max_y() {
            self.tiles[p.y][p.x]
        } else {
            Tile::External(' ')
        }
    }

    fn min_steps(self) -> usize {
        let entrance = self.find_tiles(Tile::Entrance)[0];
        PathFinder::shortest_path(self, entrance)
    }
}

struct PathFinder {
    maze: Maze,
    queue: VecDeque<(Pos, usize)>,
    visited: HashSet<Pos>,
}

impl PathFinder {
    pub fn shortest_path(maze: Maze, start: Pos) -> usize {
        let mut finder = PathFinder {
            maze,
            queue: vec![].into(),
            visited: HashSet::new(),
        };
        finder.find_path(start)
    }

    fn find_path(&mut self, start: Pos) -> usize {
        self.insert(start, 0);
        loop {
            let (p, distance) = self.next();
            let t = self.maze.get(p);
            if let Tile::Portal(_, _) = t {
                self.insert(self.maze.portal_exit(p), distance + 1);
            }
            for neighbour in p.neighbours() {
                if self.visited.contains(&neighbour) {
                    continue;
                }
                let t = self.maze.get(neighbour);
                match t {
                    Tile::Exit => return distance + 1,
                    Tile::Passage | Tile::Portal(_, _) => {
                        self.insert(neighbour, distance + 1);
                    }
                    Tile::Wall | Tile::External(_) => {}
                    _ => unimplemented!("{:?} type tile not handled", t),
                };
            }
        }
    }

    fn insert(&mut self, pos: Pos, distance: usize) {
        self.queue.push_back((pos, distance));
        self.visited.insert(pos);
    }

    fn next(&mut self) -> (Pos, usize) {
        self.queue.pop_front().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn neighbours(&self) -> Vec<Self> {
        let mut out = Vec::with_capacity(4);
        if self.x > 0 {
            out.push(Self::new(self.x - 1, self.y));
        }
        if self.y > 0 {
            out.push(Self::new(self.x, self.y - 1));
        }
        out.push(Self::new(self.x + 1, self.y));
        out.push(Self::new(self.x, self.y + 1));
        out
    }
}

impl std::ops::Add<(isize, isize)> for Pos {
    type Output = Self;

    fn add(self, other: (isize, isize)) -> Self::Output {
        Self {
            x: (self.x as isize + other.0) as usize,
            y: (self.y as isize + other.1) as usize,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    External(char),
    Wall,
    Passage,
    Entrance,
    Exit,
    Portal(char, char),
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            ' ' | 'A'..='Z' => Self::External(c),
            '#' => Self::Wall,
            '.' => Self::Passage,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test_day_20 {
    use super::*;

    #[test]
    fn parses_maze() {
        let maze = Maze::new(
            "
         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       
"
            .trim_matches('\n'),
        );
        assert_eq!(maze.get(Pos::new(0, 0)), Tile::External(' '));
        assert_eq!(maze.get(Pos::new(9, 0)), Tile::External('A'));
        assert_eq!(maze.get(Pos::new(9, 1)), Tile::External('A'));
        assert_eq!(maze.get(Pos::new(9, 2)), Tile::Entrance);
        assert_eq!(maze.get(Pos::new(13, 16)), Tile::Exit);
        assert_eq!(maze.get(Pos::new(9, 3)), Tile::Passage);
        assert_eq!(maze.get(Pos::new(2, 2)), Tile::Wall);
        assert_eq!(maze.get(Pos::new(2, 8)), Tile::Portal('B', 'C'));
    }

    #[test]
    fn finds_entrances_exits_and_portals() {
        let maze = Maze::new(
            "
   A   
   A   
  #.#  
BC...DE
  #.#  
   Z   
   Z   
"
            .trim_matches('\n'),
        );
        assert_eq!(maze.get(Pos::new(3, 2)), Tile::Entrance);
        assert_eq!(maze.get(Pos::new(3, 4)), Tile::Exit);
        assert_eq!(maze.get(Pos::new(2, 3)), Tile::Portal('B', 'C'));
        assert_eq!(maze.get(Pos::new(4, 3)), Tile::Portal('D', 'E'));

        let maze = Maze::new(
            "
   B   
   C   
  #.#  
ZZ...AA
  #.#  
   D   
   E   
"
            .trim_matches('\n'),
        );
        assert_eq!(maze.get(Pos::new(4, 3)), Tile::Entrance);
        assert_eq!(maze.get(Pos::new(2, 3)), Tile::Exit);
        assert_eq!(maze.get(Pos::new(3, 2)), Tile::Portal('B', 'C'));
        assert_eq!(maze.get(Pos::new(3, 4)), Tile::Portal('D', 'E'));

        let maze = Maze::new(
            "
   Z   
   Z   
  #.#  
CC...BB
  #.#  
   A   
   A   
"
            .trim_matches('\n'),
        );
        assert_eq!(maze.get(Pos::new(3, 4)), Tile::Entrance);
        assert_eq!(maze.get(Pos::new(3, 2)), Tile::Exit);

        let maze = Maze::new(
            "
   B   
   B   
  #.#  
AA...ZZ
  #.#  
   C   
   C   
"
            .trim_matches('\n'),
        );
        assert_eq!(maze.get(Pos::new(2, 3)), Tile::Entrance);
        assert_eq!(maze.get(Pos::new(4, 3)), Tile::Exit);
    }

    #[test]
    fn calculates_min_steps_for_simple_cases() {
        let maze = Maze::new(
            "
      
      
  ##  
AA..ZZ
  ##  
      
      
"
            .trim_matches('\n'),
        );
        assert_eq!(maze.min_steps(), 1);

        let maze = Maze::new(
            "
       
       
  ###  
AA...ZZ
  ###  
       
       
"
            .trim_matches('\n'),
        );
        assert_eq!(maze.min_steps(), 2);

        let maze = Maze::new(
            "
         
         
  #####  
  #...#  
  #.#.#  
AA..#..ZZ
  #...#  
  #####  
         
         
"
            .trim_matches('\n'),
        );
        assert_eq!(maze.min_steps(), 6);
    }

    #[test]
    fn calculates_min_steps_for_simple_case_with_portal() {
        let maze = Maze::new(
            "
             
             
  #########  
AA..BC BC..ZZ
  #########  
             
             
"
            .trim_matches('\n'),
        );
        assert_eq!(maze.min_steps(), 3);
    }

    #[test]
    fn calculates_min_steps_for_examples() {
        let maze = Maze::new(
            "
         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       
"
            .trim_matches('\n'),
        );
        assert_eq!(maze.min_steps(), 23);

        let maze = Maze::new(
            "
                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               
"
            .trim_matches('\n'),
        );
        assert_eq!(maze.min_steps(), 58);
    }
}
