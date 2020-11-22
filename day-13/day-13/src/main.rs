use intcode_computer::Computer;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use std::cmp::{max, Ordering};
use std::env;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

const ROWS: usize = 1000;
const COLUMNS: usize = 1000;

fn main() {
    let mut args = env::args();
    match args.nth(1) {
        Some(s) if s == "1" => {
            challenge_1();
        }
        Some(s) if s == "2" => {
            challenge_2();
        }
        _ => {
            usage();
        }
    }
}

fn usage() {
    let mut args = env::args();
    println!("Usage:");
    println!("    {} NUMBER", args.next().unwrap());
    println!();
    println!("Arguments:");
    println!("    number: Which part of the puzzle to solve (1 or 2)");
    std::process::exit(1);
}

fn challenge_1() {
    let mut screen = Screen::new();
    let mut computer = Computer::load_from_file("input");
    loop {
        let (out, complete) = computer.run(vec![]);
        assert_eq!(out.len() % 3, 0);
        for i in (0..out.len()).step_by(3) {
            screen.process(out[i] as usize, out[i + 1] as usize, out[i + 2] as u8);
        }
        if complete {
            break;
        }
    }
    let count = count_tile_type(&screen, Tile::Block);
    println!("Challenge 1: Block tiles left on screen = {}", count);
}

fn challenge_2() {
    let mut player = Player::new();
    let mut screen = Screen::new();
    let mut computer = Computer::load_from_file("input");
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // set number of quarters to 2
    computer.set_address(0, 2);

    let mut score = 0;
    let mut joystick = 0;
    loop {
        let (out, complete) = computer.run(vec![joystick]);
        assert_eq!(out.len() % 3, 0);
        for i in (0..out.len()).step_by(3) {
            if out[i] == -1 {
                score = out[i + 2];
                continue;
            }
            screen.process(out[i] as usize, out[i + 1] as usize, out[i + 2] as u8);
        }
        let game_state = screen.output();
        handle.write_all(b"\x1b[2J\x1b[1;1H").unwrap();
        handle.write_all(game_state.as_bytes()).unwrap();
        handle.flush().unwrap();
        sleep(Duration::from_millis(1));
        joystick = player.process(&game_state) as i64;
        if complete {
            break;
        }
    }
    println!("Challenge 2: Final score = {}", score);
}

#[derive(FromPrimitive, ToPrimitive, Debug, PartialEq, Clone, Copy)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

impl Tile {
    fn from_u8(v: u8) -> Self {
        FromPrimitive::from_u8(v).unwrap()
    }
}

struct Screen {
    state: Vec<Vec<Tile>>,
    max_x: usize,
    max_y: usize,
}

impl Screen {
    fn new() -> Self {
        let mut state = Vec::new();
        state.resize_with(ROWS, || {
            let mut row = Vec::new();
            row.resize(COLUMNS, Tile::Empty);
            row
        });
        Self {
            state,
            max_x: 0,
            max_y: 0,
        }
    }

    fn process(&mut self, x: usize, y: usize, tile_id: u8) {
        self.state[y][x] = Tile::from_u8(tile_id);
        self.max_x = max(self.max_x, x);
        self.max_y = max(self.max_y, y);
    }

    fn get(&self, x: usize, y: usize) -> Tile {
        self.state[y][x]
    }

    fn output(&self) -> String {
        let mut out = String::new();
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                let chr = match self.get(x, y) {
                    Tile::Empty => ' ',
                    Tile::Wall => 'X',
                    Tile::Block => '#',
                    Tile::Paddle => '=',
                    Tile::Ball => 'o',
                };
                out = format!("{}{}", out, chr);
            }
            out = format!("{}\n", out);
        }
        out
    }
}

fn count_tile_type(screen: &Screen, tile: Tile) -> u32 {
    let mut count = 0;
    for y in 0..ROWS {
        for x in 0..COLUMNS {
            if screen.get(x, y) == tile {
                count += 1;
            }
        }
    }
    count
}

struct Player;

impl Player {
    fn new() -> Self {
        Self {}
    }

    fn process(&mut self, state: &str) -> i8 {
        let (ball_x, _) = self.find_char(state, 'o').unwrap();
        let (paddle_x, _) = self.find_char(state, '=').unwrap();
        match ball_x.cmp(&paddle_x) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            Ordering::Equal => 0,
        }
    }

    fn find_char(&self, state: &str, chr: char) -> Option<(i32, i32)> {
        for (y, row) in state.lines().enumerate() {
            if let Some(x) = row.find(chr) {
                return Some((x as i32, y as i32));
            }
        }
        None
    }
}

#[cfg(test)]
mod test_day_13 {
    use super::*;

    use num_traits::ToPrimitive;

    impl Tile {
        fn as_u8(&self) -> u8 {
            ToPrimitive::to_u8(self).unwrap()
        }
    }

    #[test]
    fn screen_defaults_to_empty() {
        let screen = Screen::new();
        assert_eq!(screen.get(1, 2), Tile::Empty);
    }

    #[test]
    fn screen_handles_input() {
        let mut screen = Screen::new();
        screen.process(1, 2, 2);
        assert_eq!(screen.get(1, 2), Tile::Block);
    }

    #[test]
    fn counts_tiles() {
        let mut screen = Screen::new();
        screen.process(0, 0, 2);
        screen.process(1, 0, 1);
        screen.process(2, 0, 2);
        screen.process(COLUMNS - 1, ROWS - 1, 2);
        assert_eq!(count_tile_type(&screen, Tile::Block), 3);
    }

    #[test]
    fn generates_output_for_tile_types() {
        let mut screen = Screen::new();
        screen.process(0, 0, Tile::Empty.as_u8());
        assert_eq!(screen.output(), " \n");

        screen.process(0, 0, Tile::Wall.as_u8());
        assert_eq!(screen.output(), "X\n");

        screen.process(0, 0, Tile::Block.as_u8());
        assert_eq!(screen.output(), "#\n");

        screen.process(0, 0, Tile::Paddle.as_u8());
        assert_eq!(screen.output(), "=\n");

        screen.process(0, 0, Tile::Ball.as_u8());
        assert_eq!(screen.output(), "o\n");
    }

    #[test]
    fn generates_multiline_output() {
        let mut screen = Screen::new();
        screen.process(0, 0, Tile::Ball.as_u8());
        screen.process(1, 0, Tile::Wall.as_u8());
        screen.process(0, 1, Tile::Paddle.as_u8());
        assert_eq!(screen.output(), "oX\n= \n");
    }

    #[test]
    fn player_doesnt_move_paddle_if_below_ball() {
        let mut player = Player::new();
        let state = "
            o
            =
        ";
        assert_eq!(player.process(&state), 0);
    }

    #[test]
    fn player_follows_ball() {
        let mut player = Player::new();
        let state = "
             o
            =
        ";
        assert_eq!(player.process(&state), 1);
        let state = "
            o
             =
        ";
        assert_eq!(player.process(&state), -1);
    }
}
