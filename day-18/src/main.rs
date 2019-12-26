#![allow(dead_code)]
use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let map = Map::new(&input);
    for (k, path) in paths_to_keys(&map) {
        println!("{}: {}", k, path.len());
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Entrance,
    Wall,
    Passage,
    Key(char),
    Door(char),
}

use Tile::*;

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '@' => Entrance,
            '#' => Wall,
            '.' => Passage,
            'a'..='z' => Key(c),
            'A'..='Z' => Door(c),
            _ => unreachable!("{} cannot be converted to Tile", c),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    foo: Vec<Tile>,
}

impl Map {
    fn new(map_as_str: &str) -> Self {
        let tiles = map_as_str
            .lines()
            .map(|l| l.chars().map(|c| c.into()).collect())
            .collect();
        Self {
            tiles,
            foo: Vec::new(),
        }
    }

    fn iter(&self) -> impl Iterator<Item = (Pos, &Tile)> {
        self.tiles.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, tile)| (Pos::new(x, y), tile))
        })
    }

    fn x_max(&self) -> usize {
        self.tiles[0].len()
    }

    fn y_max(&self) -> usize {
        self.tiles.len()
    }

    fn get(&self, pos: Pos) -> Tile {
        self.tiles[pos.y][pos.x]
    }

    fn entrance(&self) -> Option<Pos> {
        for (position, tile) in self.iter() {
            if tile == &Entrance {
                return Some(position);
            }
        }
        None
    }
}

struct Tracker<'a> {
    map: &'a Map,
    index: usize,
    steps: Vec<Pos>,
    distances: HashMap<Pos, usize>,
}

impl<'a> Tracker<'a> {
    fn new(map: &'a Map, pos: Pos) -> Self {
        let mut tracker = Self {
            map,
            index: 0,
            steps: Default::default(),
            distances: Default::default(),
        };
        tracker.insert(pos, 0);
        tracker
    }

    fn insert(&mut self, pos: Pos, distance: usize) {
        self.steps.push(pos);
        self.distances.insert(pos, distance);
    }

    fn next(&mut self) -> Option<(Pos, usize)> {
        if self.index < self.steps.len() {
            let pos = self.steps[self.index];
            let distance = *self.distances.get(&pos).unwrap();
            self.index += 1;
            Some((pos, distance))
        } else {
            None
        }
    }

    fn find_keys(&mut self) -> HashMap<char, Vec<Pos>> {
        let mut keys: HashMap<char, Pos> = Default::default();
        loop {
            let next = self.next();
            if next.is_none() {
                break;
            }
            let (pos, distance) = next.unwrap();
            let mut to_insert = Vec::new();
            for new_pos in pos.neighbours(self.map.x_max(), self.map.y_max()) {
                if self.distances.get(&new_pos).is_some() {
                    continue;
                }
                match self.map.get(new_pos) {
                    Wall | Door(_) => continue,
                    Key(k) => {
                        keys.insert(k, new_pos);
                        to_insert.push(new_pos);
                    }
                    Passage | Entrance => {
                        to_insert.push(new_pos);
                    }
                }
            }
            for new_pos in &to_insert {
                self.insert(*new_pos, distance + 1);
            }
        }
        let mut out = HashMap::new();
        for (key, mut pos) in keys {
            let mut reverse_path = vec![pos];
            let distance = *self.distances.get(&pos).unwrap();
            for target_distance in (1..=distance).rev() {
                for new_pos in pos.neighbours(self.map.x_max(), self.map.y_max()) {
                    if self.distances.get(&new_pos) == Some(&target_distance) {
                        reverse_path.push(new_pos);
                        pos = new_pos;
                    }
                }
            }
            out.insert(key, reverse_path.iter().rev().cloned().collect());
        }
        out
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    fn neighbours(&self, x_max: usize, y_max: usize) -> Vec<Self> {
        let (x, y) = (self.x, self.y);
        let mut out = Vec::new();
        if x > 0 {
            out.push(Self::new(x - 1, y));
        }
        if y > 0 {
            out.push(Self::new(x, y - 1));
        }
        if x < x_max {
            out.push(Self::new(x + 1, y));
        }
        if y < y_max {
            out.push(Self::new(x, y + 1));
        }
        out
    }
}

fn paths_to_keys(map: &Map) -> HashMap<char, Vec<Pos>> {
    let pos = map.entrance().unwrap();
    let mut tracker = Tracker::new(&map, pos);
    tracker.find_keys()
}

#[cfg(test)]
mod test_day_18 {
    use super::*;

    #[test]
    fn finds_path_to_single_key() {
        let map = Map::new(
            "\
#########
#b.A.@.a#
#########
",
        );
        let mut expected_paths: HashMap<char, Vec<Pos>> = Default::default();
        expected_paths.insert('a', vec![Pos::new(6, 1), Pos::new(7, 1)]);
        assert_eq!(paths_to_keys(&map), expected_paths);
    }

    #[test]
    fn finds_path_to_multiple_keys() {
        let map = Map::new(
            "\
########
#b..@.a#
########
",
        );
        let mut expected_paths: HashMap<char, Vec<Pos>> = Default::default();
        expected_paths.insert('a', vec![Pos::new(5, 1), Pos::new(6, 1)]);
        expected_paths.insert('b', vec![Pos::new(3, 1), Pos::new(2, 1), Pos::new(1, 1)]);
        assert_eq!(paths_to_keys(&map), expected_paths);
    }
}
