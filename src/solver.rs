use std::collections::{HashMap, HashSet};

use crate::game::*;

pub fn search(field: &Field) -> Option<Vec<(usize, bool)>> {
    let colors_num = field.all_colors().len();

    let mut closed = HashSet::new();
    let mut open = vec![(field.clone(), Vec::new())];

    while let Some((field, path)) = open.pop() {
        closed.insert(field.clone());

        let all_blobs = field.all_blobs();

        if all_blobs.len() == colors_num {
            return Some(
                path.into_iter()
                    .map(|(jelly_index, to_right)| (jelly_index as usize, to_right))
                    .collect(),
            );
        }

        for i in 0..all_blobs.len() {
            for to_right in [true, false] {
                let mut new_field = field.clone();
                let jelly_index = all_blobs[i].1[0];
                if !new_field.move_jelly(jelly_index, to_right) {
                    continue;
                }
                if closed.contains(&new_field) {
                    continue;
                }
                let mut new_path = path.clone();
                new_path.push((jelly_index as u16, to_right));
                // let i = open.binary_search_by(|a| new_path.len().cmp(&a.1.len())).unwrap_or_else(|i| i);
                // open.insert(i, (new_jellies, new_path));
                open.push((new_field, new_path));
            }
        }
    }

    None
}

pub fn search_shortest(field: &Field) -> Option<Vec<(usize, bool)>> {
    let colors_num = field.all_colors().len();

    let mut closed = HashMap::new();
    let mut open = vec![(field.clone(), Vec::new())];
    let mut result: Option<Vec<(u16, bool)>> = None;

    let sort_interval = 32;
    let mut next_sort_count = sort_interval;

    while let Some((field, path)) = open.pop() {
        closed.insert(field.clone(), path.clone());

        if result
            .as_ref()
            .map(|p| p.len() <= path.len())
            .unwrap_or(false)
        {
            continue;
        }

        let all_blobs = field.all_blobs();

        if all_blobs.len() == colors_num {
            result = Some(path);
            continue;
        }

        for i in 0..all_blobs.len() {
            for to_right in [true, false] {
                let mut new_field = field.clone();
                let jelly_index = all_blobs[i].1[0];
                if !new_field.move_jelly(jelly_index, to_right) {
                    continue;
                }
                if let Some(p) = closed.get_mut(&new_field) {
                    if p.len() <= path.len() + 1 {
                        continue;
                    }
                    *p = path.clone();
                    p.push((jelly_index as u16, to_right));
                }
                let mut new_path = path.clone();
                new_path.push((jelly_index as u16, to_right));
                open.push((new_field, new_path));
                next_sort_count -= 1;
            }
        }

        if next_sort_count <= 0 {
            open.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
            next_sort_count = sort_interval;
        }
    }

    result.map(|path| {
        path.into_iter()
            .map(|(jelly_index, to_right)| (jelly_index as usize, to_right))
            .collect()
    })
}
