use crate::Player;
use specs::prelude::*;

use super::{Map, Position, Viewshed};
use rltk::{field_of_view, Point};
use specs::{Join, System, WriteExpect, WriteStorage};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player) = data;

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_titles.clear();
                viewshed.visible_titles =
                    field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
                viewshed
                    .visible_titles
                    .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

                let _p = player.get(ent);
                if let Some(_p) = _p {
                    for t in map.visible_titles.iter_mut() {
                        *t = false;
                    }
                    for vis in viewshed.visible_titles.iter() {
                        let idx = map.xy_idx(vis.x, vis.y);
                        map.revealed_titles[idx] = true;
                        map.visible_titles[idx] = true;
                    }
                }
            }
        }
    }
}
