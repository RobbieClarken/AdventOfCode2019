use std::collections::{HashMap, VecDeque};
use std::time::Instant;

type CollectionOrder = Vec<(usize, char)>;

fn main() {
    let t0 = Instant::now();
    challenge_1();
    challenge_2();
    println!("Time taken: {}", t0.elapsed().as_millis());
}

fn challenge_1() {
    let input = std::fs::read_to_string("input").unwrap();
    let map = Map::new(&input);
    let steps = min_steps(map);
    println!("Challenge 1: Steps to collect all keys: {}", steps);
}

fn challenge_2() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut map = Map::new(&input);
    apply_entrance_correction(&mut map);
    let steps = min_steps(map);
    println!("Challenge 2: Steps to collect all keys: {}", steps);
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

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Entrance => '@',
            Wall => '#',
            Passage => '.',
            Key(c) => c,
            Door(c) => c,
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

    fn set(&mut self, pos: Pos, tile: Tile) {
        self.tiles[pos.y][pos.x] = tile;
    }

    fn entrances(&self) -> Vec<Pos> {
        self.iter()
            .filter(|(_, tile)| *tile == &Entrance)
            .map(|(position, _)| position)
            .collect()
    }

    fn unlock(&mut self, key: char) {
        for (_, tile) in self.iter_mut() {
            if (tile == &Key(key)) | (tile == &Door(key.to_ascii_uppercase())) {
                *tile = Passage;
            }
        }
    }

    fn unlock_many<I>(&mut self, keys: I)
    where
        I: Iterator<Item = char>,
    {
        for key in keys {
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

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut row = 0;
        for (pos, tile) in self.iter() {
            if pos.y != row {
                writeln!(f)?;
                row = pos.y;
            }
            write!(f, "{}", Into::<char>::into(*tile))?;
        }
        Ok(())
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

fn last_positions(
    collect_order: &[(usize, char)],
    entrances: &[Pos],
    key_locations: &HashMap<char, Pos>,
) -> Vec<Pos> {
    let num_robots = entrances.len();
    let mut last_keys = Vec::with_capacity(num_robots);
    last_keys.resize(num_robots, None);
    for (robot, key) in collect_order {
        last_keys[*robot] = Some(key);
    }
    last_keys
        .iter()
        .zip(entrances)
        .map(|(k, entrance)| {
            if let Some(k) = k {
                *key_locations.get(k).unwrap()
            } else {
                *entrance
            }
        })
        .collect()
}

fn min_steps(orig_map: Map) -> usize {
    let key_locations = orig_map.key_locations();
    let mut collect_orders: VecDeque<(CollectionOrder, usize)> = Default::default();
    let entrances = orig_map.entrances();
    for (robot, entrance) in entrances.iter().enumerate() {
        for (key, path) in Tracker::paths_to_keys(&orig_map, *entrance) {
            collect_orders.push_back((vec![(robot, key)], path.len()));
        }
    }
    for next_len in 2..=key_locations.len() {
        while collect_orders[0].0.len() < next_len {
            let (order, steps) = collect_orders.pop_front().unwrap();
            let mut map = orig_map.clone();
            map.unlock_many(order.iter().map(|(_, key)| *key));
            for (robot, robot_pos) in last_positions(&order, &entrances, &key_locations)
                .iter()
                .enumerate()
            {
                for (key, path) in Tracker::paths_to_keys(&map, *robot_pos) {
                    let mut new_order = order.clone();
                    new_order.push((robot, key));
                    collect_orders.push_back((new_order, steps + path.len()));
                }
            }
        }
        let mut best: HashMap<Vec<Vec<char>>, (CollectionOrder, usize)> = Default::default();
        for (order, steps) in &collect_orders {
            let num_robots = entrances.len();
            let mut sorted: Vec<Vec<char>> = Vec::with_capacity(num_robots);
            for robot in 0..num_robots {
                let mut order_for_robot: Vec<_> = order
                    .iter()
                    .filter(|(r, _)| robot == *r)
                    .map(|(_, key)| *key)
                    .collect();
                if !order_for_robot.is_empty() {
                    let last = order_for_robot.pop().unwrap();
                    order_for_robot.sort();
                    order_for_robot.push(last);
                }
                sorted.push(order_for_robot);
            }
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

fn apply_entrance_correction(map: &mut Map) {
    let center = *map.entrances().first().unwrap();
    map.set(center, Wall);
    map.set(Pos::new(center.x - 1, center.y), Wall);
    map.set(Pos::new(center.x + 1, center.y), Wall);
    map.set(Pos::new(center.x, center.y - 1), Wall);
    map.set(Pos::new(center.x, center.y + 1), Wall);
    map.set(Pos::new(center.x - 1, center.y - 1), Entrance);
    map.set(Pos::new(center.x - 1, center.y + 1), Entrance);
    map.set(Pos::new(center.x + 1, center.y - 1), Entrance);
    map.set(Pos::new(center.x + 1, center.y + 1), Entrance);
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
        assert_eq!(
            Tracker::paths_to_keys(&map, *map.entrances().first().unwrap()),
            expected_paths
        );
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
        assert_eq!(
            Tracker::paths_to_keys(&map, *map.entrances().first().unwrap()),
            expected_paths
        );
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

    #[test]
    fn applies_entrance_correction() {
        let mut map = Map::new(
            "\
#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######",
        );
        apply_entrance_correction(&mut map);
        assert_eq!(
            map.to_string(),
            "\
#######
#a.#Cd#
##@#@##
#######
##@#@##
#cB#Ab#
#######"
        );
        assert_eq!(map.entrances().len(), 4);
    }

    #[test]
    fn calculates_steps_for_multi_robot_case() {
        let map = Map::new(
            "\
#######
#a.#Cd#
##@#@##
#######
##@#@##
#cB#Ab#
#######",
        );
        assert_eq!(min_steps(map), 8);

        let map = Map::new(
            "\
###############
#d.ABC.#.....a#
######@#@######
###############
######@#@######
#b.....#.....c#
###############
",
        );
        assert_eq!(min_steps(map), 24);

        let map = Map::new(
            "\
#############
#DcBa.#.GhKl#
#.###@#@#I###
#e#d#####j#k#
###C#@#@###J#
#fEbA.#.FgHi#
#############
",
        );
        assert_eq!(min_steps(map), 32);

        let map = Map::new(
            "\
#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba@#@BcIJ#
#############
#nK.L@#@G...#
#M###N#H###.#
#o#m..#i#jk.#
#############
",
        );
        assert_eq!(min_steps(map), 72);
    }
}
