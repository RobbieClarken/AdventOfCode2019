use intcode_computer::Computer;

const ROWS: usize = 1000;
const COLUMNS: usize = 1000;

const EMPTY: u8 = 0;
const BLOCK: u8 = 2;

fn main() {
    challenge_1();
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
    let count = count_tile_type(&screen, BLOCK);
    println!("Challenge 1: Block tiles left on screen = {}", count);
}

struct Screen {
    state: Vec<Vec<u8>>,
}

impl Screen {
    fn new() -> Self {
        let mut state = Vec::new();
        state.resize_with(ROWS, || {
            let mut row = Vec::new();
            row.resize(COLUMNS, EMPTY);
            row
        });
        Self { state }
    }

    fn process(&mut self, x: usize, y: usize, tile_id: u8) {
        self.state[y][x] = tile_id;
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.state[y][x]
    }
}

fn count_tile_type(screen: &Screen, tile_id: u8) -> u32 {
    let mut count = 0;
    for y in 0..ROWS {
        for x in 0..COLUMNS {
            if screen.get(x, y) == tile_id {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod test_day_13 {
    use super::*;

    const WALL: u8 = 1;

    #[test]
    fn screen_defaults_to_empty() {
        let screen = Screen::new();
        assert_eq!(screen.get(1, 2), EMPTY);
    }

    #[test]
    fn screen_handles_input() {
        let mut screen = Screen::new();
        screen.process(1, 2, BLOCK);
        assert_eq!(screen.get(1, 2), BLOCK);
    }

    #[test]
    fn counts_tiles() {
        let mut screen = Screen::new();
        screen.process(0, 0, BLOCK);
        screen.process(1, 0, WALL);
        screen.process(2, 0, BLOCK);
        screen.process(COLUMNS - 1, ROWS - 1, BLOCK);
        assert_eq!(count_tile_type(&screen, BLOCK), 3);
    }
}
