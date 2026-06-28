use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::Bounds;

const TERRAIN_TILESET_PATH: &str = "tileset_terrain.png";
const DIRT_FILL: [usize; 4] = [9, 17, 18, 25];

// ---- layer 2a: the tile-role taxonomy for one terrain ----
enum Dirt {
    Plain,
    Dot,
    Horizontal(HSeg), // a 1-tall strip
    Vertical(VSeg),   // a 1-wide strip
    Large(Cell3),     // a filled area: 3x3 of corners/edges/center
}
enum HSeg {
    Left,
    Middle,
    Right,
}
enum VSeg {
    Top,
    Middle,
    Bottom,
}
enum Cell3 {
    TopLeft,
    TopMiddle,
    TopRight,
    MiddleLeft,
    Center,
    MiddleRight,
    BottomLeft,
    BottomMiddle,
    BottomRight,
}

// ---- layer 2b: role -> atlas index (your IndexedTileSet, kept) ----
trait IndexedTileSet {
    fn to_tile_index(&self) -> usize;
}

impl IndexedTileSet for Dirt {
    fn to_tile_index(&self) -> usize {
        // NOTE: these indices are still the provisional ones from last turn —
        // they need the verification pass to lock the real corners.
        match self {
            Dirt::Plain => 48,
            Dirt::Dot => 3,
            Dirt::Horizontal(h) => match h {
                HSeg::Left => 8,
                HSeg::Middle => 9,
                HSeg::Right => 10,
            },
            Dirt::Vertical(v) => match v {
                VSeg::Top => 1,
                VSeg::Middle => 9,
                VSeg::Bottom => 17,
            },
            Dirt::Large(c) => match c {
                Cell3::TopLeft => 0,
                Cell3::TopMiddle => 1,
                Cell3::TopRight => 2,
                Cell3::MiddleLeft => 8,
                Cell3::Center => 9,
                Cell3::MiddleRight => 10,
                Cell3::BottomLeft => 16,
                Cell3::BottomMiddle => 17,
                Cell3::BottomRight => 18,
            },
        }
    }
}

// ---- layer 2c: neighbours -> role, then fill in place ----
trait AutoTile: IndexedTileSet + Sized {
    fn role(n: bool, s: bool, w: bool, e: bool) -> Self;
}

impl AutoTile for Dirt {
    fn role(n: bool, s: bool, w: bool, e: bool) -> Self {
        match (n, s, w, e) {
            (true, true, true, true) => Dirt::Large(Cell3::Center),
            (false, true, true, true) => Dirt::Large(Cell3::TopMiddle),
            (true, false, true, true) => Dirt::Large(Cell3::BottomMiddle),
            (true, true, false, true) => Dirt::Large(Cell3::MiddleLeft),
            (true, true, true, false) => Dirt::Large(Cell3::MiddleRight),
            (false, true, false, true) => Dirt::Large(Cell3::TopLeft),
            (false, true, true, false) => Dirt::Large(Cell3::TopRight),
            (true, false, false, true) => Dirt::Large(Cell3::BottomLeft),
            (true, false, true, false) => Dirt::Large(Cell3::BottomRight),
            (false, false, true, true) => Dirt::Horizontal(HSeg::Middle),
            (false, false, false, true) => Dirt::Horizontal(HSeg::Left),
            (false, false, true, false) => Dirt::Horizontal(HSeg::Right),
            (true, true, false, false) => Dirt::Vertical(VSeg::Middle),
            (false, true, false, false) => Dirt::Vertical(VSeg::Top),
            (true, false, false, false) => Dirt::Vertical(VSeg::Bottom),
            (false, false, false, false) => Dirt::Dot,
        }
    }
}

fn fill_variant(p: IVec2) -> usize {
    let mut h = (p.x as u32).wrapping_mul(0x9E3779B1) ^ (p.y as u32).wrapping_mul(0x85EBCA77);
    h ^= h >> 16;
    h = h.wrapping_mul(0x7FEB352D);
    h ^= h >> 15;
    DIRT_FILL[(h as usize) % DIRT_FILL.len()]
}

// generic over the terrain: edits `grid` in place for any region shape
fn autotile<T: AutoTile>(region: &HashSet<IVec2>, grid: &mut HashMap<IVec2, usize>) {
    for &c in region {
        let here = |d: IVec2| region.contains(&(c + d));
        let role = T::role(
            here(IVec2::new(0, -1)),
            here(IVec2::new(0, 1)),
            here(IVec2::new(-1, 0)),
            here(IVec2::new(1, 0)),
        );
        grid.insert(c, fill_variant(c));
    }
}

pub fn setup(
    bounds: &Bounds,
    commands: &mut Commands,
    assets: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let terrain_img = assets.load(TERRAIN_TILESET_PATH);
    let terrain_layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        10,
        9,
        None,
        None,
    ));

    let bounds_set = (bounds.min_x..=bounds.max_x)
        .map(|x| (bounds.min_y..=bounds.max_y).map(move |y| IVec2::new(x, y)))
        .flatten()
        .collect();
    let mut grid = HashMap::new();
    autotile::<Dirt>(&bounds_set, &mut grid);

    for (position, tile_index) in grid {
        // sprite
        let world = crate::tile_to_world(position);
        commands.spawn((
            Sprite::from_atlas_image(
                terrain_img.clone(),
                TextureAtlas {
                    layout: terrain_layout.clone(),
                    index: tile_index,
                },
            ),
            Transform::from_translation(world.extend(3.0)),
        ));
    }
}
