use std::collections::HashSet;

use macroquad::prelude::*;

#[derive(Default)]
struct World {
    camera: Camera2D,
    walls: HashSet<(i32, i32)>,
}

fn update(world: &mut World) {
    world.camera = Camera2D {
        zoom: 0.05 * vec2(1.0, screen_width() / screen_height()),
        ..Default::default()
    };

    if is_mouse_button_down(MouseButton::Left) {
        let (mouse_x, mouse_y) = mouse_position();
        let Vec2 { x, y } = world.camera.screen_to_world(vec2(mouse_x, mouse_y));
        world.walls.insert((x.floor() as i32, y.floor() as i32));
    }
}

fn draw(world: &World) {
    set_camera(&world.camera);

    clear_background(MAGENTA);

    for &(x, y) in &world.walls {
        draw_rectangle(x as f32, y as f32, 1., 1., GRAY);
    }

    draw_circle(0., 0., 1., SKYBLUE);
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
