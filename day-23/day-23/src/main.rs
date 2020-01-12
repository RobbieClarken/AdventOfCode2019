use intcode_computer::Computer;
use std::collections::VecDeque;

fn main() {
    challenge_1();
}

fn challenge_1() {
    let mut computers: Vec<_> = (0..50).map(|_| Computer::load_from_file("input")).collect();
    let mut queue: VecDeque<(usize, Vec<i64>)> = VecDeque::new();
    for addr in 0..50 {
        queue.push_back((addr, vec![addr as i64, -1]));
    }
    let answer = 'outer: loop {
        let (addr, packets) = queue.pop_front().expect("ran out of packets on queue");
        let computer = &mut computers[addr];
        let (mut output, _) = computer.run(packets);
        while !output.is_empty() {
            let target_addr = output.remove(0) as usize;
            let x = output.remove(0);
            let y = output.remove(0);
            if target_addr == 255 {
                break 'outer y;
            }
            queue.push_back((target_addr, vec![x, y]));
        }
    };
    println!(
        "Challenge 1: The first Y value sent to address 255 = {}",
        answer
    );
}
