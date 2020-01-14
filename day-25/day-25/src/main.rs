use intcode_computer::Computer;
use std::io;

const WIDTH: usize = 50;
const HEIGHT: usize = 20;

fn main() -> io::Result<()> {
    let mut computer = Computer::load_from_file("input");
    let mut map = Map::new();
    let mut history: Vec<String> = Vec::new();
    run_commands_from_file(&mut computer, &mut map, &mut history);
    // run_interactive(&mut computer, &mut map, &mut history)?;
    // find_combination_of_items_to_pass_floor(&mut computer, &mut map, &mut history);
    Ok(())
}

#[allow(dead_code)]
fn find_combination_of_items_to_pass_floor(computer: &mut Computer) {
    let items = [
        "mutex",
        "ornament",
        "astrolabe",
        "sand",
        "semiconductor",
        "shell",
        "klein bottle",
    ];
    for item_specification in 0..2usize.pow(items.len() as u32) {
        let mut commands = String::new();
        for (i, item) in items.iter().enumerate() {
            let keep = (item_specification >> i) & 1 == 1;
            if !keep {
                commands.push_str(&format!("drop {}\n", item));
            }
        }
        commands.push_str("south\n");

        let mut computer = computer.clone();
        let input = commands.chars().map(|c| c as i64).collect();
        let (output, _) = computer.run(input);
        let output: String = output.iter().map(|&c| c as u8 as char).collect();
        if !output.contains("ejected") {
            print!("{}", commands);
            break;
        }
    }
}

fn run_commands_from_file(computer: &mut Computer, map: &mut Map, history: &mut Vec<String>) {
    let commands: Vec<String> = std::fs::read_to_string("commands")
        .unwrap()
        .lines()
        .map(|l| {
            let mut l = l.to_owned();
            l.push('\n');
            l
        })
        .collect();
    for command in commands {
        map.process(&command.trim());
        history.push(command.clone());
        let input = command.chars().map(|c| c as i64).collect();
        let (output, _) = computer.run(input);
        let output: String = output.iter().map(|&c| c as u8 as char).collect();
        print!("{}", map.draw());
        print!("{}", output);
    }
}

#[allow(dead_code)]
fn run_interactive(
    computer: &mut Computer,
    map: &mut Map,
    history: &mut Vec<String>,
) -> io::Result<()> {
    let mut input: Vec<i64> = vec![];
    loop {
        let (output, _) = computer.run(input);
        let output: String = output.iter().map(|&c| c as u8 as char).collect();
        print!("{}", map.draw());
        print!("{}", output);

        let command = loop {
            let mut command = String::new();
            io::stdin().read_line(&mut command)?;
            command = String::from(match command.trim().as_ref() {
                "n" => "north\n",
                "s" => "south\n",
                "e" => "east\n",
                "w" => "west\n",
                "i" => "ignore\n",
                "h" => "history\n",
                _ => &command,
            });
            if command == "history\n" {
                println!("Command History:");
                for cmd in history.clone() {
                    print!("{}", cmd);
                }
                continue;
            }
            map.process(&command.trim());
            if command != "ignore\n" {
                break command;
            }
        };
        history.push(command.clone());
        input = command.chars().map(|c| c as i64).collect();
    }
}

struct Map {
    tiles: [[char; WIDTH]; HEIGHT],
    x: usize,
    y: usize,
    ignore_next: bool,
}

impl Map {
    fn new() -> Self {
        let mut tiles = [['.'; WIDTH]; HEIGHT];
        let x = WIDTH / 2;
        let y = HEIGHT / 2;
        tiles[y][x] = '*';
        Self {
            tiles,
            x,
            y,
            ignore_next: false,
        }
    }

    fn process(&mut self, s: &str) {
        if self.ignore_next {
            self.ignore_next = false;
            return;
        }
        match s {
            "north" => self.walk(0, -1),
            "south" => self.walk(0, 1),
            "east" => self.walk(1, 0),
            "west" => self.walk(-1, 0),
            "ignore" => {
                self.ignore_next = true;
            }
            _ => {}
        };
    }

    fn walk(&mut self, dx: isize, dy: isize) {
        self.x = (self.x as isize + dx) as usize;
        self.y = (self.y as isize + dy) as usize;
        self.tiles[self.y][self.x] = '*';
    }

    fn draw(&self) -> String {
        self.tiles
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, c)| if (x, y) == (self.x, self.y) { &'@' } else { c })
                    .chain(&['\n'])
            })
            .collect()
    }
}
