use advent2023::solutions;
use std::env;

fn read_day_from_args() -> Option<u8> {
    env::args().nth(1).and_then(|arg| arg.parse().ok())
}

fn main() {
    let solutions = solutions();
    if let Some(solution) = read_day_from_args().and_then(|day| solutions.get(&day)) {
        solution.execute()
    }
}
