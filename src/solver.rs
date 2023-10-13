use crate::game::*;

pub fn search(field: &Field, mut jellies: Vec<Jelly>) -> Option<Vec<([i8; 2], bool)>> {
    let colors_num = all_colors(&jellies).len();
    jellies.sort();
    let mut closed: Vec<Vec<Jelly>> = Vec::new();
    let mut open = vec![(jellies.clone(), Vec::new())];
    let initial_jellies = jellies.clone();

    while let Some((jellies, path)) = open.pop() {
        closed.push(jellies.clone());

        let all_blobs = all_blobs(&jellies);

        if is_end(all_blobs.len(), colors_num) {
            return Some(reconstruct_path(field, initial_jellies, path));
        }

        for i in 0..all_blobs.len() {
            for to_right in [true, false] {
                let mut new_jellies = jellies.clone();
                let jelly_index = all_blobs[i][0];
                if move_jelly(field, &mut new_jellies, jelly_index, to_right) {
                    if !closed.contains(&new_jellies) {
                        let mut new_path = path.clone();
                        new_path.push((jelly_index as u8, to_right));
                        // let i = open.binary_search_by_key(&(usize::MAX - new_path.len()), |o| usize::MAX - o.1.len()).unwrap_or_else(|i| i);
                        // open.insert(i, (new_jellies, new_path));
                        open.push((new_jellies, new_path));
                    }
                }
            }
        }
    }

    None
}

fn reconstruct_path(
    field: &Field,
    mut jellies: Vec<Jelly>,
    path: Vec<(u8, bool)>,
) -> Vec<([i8; 2], bool)> {
    let mut ret = Vec::new();
    for (jelly_index, to_right) in path {
        let jelly = jellies[jelly_index as usize];
        ret.push(([jelly.x, jelly.y], to_right));
        move_jelly(field, &mut jellies, jelly_index as usize, to_right);
    }
    ret
}
