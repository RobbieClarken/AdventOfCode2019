use std::collections::{HashMap, VecDeque};
use std::time::Instant;

fn main() {
    let t0 = Instant::now();
    challenge_1();
    println!("Time taken: {}", t0.elapsed().as_millis());
}

fn challenge_1() {
    let input = std::fs::read_to_string("input").unwrap();
    let map = Map::new(&input);
    let steps = min_steps(map);
    println!("Steps to collect all keys: {}", steps);
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
}

impl Map {
    fn new(map_as_str: &str) -> Self {
        let tiles = map_as_str
            .lines()
            .map(|l| l.chars().map(|c| c.into()).collect())
            .collect();
        Self { tiles }
    }

    fn iter(&self) -> impl Iterator<Item = (Pos, &Tile)> {
        self.tiles.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, tile)| (Pos::new(x, y), tile))
        })
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = (Pos, &mut Tile)> {
        self.tiles.iter_mut().enumerate().flat_map(|(y, row)| {
            row.iter_mut()
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

    fn entrance(&self) -> Pos {
        for (position, tile) in self.iter() {
            if tile == &Entrance {
                return position;
            }
        }
        unreachable!("maps must have an entrance");
    }

    fn unlock(&mut self, key: char) {
        for (_, tile) in self.iter_mut() {
            if (tile == &Key(key)) | (tile == &Door(key.to_ascii_uppercase())) {
                *tile = Passage;
            }
        }
    }

    fn unlock_many(&mut self, keys: &str) {
        for key in keys.chars() {
            self.unlock(key);
        }
    }

    fn key_locations(&self) -> HashMap<char, Pos> {
        let mut locations = HashMap::new();
        for (pos, &tile) in self.iter() {
            if let Key(c) = tile {
                locations.insert(c, pos);
            }
        }
        locations
    }
}

struct Tracker<'a> {
    map: &'a Map,
    index: usize,
    steps: Vec<Pos>,
    distances: HashMap<Pos, usize>,
}

impl<'a> Tracker<'a> {
    fn paths_to_keys(map: &'a Map, pos: Pos) -> HashMap<char, Vec<Pos>> {
        let mut tracker = Self::new(map, pos);
        tracker.find_paths_to_keys()
    }

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

    fn find_paths_to_keys(&mut self) -> HashMap<char, Vec<Pos>> {
        // Note: as an optimisation, if the shortest path to key `a` is via key `b` we only return
        // the path to key `b` because it doesn't make sense to collect `a` before `b`.
        let mut keys: HashMap<char, (Pos, usize)> = Default::default();
        let x_max = self.map.x_max();
        let y_max = self.map.y_max();
        loop {
            let next = self.next();
            if next.is_none() {
                break;
            }
            let (pos, distance) = next.unwrap();
            for new_pos in pos.neighbours(x_max, y_max) {
                if self.distances.get(&new_pos).is_some() {
                    continue;
                }
                match self.map.get(new_pos) {
                    Wall | Door(_) => continue,
                    Key(k) => {
                        keys.insert(k, (new_pos, distance + 1));
                    }
                    Passage | Entrance => {
                        self.insert(new_pos, distance + 1);
                    }
                }
            }
        }
        let mut out = HashMap::with_capacity(keys.len());
        for (key, (mut pos, distance)) in keys {
            let mut reverse_path = Vec::with_capacity(distance);
            reverse_path.push(pos);
            'outer: for target_distance in (1..distance).rev() {
                for new_pos in pos.neighbours(x_max, y_max) {
                    if self.distances.get(&new_pos) == Some(&target_distance) {
                        reverse_path.push(new_pos);
                        pos = new_pos;
                        continue 'outer;
                    }
                }
                unreachable!();
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

fn min_steps(orig_map: Map) -> usize {
    let key_locations = orig_map.key_locations();
    let mut collect_orders: VecDeque<(String, usize)> = Default::default();
    for (key, path) in Tracker::paths_to_keys(&orig_map, orig_map.entrance()) {
        collect_orders.push_back((key.to_string(), path.len()));
    }
    for next_len in 2..=key_locations.len() {
        while collect_orders[0].0.len() < next_len {
            let (order, steps) = collect_orders.pop_front().unwrap();
            let last_key = order.chars().last().unwrap();
            let last_key_pos = *key_locations.get(&last_key).unwrap();
            let mut map = orig_map.clone();
            map.unlock_many(&order);
            for (key, path) in Tracker::paths_to_keys(&map, last_key_pos) {
                let mut new_order = order.clone();
                new_order.push(key);
                collect_orders.push_back((new_order, steps + path.len()));
            }
        }
        let mut best: HashMap<String, (String, usize)> = Default::default();
        for (order, steps) in &collect_orders {
            let mut sorted: Vec<_> = order.chars().collect();
            let last_key = sorted.pop().unwrap();
            sorted.sort();
            let mut sorted: String = sorted.into_iter().collect();
            sorted.push(last_key);
            best.entry(sorted)
                .and_modify(|e| {
                    if *steps < e.1 {
                        *e = (order.clone(), *steps);
                    }
                })
                .or_insert((order.clone(), *steps));
        }
        let best: Vec<_> = best.values().cloned().collect();
        collect_orders = best.into();
    }
    collect_orders
        .iter()
        .map(|(_, steps)| *steps)
        .min()
        .unwrap()
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
        assert_eq!(Tracker::paths_to_keys(&map, map.entrance()), expected_paths);
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
        assert_eq!(Tracker::paths_to_keys(&map, map.entrance()), expected_paths);
    }

    #[test]
    fn unlocks_door() {
        let mut map = Map::new("aAbB");
        map.unlock('a');
        assert_eq!(map.get(Pos::new(0, 0)), Passage);
        assert_eq!(map.get(Pos::new(1, 0)), Passage);
        assert_eq!(map.get(Pos::new(2, 0)), Key('b'));
        assert_eq!(map.get(Pos::new(3, 0)), Door('B'));
    }

    #[test]
    fn handles_two_equal_paths_to_key() {
        let map = Map::new(
            "\
#####
#...#
#@#a#
#...#
#####
",
        );
        assert_eq!(min_steps(map), 4);
    }

    #[test]
    fn finds_min_steps_to_get_all_keys_for_examples() {
        let map = Map::new(
            "\
#########
#b.A.@.a#
#########
",
        );
        assert_eq!(min_steps(map), 8);

        let map = Map::new(
            "\
########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################
",
        );
        assert_eq!(min_steps(map), 86);

        let map = Map::new(
            "\
########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################
",
        );
        assert_eq!(min_steps(map), 132);

        let map = Map::new(
            "\
########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################
",
        );
        assert_eq!(min_steps(map), 81);
    }

    #[test]
    #[ignore]
    fn finds_min_steps_to_get_all_keys_for_slow_examples() {
        let map = Map::new(
            "\
#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################
",
        );
        assert_eq!(min_steps(map), 136);
    }
}
