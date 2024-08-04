use minifb::{Window, Key};
use nalgebra_glm::{Vec2};

pub struct Player{
    pub pos: Vec2,
    pub a: f32,
    pub fov: f32,
}

pub fn process_events(window: &Window, player: &mut Player, maze: &Vec<Vec<char>>, block_size: usize) {
    const MOVE_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = 3.14 / 50.0;

    let mut new_pos = player.pos;

    if window.is_key_down(Key::Left) {
        player.a -= ROTATION_SPEED;
    }
    if window.is_key_down(Key::Right) {
        player.a += ROTATION_SPEED;
    }
    if window.is_key_down(Key::Up) {
        new_pos.x = player.pos.x + MOVE_SPEED * player.a.cos();
        new_pos.y = player.pos.y + MOVE_SPEED * player.a.sin();
    }
    if window.is_key_down(Key::Down) {
        new_pos.x = player.pos.x - MOVE_SPEED * player.a.cos();
        new_pos.y = player.pos.y - MOVE_SPEED * player.a.sin();
    }

    // Verificar colisiones
    let i = new_pos.x as usize / block_size;
    let j = new_pos.y as usize / block_size;

    if maze[j][i] == ' ' {
        player.pos = new_pos;
    }
}
