#[allow(unused)]
fn bench(desc: &str, workload: impl FnOnce()) {
    use std::time::Instant;

    let before = Instant::now();
    workload();
    println!("{desc}: {:.2?}", before.elapsed());
}

#[allow(unused)]
fn debug<T: std::fmt::Debug>(ctx: T, delay: u64) {
    if delay > 0 {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    print!("{ctx:?}");

    if delay > 0 {
        use std::{thread, time::Duration};
        thread::sleep(Duration::from_millis(delay));
    }
}

fn parse_input(input: &str) -> Vec<u32> {
    todo!()
}

fn part1(input: &str) -> Option<u32> {
    todo!()
}

fn part2(input: &str) -> Option<u32> {
    todo!()
}

fn main() {
    let input0 = include_str!("input0.txt");
    let input1 = include_str!("input1.txt");

    println!("{:?}", part1(input0));
    println!("{:?}", part1(input1));

    println!("{:?}", part2(input0));
    println!("{:?}", part2(input1));
}