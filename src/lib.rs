pub mod game;
pub mod solver;

pub fn solve_and_print(str: &str) {
    let mut field = game::Field::from_str(str.trim());
    let t = std::time::Instant::now();
    let result = solver::search_shortest(&field);
    println!("Time: {:?}", t.elapsed());
    if let Some(result) = result {
        println!("Found solution:");
        field.draw();
        println!("Moves ({}):", result.len());
        for (pos, to_right) in result {
            println!("({}, {})", pos % field.width, pos / field.width);
            field.move_jelly(pos, to_right);
            field.draw();
        }
    } else {
        println!("No solution found");
    }
}
