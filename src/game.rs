#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Field {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<u8>,
}

impl Field {
    pub fn new(width: usize, height: usize) -> Field {
        Field {
            width,
            height,
            cells: vec![0; width * height],
        }
    }

    pub fn from_str(str: &str) -> Self {
        let mut width = None;
        let mut cells = Vec::new();
        for line in str.lines() {
            if width.is_none() {
                width = Some(line.len());
            } else if width.unwrap() != line.len() {
                panic!("Inconsistent line length");
            }
            for c in line.chars() {
                cells.push(match c {
                    ' ' => 0,
                    'X' => 1,
                    c if c.is_ascii_digit() => c as u8 - b'0' + 2,
                    _ => panic!("Invalid character"),
                });
            }
        }
        Field {
            width: width.unwrap(),
            height: cells.len() / width.unwrap(),
            cells,
        }
    }

    pub fn draw(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!(
                    "{}",
                    match self.cells[y * self.width + x] {
                        0 => ' ',
                        1 => 'X',
                        c => (c - 2 + b'0') as char,
                    }
                );
            }
            println!();
        }
    }

    fn is_wall(&self, pos: usize) -> bool {
        self.cells[pos] == 1
    }

    pub fn all_colors(&self) -> Vec<u8> {
        let mut ret = Vec::new();
        for c in &self.cells {
            if 1 < *c && !ret.contains(c) {
                ret.push(*c);
            }
        }
        ret
    }

    pub fn all_blobs(&self) -> Vec<(u8, Vec<usize>)> {
        let mut ret = Vec::new();
        let mut visited = vec![false; self.cells.len()];
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = y * self.width + x;
                if visited[pos] || self.cells[pos] <= 1 {
                    continue;
                }
                let color = self.cells[pos];
                let mut blob = Vec::new();
                let mut open = vec![pos];
                while let Some(pos) = open.pop() {
                    if visited[pos] {
                        continue;
                    }
                    visited[pos] = true;
                    blob.push(pos);
                    let x = pos % self.width;
                    let y = pos / self.width;
                    if x > 0 && self.cells[pos - 1] == self.cells[pos] {
                        open.push(pos - 1);
                    }
                    if x + 1 < self.width && self.cells[pos + 1] == self.cells[pos] {
                        open.push(pos + 1);
                    }
                    if y > 0 && self.cells[pos - self.width] == self.cells[pos] {
                        open.push(pos - self.width);
                    }
                    if y + 1 < self.height && self.cells[pos + self.width] == self.cells[pos] {
                        open.push(pos + self.width);
                    }
                }
                ret.push((color, blob));
            }
        }
        ret
    }

    pub fn move_jelly(&mut self, pos: usize, to_right: bool) -> bool {
        let dx = if to_right { 1 } else { -1 };

        let all_blobs = self.all_blobs();
        let blob = all_blobs.iter().position(|b| b.1.contains(&pos)).unwrap();

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
                for j in &all_blobs[i].1 {
                    if self.is_wall(j - self.width) {
                        // Cannot jump by wall
                        for i in 0..all_blobs.len() {
                            if blob_moves[i].0 {
                                blob_moves[i].0 = false;
                            }
                        }
                        break 'outer;
                    }
                    let j = j - self.width;
                    if 1 < self.cells[j] {
                        let j = all_blobs.iter().position(|b| b.1.contains(&j)).unwrap();
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
                for j in &all_blobs[i].1 {
                    if self.is_wall((*j as i32 + self.width as i32 * dy + dx) as usize) {
                        // Cannot move by wall
                        for i in 0..all_blobs.len() {
                            if blob_moves[i].1 {
                                blob_moves[i].1 = false;
                            }
                        }
                        break 'outer;
                    }

                    let l = (*j as i32 + self.width as i32 * dy + dx) as usize;
                    if 1 < self.cells[l] {
                        let k = all_blobs.iter().position(|b| b.1.contains(&l)).unwrap();
                        if blob_moves[i].0
                            && blob_moves[k].0
                            && (self.cells[(*j as i32 + dx) as usize] <= 1
                                || !all_blobs[k].1.contains(&((*j as i32 + dx) as usize)))
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

        // Update cells
        for c in &mut self.cells {
            if 1 < *c {
                *c = 0;
            }
        }
        let all_blobs: Vec<_> = all_blobs
            .into_iter()
            .enumerate()
            .map(|(i, (color, poss))| {
                let poss: Vec<_> = poss
                    .into_iter()
                    .map(|p| {
                        let (jump, slide) = blob_moves[i];
                        let mut p = p as i32;
                        if jump {
                            p -= self.width as i32;
                        }
                        if slide {
                            p += dx;
                        }
                        self.cells[p as usize] = color;
                        p as usize
                    })
                    .collect();
                (color, poss)
            })
            .collect();

        let mut blob_moves: Vec<_> = blob_moves.iter().map(|(_, s)| *s).collect();

        // Check if the jelly can move
        'outer: while {
            let mut updated = false;

            for i in 0..all_blobs.len() {
                if !blob_moves[i] {
                    continue;
                }

                for j in &all_blobs[i].1 {
                    if self.is_wall((*j as i32 + dx) as usize) {
                        // Cannot move by wall
                        for i in 0..all_blobs.len() {
                            if blob_moves[i] {
                                blob_moves[i] = false;
                            }
                        }
                        break 'outer;
                    }

                    let l = (*j as i32 + dx) as usize;
                    if 1 < self.cells[l] {
                        let k = all_blobs.iter().position(|b| b.1.contains(&l)).unwrap();
                        if self.cells[(*j as i32 + dx) as usize] <= 1
                            || !all_blobs[k].1.contains(&((*j as i32 + dx) as usize))
                        {
                            continue;
                        }
                        if !blob_moves[k] {
                            blob_moves[k] = true;
                            updated = true;
                        }
                    }
                }
            }

            updated
        } {}

        // Update cells
        for c in &mut self.cells {
            if 1 < *c {
                *c = 0;
            }
        }
        let all_blobs: Vec<_> = all_blobs
            .into_iter()
            .enumerate()
            .map(|(i, (color, poss))| {
                let poss: Vec<_> = poss
                    .into_iter()
                    .map(|p| {
                        let slide = blob_moves[i];
                        let mut p = p as i32;
                        if slide {
                            p += dx;
                        }
                        self.cells[p as usize] = color;
                        p as usize
                    })
                    .collect();
                (color, poss)
            })
            .collect();

        self.fall(all_blobs);

        true
    }

    pub fn fall(&mut self, mut all_blobs: Vec<(u8, Vec<usize>)>) {
        let mut blobs = (0..all_blobs.len()).collect::<Vec<_>>();

        while !blobs.is_empty() {
            // Blocked by walls
            blobs.retain(|i| {
                for j in &all_blobs[*i].1 {
                    if self.is_wall(*j + self.width) {
                        return false;
                    }
                }
                true
            });

            // Blocked by other jellies
            'outer: loop {
                for (blobs_i, i) in blobs.iter().copied().enumerate() {
                    for j in &all_blobs[i].1 {
                        let j = j + self.width;
                        if 1 < self.cells[j] {
                            let j_is_not_falling =
                                blobs.iter().all(|i| !all_blobs[*i].1.contains(&j));
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
            for c in &mut self.cells {
                if 1 < *c {
                    *c = 0;
                }
            }
            all_blobs = all_blobs
                .into_iter()
                .enumerate()
                .map(|(i, (color, mut poss))| {
                    if blobs.contains(&i) {
                        for p in &mut poss {
                            *p += self.width;
                        }
                    }
                    for p in &poss {
                        self.cells[*p] = color;
                    }
                    (color, poss)
                })
                .collect();
        }
    }
}

#[test]
fn test() {
    let mut field = Field::from_str(
        r#"XXXXXXXXXXXXXXXXXXXXXXXXXXXX
XXXXXXX1100      0011XXXXXXX
XXXXXXX1100      0011XXXXXXX
XXXXXXXXXXX      XXXXXXXXXXX
XXXXXXXXXXX      XXXXXXXXXXX
XXXXXXXXXX        XXXXXXXXXX
XXXXXXXXXX        XXXXXXXXXX
XXXXXXXXXX        XXXXXXXXXX
XXXXXXXXXX        XXXXXXXXXX
XXXXXXXXXX        XXXXXXXXXX
XXXXXXXXXX        XXXXXXXXXX
X  XX  XX          XX  XX  X
X  XX  XX          XX  XX  X
X00  00              00  00X
X00  00              00  00X
XXXXXXXXXXXXXXXXXXXXXXXXXXXX"#,
    );
    field.draw();

    let blobs = field.all_blobs();
    dbg!(&blobs);

    field.move_jelly(blobs[0].1[0], true);
    field.draw();
}
