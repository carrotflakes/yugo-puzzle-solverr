pub fn move_jelly(
    field: &Field,
    jellies: &mut [Jelly],
    jelly_index: usize,
    to_right: bool,
) -> bool {
    let dx = if to_right { 1 } else { -1 };

    let all_blobs = all_blobs(jellies);
    let blob = all_blobs
        .iter()
        .position(|b| b.contains(&jelly_index))
        .unwrap();

    // Move types: (jump, slide)
    let mut blob_moves: Vec<_> = all_blobs.iter().map(|_| (false, false)).collect();
    blob_moves[blob] = (true, true);

    // Check if the jelly can jump
    'outer: while {
        let mut updated = false;

        for i in 0..all_blobs.len() {
            if !blob_moves[i].0 {
                continue;
            }
            for j in &all_blobs[i] {
                if field.is_wall(jellies[*j].x, jellies[*j].y - 1) {
                    // Cannot jump by wall
                    for i in 0..all_blobs.len() {
                        if blob_moves[i].0 {
                            blob_moves[i].0 = false;
                        }
                    }
                    break 'outer;
                }
                if let Some(j) = get_jelly_by_position(jellies, [jellies[*j].x, jellies[*j].y - 1])
                {
                    let j = all_blobs.iter().position(|b| b.contains(&j)).unwrap();
                    if !blob_moves[j].0 {
                        blob_moves[j].0 = true;
                        updated = true;
                    }
                }
            }
        }

        updated
    } {}

    // Check if the jelly can move
    'outer: while {
        let mut updated = false;

        for i in 0..all_blobs.len() {
            if !blob_moves[i].1 {
                continue;
            }
            let dy = if blob_moves[i].0 { -1 } else { 0 };
            for j in &all_blobs[i] {
                if field.is_wall(jellies[*j].x + dx, jellies[*j].y + dy) {
                    // Cannot move by wall
                    for i in 0..all_blobs.len() {
                        if blob_moves[i].1 {
                            blob_moves[i].1 = false;
                        }
                    }
                    break 'outer;
                }

                if let Some(l) =
                    get_jelly_by_position(jellies, [jellies[*j].x + dx, jellies[*j].y + dy])
                {
                    let k = all_blobs.iter().position(|b| b.contains(&l)).unwrap();
                    if blob_moves[i].0
                        && blob_moves[k].0
                        && get_jelly_by_position(jellies, [jellies[*j].x + dx, jellies[*j].y])
                            .map(|j| !all_blobs[k].contains(&j))
                            .unwrap_or(false)
                    {
                        continue;
                    }
                    if !blob_moves[k].1 {
                        blob_moves[k].1 = true;
                        updated = true;
                    }
                }
            }
        }

        updated
    } {}

    if blob_moves[blob] == (false, false) {
        return false;
    }

    // Move the jellies
    for (i, (jump, slide)) in blob_moves.iter().enumerate() {
        if *jump {
            for j in &all_blobs[i] {
                jellies[*j].y = jellies[*j].y - 1;
            }
        }
        if *slide {
            for j in &all_blobs[i] {
                jellies[*j].x = jellies[*j].x + dx;
            }
        }
    }

    // Fall the jellies
    fall_jellies(field, jellies, &all_blobs);

    true
}

pub fn fall_jellies(field: &Field, jellies: &mut [Jelly], all_blobs: &Vec<Vec<usize>>) {
    let mut blobs = (0..all_blobs.len()).collect::<Vec<_>>();

    while !blobs.is_empty() {
        // Blocked by walls
        blobs.retain(|i| {
            for j in &all_blobs[*i] {
                if field.is_wall(jellies[*j].x, jellies[*j].y + 1) {
                    return false;
                }
            }
            true
        });

        // Blocked by other jellies
        'outer: loop {
            for (blobs_i, i) in blobs.iter().copied().enumerate() {
                for j in &all_blobs[i] {
                    if let Some(j) =
                        get_jelly_by_position(jellies, [jellies[*j].x, jellies[*j].y + 1])
                    {
                        let j_is_not_falling = blobs.iter().all(|i| !all_blobs[*i].contains(&j));
                        if j_is_not_falling {
                            blobs.remove(blobs_i);
                            continue 'outer;
                        }
                    }
                }
            }
            break;
        }

        // Fall by one
        for i in &blobs {
            for j in &all_blobs[*i] {
                jellies[*j].y = jellies[*j].y + 1;
            }
        }
    }
}

fn get_jelly_by_position(jellies: &[Jelly], pos: [i8; 2]) -> Option<usize> {
    jellies
        .iter()
        .position(|jelly| jelly.x == pos[0] && jelly.y == pos[1])
}

pub fn is_end(blobs_num: usize, colors_num: usize) -> bool {
    blobs_num == colors_num
}

pub fn all_colors(jellies: &[Jelly]) -> Vec<u8> {
    let mut result = vec![];
    for jelly in jellies {
        if !result.contains(&jelly.color) {
            result.push(jelly.color);
        }
    }
    result
}

pub fn all_blobs(jellies: &[Jelly]) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = vec![];
    for i in 0..jellies.len() {
        if result.iter().any(|b| b.contains(&i)) {
            continue;
        }
        let blob = get_blob(jellies, i);
        result.push(blob.clone());
    }
    result
}

pub fn get_blob(jellies: &[Jelly], jelly_index: usize) -> Vec<usize> {
    let mut result = vec![jelly_index];
    loop {
        let mut new = vec![];
        for i in &result {
            for j in 0..jellies.len() {
                if !result.contains(&j) && is_connected(&jellies[*i], &jellies[j]) {
                    new.push(j);
                }
            }
        }
        if new.is_empty() {
            break;
        }
        new.sort();
        new.dedup();
        result.append(&mut new);
    }
    result
}

fn is_connected(jelly1: &Jelly, jelly2: &Jelly) -> bool {
    jelly1.color == jelly2.color
        && ((jelly1.x == jelly2.x && (jelly1.y == jelly2.y + 1 || jelly1.y == jelly2.y - 1))
            || (jelly1.y == jelly2.y && (jelly1.x == jelly2.x + 1 || jelly1.x == jelly2.x - 1)))
}

pub fn game_from_str(str: &str) -> (Field, Vec<Jelly>) {
    let mut walls = vec![];
    let mut width = None;
    let mut jellies = vec![];

    for (y, line) in str.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        if width.is_none() {
            width = Some(line.len());
        } else if width.unwrap() != line.len() {
            panic!("The width of the field is not consistent");
        }
        for (x, c) in line.chars().enumerate() {
            match c {
                'X' => walls.push(true),
                ' ' => walls.push(false),
                n if n.is_ascii_digit() => {
                    let n = n.to_digit(10).unwrap() as u8;
                    jellies.push(Jelly {
                        x: x as i8,
                        y: y as i8,
                        color: n,
                    });
                    walls.push(false);
                }
                _ => panic!("Invalid character"),
            }
        }
    }

    let field = Field {
        width: width.unwrap(),
        height: str.lines().count(),
        walls,
    };
    (field, jellies)
}

pub struct Field {
    pub width: usize,
    pub height: usize,
    pub walls: Vec<bool>,
}

impl Field {
    pub fn new(width: usize, height: usize) -> Field {
        Field {
            width,
            height,
            walls: vec![false; width * height],
        }
    }

    pub fn is_wall(&self, x: i8, y: i8) -> bool {
        self.walls[y as usize * self.width + x as usize]
    }

    pub fn set_wall(&mut self, x: i8, y: i8, value: bool) {
        self.walls[y as usize * self.width + x as usize] = value;
    }

    pub fn draw(&self) {
        for y in 0..self.height as i8 {
            for x in 0..self.width as i8 {
                if self.is_wall(x, y) {
                    print!("X");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    pub fn draw_with_jellies(&self, jellies: &[Jelly]) {
        for y in 0..self.height as i8 {
            for x in 0..self.width as i8 {
                if self.is_wall(x, y) {
                    print!("X");
                } else {
                    let mut found = None;
                    for jelly in jellies {
                        if jelly.x == x && jelly.y == y {
                            found = Some(jelly.color);
                            break;
                        }
                    }
                    if let Some(color) = found {
                        print!("{}", color);
                    } else {
                        print!(" ");
                    }
                }
            }
            println!();
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Jelly {
    pub x: i8,
    pub y: i8,
    pub color: u8,
}
