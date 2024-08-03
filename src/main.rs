use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{Vec2};
use std::time::Duration;
use std::f32::consts::PI;

mod framebuffer;
use framebuffer::Framebuffer;
mod maze;
use maze::load_maze;

mod player;
use player::Player;

mod caster;
use caster::{Intersect, cast_ray};

fn cell_to_color(cell: char) -> u32 {
    let default_color = 0x000000;
    match cell {
        '+' => 0xFF00FF,
        '-' => 0xDD11DD,
        '|' => 0xCC11CC,
        'g' => 0xFF0000,
        _ => default_color,
    }
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size:usize, cell: char) {

    for x in xo..xo + block_size{
        for y in yo..yo + block_size{
            if cell != ' '{
                let color = cell_to_color(cell);
                framebuffer.set_current_color(color);
                framebuffer.point(x,y);
            }
        }
    }


}


fn render2d(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./maze.txt");
    let block_size = 100;

    for row in 0..maze.len(){
        for col in 0..maze[row].len(){
            draw_cell(framebuffer, col * block_size, row * block_size,block_size, maze[row][col]);

        }
    }
    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.point(player.pos.x as usize, player.pos.y as usize);

    cast_ray(framebuffer, &maze, player, player.a, block_size, true);
}


fn main() {
    let window_width = 1300;
    let window_height = 900;

    let framebuffer_width = 1300;
    let framebuffer_height = 900;

    let frame_delay = Duration::from_millis(0);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Rust Graphics - Maze Example",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    // move the window around
    window.set_position(100, 100);
    window.update();
    
    // initialize values
    framebuffer.set_background_color(0x333355);
    let player = Player{
        pos: Vec2::new(150.0, 150.0),
        a: PI/3.0,
    };
    framebuffer.clear();

    while window.is_open() {
        // listen to inputs
        if window.is_key_down(Key::Escape) {
            break;
        }

        render2d(&mut framebuffer, &player);
        // Update the window with the framebuffer contents
window
.update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
.unwrap();

std::thread::sleep(frame_delay);

    }
}