use minifb::{Window,Key};
use nalgebra_glm::{Vec2};
use crate::audio::AudioPlayer;

pub struct Player {
    pub pos: Vec2,
    pub a: f32,
    pub fov: f32,
    pub velocity: Vec2,
    pub previous_mouse_pos: Vec2, 
}

impl Player {
    pub fn new(pos: Vec2, a: f32, fov: f32) -> Self {
        Self { pos, a, fov ,  velocity: Vec2::new(0.0, 0.0), previous_mouse_pos: Vec2::new(0.0, 0.0),}
    }

    pub fn player_movement(&self, new_pos: Vec2, maze: &[Vec<char>], block_size: usize) -> bool {
        let row = (new_pos.y / block_size as f32).floor() as usize;
        let col = (new_pos.x / block_size as f32).floor() as usize;

        if row < maze.len() && col < maze[0].len() {
            let cell = maze[row][col];
            return cell == ' ';
        }

        false
    }
}

pub fn process_events(window: &Window, player: &mut Player, maze: &[Vec<char>], block_size: usize, audio_player: &AudioPlayer) {
    const MOVE_SPEED: f32 = 2.0;
    const ROTATION_SPEED: f32 = 3.14 / 80.0;

    let mut moved = false;

    if let Some((mouse_x, _mouse_y)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
        let mouse_x = mouse_x as f32;
        let delta_x = mouse_x - player.previous_mouse_pos.x;
        if delta_x.abs() > 0.1 { 
            player.a += delta_x.signum() * ROTATION_SPEED;
        }
        player.previous_mouse_pos.x = mouse_x;
    }

    let mut new_pos = player.pos;

    if window.is_key_down(Key::A) {
        player.a -= ROTATION_SPEED;
       
    }
    if window.is_key_down(Key::D) {
        player.a += ROTATION_SPEED;
     
    }
    if window.is_key_down(Key::W) {
        new_pos.x += MOVE_SPEED * player.a.cos();
        new_pos.y += MOVE_SPEED * player.a.sin();
        moved = true;
       
    }
    if window.is_key_down(Key::S) {
        new_pos.x -= MOVE_SPEED * player.a.cos();
        new_pos.y -= MOVE_SPEED * player.a.sin();
        moved = true;
      
    }
    if window.is_key_down(Key::P) {
        
    }

    if moved && player.player_movement(new_pos, maze, block_size) {
        if player.pos != new_pos { 
            player.pos = new_pos;
        }
        audio_player.play(); 
        
    }else{
        audio_player.pause();
    }
    
}