use std::collections::HashMap;

use bevy::prelude::*;

const TILE_SIZE: f32 = 16.0;
const CART_TILESET_PATH: &str = "tileset_cart.png";
const RAIL_TILESET_PATH: &str = "tileset_rails.png";
const SHEET_COLS: u32 = 1; // TODO from image
const SHEET_ROWS: u32 = 1; // TODO from image

const IDX_HORIZONTAL: usize = 0; // TODO
const IDX_VERTICAL: usize = 0; // TODO
const IDX_TLCORNER: usize = 0; // '/'  TODO
const IDX_TRCORNER: usize = 0; // '\'  TODO
const IDX_BLCORNER: usize = 0;
const IDX_BRCORNER: usize = 0;
const IDX_CROSSROAD: usize = 0; // '+'  TODO

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_offset(&self) -> IVec2 {
        match self {
            Direction::Up => IVec2::new(0, -1),
            Direction::Down => IVec2::new(0, 1),
            Direction::Left => IVec2::new(-1, 0),
            Direction::Right => IVec2::new(1, 0),
        }
    }

    fn left(&self) -> Self {
        match self {
            Self::Left => Self::Down,
            Self::Right => Self::Up,
            Self::Up => Self::Left,
            Self::Down => Self::Right,
        }
    }

    fn right(&self) -> Self {
        match self {
            Self::Left => Self::Up,
            Self::Right => Self::Down,
            Self::Up => Self::Right,
            Self::Down => Self::Left,
        }
    }
}

#[derive(Component)]
struct Cart {
    position: IVec2,
    direction: Direction,
    progress: f32,
    speed: f32,
    crashed: bool,
    last_choice: Direction,
}

impl Cart {
    fn from(position: IVec2, c: char) -> Option<Self> {
        let direction;
        match c {
            '^' => direction = Direction::Up,
            '>' => direction = Direction::Right,
            'v' => direction = Direction::Down,
            '<' => direction = Direction::Left,
            _ => return None,
        }
        Some(Self {
            position,
            direction,
            progress: 0.0,
            speed: 1.0,
            crashed: false,
            last_choice: Direction::Right,
        })
    }

    fn choose_direction(&mut self, rail_flavor: &RailFlavor) -> Direction {
        match (self.direction, rail_flavor) {
            (_, RailFlavor::Horizontal) => self.direction,
            (_, RailFlavor::Vertical) => self.direction,
            (_, RailFlavor::Crossroad) => {
                let direction = match self.last_choice {
                    Direction::Right => self.direction.left(),
                    Direction::Left => self.direction,
                    _ => self.direction.right(),
                };
                self.last_choice = direction;
                return direction;
            }
            (Direction::Up, RailFlavor::TLCorner) => Direction::Right,
            (Direction::Left, RailFlavor::TLCorner) => Direction::Down,
            (Direction::Up, RailFlavor::TRCorner) => Direction::Left,
            (Direction::Right, RailFlavor::TRCorner) => Direction::Down,
            (Direction::Down, RailFlavor::BLCorner) => Direction::Right,
            (Direction::Left, RailFlavor::BLCorner) => Direction::Up,
            (Direction::Down, RailFlavor::BRCorner) => Direction::Left,
            (Direction::Right, RailFlavor::BRCorner) => Direction::Up,
            (_, _) => {
                println!(
                    "Cart moving {:?} tried to enter rail {:?}",
                    self.direction, rail_flavor
                );
                panic!();
            }
        }
    }

    fn to_tile_index(&self) -> usize {
        match self.direction {
            Direction::Down => 3,
            Direction::Up => 3,
            Direction::Left => 0,
            Direction::Right => 0,
        }
    }
}

#[derive(Component)]
struct Rail {
    position: IVec2,
    flavor: RailFlavor,
}

impl Rail {
    fn from(position: IVec2, left_neighbor: char, c: char) -> Option<Self> {
        let flavor;
        match (left_neighbor, c) {
            (_, '|') => flavor = RailFlavor::Vertical,
            (_, '-') => flavor = RailFlavor::Horizontal,
            (_, '+') => flavor = RailFlavor::Crossroad,
            ('-', '/') => flavor = RailFlavor::BRCorner,
            ('+', '/') => flavor = RailFlavor::BRCorner,
            ('-', '\\') => flavor = RailFlavor::TRCorner,
            ('+', '\\') => flavor = RailFlavor::TRCorner,
            (_, '\\') => flavor = RailFlavor::BLCorner,
            (_, '/') => flavor = RailFlavor::TLCorner,
            (_, '>') => flavor = RailFlavor::Horizontal,
            (_, '<') => flavor = RailFlavor::Horizontal,
            (_, '^') => flavor = RailFlavor::Vertical,
            (_, 'v') => flavor = RailFlavor::Vertical,
            (_, _) => return None,
        }
        Some(Self { position, flavor })
    }
}

#[derive(Clone, Copy, Debug)]
enum RailFlavor {
    Vertical,
    Horizontal,
    Crossroad,
    TLCorner,
    BLCorner,
    TRCorner,
    BRCorner,
}

impl RailFlavor {
    fn to_tile_index(&self) -> usize {
        match self {
            Self::Vertical => 2,
            Self::Horizontal => 52,
            Self::Crossroad => 80,
            Self::TLCorner => 8,
            Self::TRCorner => 9,
            Self::BLCorner => 18,
            Self::BRCorner => 19,
        }
    }
}

#[derive(Resource)]
struct RailGrid {
    rails: HashMap<IVec2, RailFlavor>,
    bounds: Bounds,
}

struct RailMap {
    bounds: Bounds,
    carts: Vec<Cart>,
    rails: Vec<Rail>,
}

struct Bounds {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

fn get_stats(rails: &Vec<Rail>, carts: &Vec<Cart>) -> Bounds {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for r in rails.iter() {
        if r.position.x < min_x {
            min_x = r.position.x;
        }
        if r.position.x > max_x {
            max_x = r.position.x;
        }
        if r.position.y < min_y {
            min_y = r.position.y;
        }
        if r.position.y > max_y {
            max_y = r.position.y;
        }
    }
    for c in carts.iter() {
        if c.position.x < min_x {
            min_x = c.position.x;
        }
        if c.position.x > max_x {
            max_x = c.position.x;
        }
        if c.position.y < min_y {
            min_y = c.position.y;
        }
        if c.position.y > max_y {
            max_y = c.position.y;
        }
    }
    Bounds {
        min_x,
        min_y,
        max_x,
        max_y,
    }
}

fn read_input(path: &str) -> RailMap {
    let mut carts = Vec::new();
    let mut rails = Vec::new();

    let input = std::fs::read_to_string(path).expect("Bad path for read_input");
    let lines = input.lines();
    for (y, line) in lines.enumerate() {
        let mut left_char = ' ';
        for (x, c) in line.char_indices() {
            if let Some(cart) = Cart::from(
                IVec2::new(
                    x.try_into().expect("X out of bounds for cart"),
                    y.try_into().expect("Y out of bounds for cart"),
                ),
                c,
            ) {
                carts.push(cart);
            }
            if let Some(rail) = Rail::from(
                IVec2::new(
                    x.try_into().expect("X out of bounds for rail"),
                    y.try_into().expect("Y out of bounds for rail"),
                ),
                left_char,
                c,
            ) {
                rails.push(rail);
            }
            left_char = c;
        }
    }

    let bounds = get_stats(&rails, &carts);

    RailMap {
        carts,
        rails,
        bounds,
    }
}

fn tile_to_world(position: IVec2) -> Vec2 {
    Vec2::new(
        position.x as f32 * TILE_SIZE,
        -(position.y as f32) * TILE_SIZE,
    )
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let rail_img = assets.load(RAIL_TILESET_PATH);
    let rail_layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        10,
        9,
        None,
        None,
    ));
    let cart_img = assets.load(CART_TILESET_PATH);
    let cart_layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        3,
        2,
        None,
        None,
    ));

    let map = read_input("input.txt");
    let center_tile = IVec2::new(
        (map.bounds.min_x + map.bounds.max_x) / 2,
        (map.bounds.min_y + map.bounds.max_y) / 2,
    );
    let center = tile_to_world(center_tile);
    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(center.x, center.y, 999.0),
    ));

    // rails -> resource lookup table
    let mut rails = HashMap::new();
    for r in map.rails {
        rails.insert(r.position, r.flavor);
        // sprite
        let world = tile_to_world(r.position);
        commands.spawn((
            Sprite::from_atlas_image(
                rail_img.clone(),
                TextureAtlas {
                    layout: rail_layout.clone(),
                    index: r.flavor.to_tile_index(),
                },
            ),
            Transform::from_translation(world.extend(0.0)),
        ));
    }
    commands.insert_resource(RailGrid {
        rails,
        bounds: map.bounds,
    });

    // carts -> entities (with something visible attached)
    for cart in map.carts {
        let world = tile_to_world(cart.position);
        commands.spawn((
            Sprite::from_atlas_image(
                cart_img.clone(),
                TextureAtlas {
                    layout: cart_layout.clone(),
                    index: cart.to_tile_index(),
                },
            ),
            Transform::from_translation(world.extend(1.0)),
            cart,
        ));
    }
}

fn carts_move(
    time: Res<Time>,
    grid: Res<RailGrid>,
    mut carts: Query<(&mut Cart, &mut Transform, &mut Sprite)>,
) {
    for (mut cart, mut transform, mut sprite) in &mut carts {
        cart.progress += cart.speed * time.delta_secs();
        if cart.progress >= 1.0 {
            cart.progress -= 1.0;
            let offset = cart.direction.to_offset();
            cart.position += offset;
            match grid.rails.get(&cart.position).copied() {
                Some(flavor) => {
                    let d = cart.choose_direction(&flavor);
                    cart.direction = d;
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.index = cart.to_tile_index();
                    }
                }
                None => { /* ran off the end of the track — crash or stop */ }
            }
        }
        let a = tile_to_world(cart.position);
        let b = tile_to_world(cart.position + cart.direction.to_offset());
        transform.translation = a.lerp(b, cart.progress).extend(1.0);
    }
}

fn carts_crash(mut carts: Query<(&mut Cart, &Transform)>) {
    // for (mut cart, mut transform) in &mut carts {}
}

fn main() {
    // let input = read_input("input.txt");
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, (carts_move, carts_crash))
        .run();
}
