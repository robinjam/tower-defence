use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use macroquad::prelude::*;

#[derive(Default)]
struct World {
    camera: Camera2D,
    walls: HashSet<IVec2>,
}

impl World {
    /// returns a field containing the shortest distance from each tile to (0,0)
    fn integration_field(&self) -> HashMap<IVec2, usize> {
        let mut unvisited = BinaryHeap::new();
        unvisited.push(State {
            cost: 0,
            position: IVec2::ZERO,
        });

        let mut distance: HashMap<IVec2, usize> = HashMap::new();
        distance.insert(IVec2::ZERO, 0);

        while let Some(State { cost, position }) = unvisited.pop() {
            if distance.get(&position).is_some_and(|&c| c < cost) {
                continue;
            }

            for neighbour in [IVec2::Y, IVec2::X, IVec2::NEG_Y, IVec2::NEG_X]
                .iter()
                .map(|&dir| position + dir)
                .filter(|&pos| pos == pos.clamp(ivec2(-50, -50), ivec2(50, 50)))
                .filter(|pos| !self.walls.contains(pos))
            {
                let next = State {
                    cost: cost + 1,
                    position: neighbour,
                };

                if distance.get(&next.position).is_none_or(|&c| next.cost < c) {
                    unvisited.push(next);
                    // Relaxation, we have now found a better way 🧘‍♂️
                    distance.insert(next.position, next.cost);
                }
            }
        }

        distance
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: IVec2,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // TODO: In case of a tie we probably should compare positions
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn update(world: &mut World) {
    world.camera = Camera2D {
        zoom: 0.05 * vec2(1.0, screen_width() / screen_height()),
        ..Default::default()
    };

    if is_mouse_button_down(MouseButton::Left) {
        let (mouse_x, mouse_y) = mouse_position();
        let Vec2 { x, y } = world.camera.screen_to_world(vec2(mouse_x, mouse_y));
        world
            .walls
            .insert(ivec2(x.floor() as i32, y.floor() as i32));
    }
}

fn draw(world: &World) {
    set_camera(&world.camera);

    clear_background(WHITE);

    for &wall in &world.walls {
        draw_rectangle(wall.x as f32, wall.y as f32, 1., 1., GRAY);
    }

    for (&IVec2 { x, y }, d) in &world.integration_field() {
        draw_rectangle(
            x as f32,
            y as f32,
            1.,
            1.,
            Color::from_rgba(0, 0, 0, (*d * 4) as u8),
        );
    }

    draw_circle(0., 0., 1., BLACK);
}

#[macroquad::main("Tower Defence 2: Electric Boogaloo")]
async fn main() {
    let mut world = World::default();

    loop {
        update(&mut world);
        draw(&world);
        next_frame().await;
    }
}
