fn main() {
    let stdin = std::io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    let height = buffer.trim().parse::<usize>().unwrap();
    buffer.clear();
    stdin.lines().take(height).for_each(|line| {
        buffer.push_str(&line.unwrap());
        buffer.push('\n');
    });

    yugo_puzzle_solver::solve_and_print(buffer.trim());
}
