use yugo_puzzle_solver::game::*;

fn main() {
    //     let (field, mut jellies) = game_from_str(
    //         r"
    // XXXXXXXX
    // X0 0   X
    // X 1 2  X
    // X 12X  X
    // X      X
    // X      X
    // X      X
    // XXXXXXXX"
    //             .trim(),
    //     );

    //     field.draw_with_jellies(&jellies);

    //     dbg!(is_end(
    //         all_blobs(&jellies).len(),
    //         all_colors(&jellies).len()
    //     ));

    //     dbg!(move_jelly(&field, &mut jellies, 0, true));
    //     field.draw_with_jellies(&jellies);

    //     dbg!(is_end(
    //         all_blobs(&jellies).len(),
    //         all_colors(&jellies).len()
    //     ));

    //     dbg!(move_jelly(&field, &mut jellies, 2, true));
    //     field.draw_with_jellies(&jellies);

    //     let (field, mut jellies) = game_from_str(
    //         r"
    // XXXXXXXX
    // X      X
    // X      X
    // X      X
    // X      X
    // X  0   X
    // X  0   X
    // X011   X
    // X011   X
    // XXXXXXXX"   .trim(),
    //     );
    //     dbg!(move_jelly(&field, &mut jellies, 3, true));
    //     field.draw_with_jellies(&jellies);

    yugo_puzzle_solver::solve_and_print(
        r"
XXXXXXXX
X 1    X
X 1    X
X 0    X
X 0    X
X 1    X
X 1    X
X 0    X
X 0    X
XXXXXXXX",
    );

//     yugo_puzzle_solver::solve_and_print(
//         r"
// XXXXXXXXXXXXXXXX
// X          1   X
// X          1   X
// X          XX  X
// X          XX  X
// X              X
// X              X
// X 02   2     0 X
// X 02   2     0 X
// XXXXX1 XX XXXXXX
// XXXXX1 XX XXXXXX
// XXXXXXXXXXXXXXXX",
//     );
    yugo_puzzle_solver::solve_and_print(
        r"
XXXXXXXXXXXXXXX
X01           X
X01           X
X011     111  X
X011     111  X
XXXX     222  X
XXXX    X222X X
XXXX    X212X X
XXXX    X212X X
XXXX     000  X
XXXX     000  X
XXXXX    000  X
XXXXX    000  X
XXXXX   XXXXXXX
XXXXX  XXXXXXXX
XXXXXXXXXXXXXXX",
    );
}
