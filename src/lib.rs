pub mod game;
pub mod solver;

pub fn solve_and_print(str: &str) {
    let (field, mut jellies) = game::game_from_str(str.trim());
    let t = std::time::Instant::now();
    let result = solver::search(&field, jellies.clone());
    println!("Time: {:?}", t.elapsed());
    if let Some(result) = result {
        println!("Found solution:");
        field.draw_with_jellies(&jellies);
        println!("Moves:");
        for (pos, to_right) in result {
            println!("({}, {})", pos[0], pos[1]);
            let jelly_index = jellies
                .iter()
                .position(|j| j.x == pos[0] && j.y == pos[1])
                .unwrap();
            game::move_jelly(&field, &mut jellies, jelly_index, to_right);
            field.draw_with_jellies(&jellies);
        }
    } else {
        println!("No solution found");
    }
}
