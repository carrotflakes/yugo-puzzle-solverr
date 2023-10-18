use std::collections::HashSet;

use crate::game::*;

pub fn search(field: &Field, mut jellies: Vec<Jelly>) -> Option<Vec<([i8; 2], bool)>> {
    let colors_num = all_colors(&jellies).len();
    jellies.sort();
    let mut closed = HashSet::new();
    let mut open = vec![(jellies.clone(), Vec::new())];
    let initial_jellies = jellies.clone();

    let sort_interval = 1024;
    let mut next_sort_count = sort_interval;

    while let Some((jellies, path)) = open.pop() {
        closed.insert(jellies.clone());

        let all_blobs = all_blobs(&jellies);

        if is_end(all_blobs.len(), colors_num) {
            // eprintln!("Nodes: {}", closed.len());
            return Some(reconstruct_path(field, initial_jellies, path));
        }

        for i in 0..all_blobs.len() {
            for to_right in [true, false] {
                let mut new_jellies = jellies.clone();
                let jelly_index = all_blobs[i][0];
                if move_jelly(field, &mut new_jellies, jelly_index, to_right) {
                    new_jellies.sort();
                    if !closed.contains(&new_jellies) {
                        let mut new_path = path.clone();
                        new_path.push((jelly_index as u8, to_right));
                        // let i = open.binary_search_by(|a| new_path.len().cmp(&a.1.len())).unwrap_or_else(|i| i);
                        // open.insert(i, (new_jellies, new_path));
                        open.push((new_jellies, new_path));
                        next_sort_count -= 1;
                    }
                }
            }
        }

        if next_sort_count <= 0 {
            open.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
            next_sort_count = sort_interval;
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
        jellies.sort();
        let jelly = jellies[jelly_index as usize];
        ret.push(([jelly.x, jelly.y], to_right));
        move_jelly(field, &mut jellies, jelly_index as usize, to_right);
    }
    ret
}
