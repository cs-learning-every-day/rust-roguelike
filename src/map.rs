use std::cmp::{max, min};

use rltk::{RandomNumberGenerator, Rltk, RGB};

use super::Rect;

#[derive(PartialEq, Clone, Copy)]
pub enum TitleType {
    Wall,
    Floor,
}

pub struct Map {
    pub titles: Vec<TitleType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn apply_from_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.titles[idx] = TitleType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, 2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.titles[idx] = TitleType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.titles[idx] = TitleType::Floor;
            }
        }
    }

    /// Makes a map with solid boundaries and 400 randomly placed walls. No guarantees that it won't
    /// look awful.
    pub fn new_map_test() -> Map {
        let mut map = Map {
            titles: vec![TitleType::Floor; 80 * 50],
            rooms: Vec::new(),
            width: 80,
            height: 50,
        };

        for x in 0..80 {
            let mut idx = map.xy_idx(x, 0);
            map.titles[idx] = TitleType::Wall;
            idx = map.xy_idx(x, 49);
            map.titles[idx] = TitleType::Wall;
        }

        for y in 0..50 {
            let mut idx = map.xy_idx(0, y);
            map.titles[idx] = TitleType::Wall;
            idx = map.xy_idx(79, y);
            map.titles[idx] = TitleType::Wall;
        }

        let mut rng = rltk::RandomNumberGenerator::new();
        for _i in 0..400 {
            let x = rng.roll_dice(1, 79);
            let y = rng.roll_dice(1, 49);
            let idx = map.xy_idx(x, y);
            if idx != map.xy_idx(40, 25) {
                map.titles[idx] = TitleType::Wall;
            }
        }

        map
    }

    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map = Map {
            rooms: Vec::new(),
            titles: vec![TitleType::Wall; 80 * 50],
            width: 80,
            height: 50,
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, 80 - w - 1) - 1;
            let y = rng.roll_dice(1, 50 - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false;
                }
            }
            if ok {
                map.apply_from_to_map(&new_room);
                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }
                map.rooms.push(new_room);
            }
        }

        map
    }
}

pub fn draw_map(map: &Map, ctx: &mut Rltk) {
    let mut y = 0;
    let mut x = 0;
    for title in map.titles.iter() {
        match title {
            TitleType::Wall => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('#'),
                );
            }
            TitleType::Floor => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('.'),
                );
            }
        }
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
