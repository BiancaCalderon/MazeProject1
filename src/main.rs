use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{Vec2};
use std::f32::consts::PI;
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::time::{Instant, Duration};
use rusttype::Scale;


mod framebuffer;
use framebuffer::Framebuffer;
mod maze;
use maze::load_maze;

mod player;
use player::{Player, process_events};

mod caster;
use caster::{Intersect, cast_ray};

mod texture;
use texture::Texture;

//static WALL1: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/WALL2.jpg")));

fn cell_to_texture_color(cell: char, tx: u32, ty: u32) -> u32 {
    let wall_color = 0x30822e;
    let default_color = 0x000000;

    match cell {
        '+' => wall_color,
        '-' => wall_color,
        '|' => wall_color,
        'g' => wall_color,
        _ => default_color,
    }
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size:usize, cell: char) {

    for x in xo..xo + block_size{
        for y in yo..yo + block_size{
            if cell != ' '{
                framebuffer.set_current_color(0x000000);
                framebuffer.point(x,y);
            }
        }
    }
}

fn render3d(framebuffer: &mut Framebuffer, player: &Player){
    let maze = load_maze("./maze.txt");
    let num_rays = framebuffer.width; 
    let block_size = 100;
    
    for i in 0..framebuffer.width{
        framebuffer.set_current_color(0x383838);
        for j in 0..(framebuffer.height / 2){
            framebuffer.point(i, j);
        }
        framebuffer.set_current_color(0x717171);
        for j in (framebuffer.height / 2)..framebuffer.height {
            framebuffer.point(i, j);
        }
    }

    let hh = framebuffer.height as f32 / 2.0;
    for i in 0..num_rays {
        let current_ray = (i as f32 / num_rays as f32);
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let Intersect = cast_ray(framebuffer, &maze, player, a, block_size, false);

        let distance = Intersect.distance * (a - player.a).cos();

        let stake_height = (framebuffer.height as f32 / distance) * 70.0;

        let stake_top = (hh - (stake_height / 2.0)) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)) as usize;

        for y in stake_top..stake_bottom{
            let ty = (y as f32 - stake_top as f32) / (stake_bottom as f32 - stake_top as f32) *  128.0;
            let tx = Intersect.tx;
            let color = cell_to_texture_color(Intersect.impact, tx as u32, ty as u32);
            framebuffer.set_current_color(color);
            framebuffer.point(i, y);
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

    let num_rays = 100; 
    for i in 0..num_rays {
        let current_ray = (i as f32 / num_rays as f32);
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray(framebuffer, &maze, player, a, block_size, true);
    }
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

    // Mueve la ventana
    window.set_position(100, 100);
    window.update();

    // Inicializa valores
    framebuffer.set_background_color(0x333355);
    let mut player = Player {
        pos: Vec2::new(150.0, 150.0),
        a: PI / 3.0,
        fov: PI / 3.0,
    };
    let mut mode = "3D";

    // Cargar el laberinto y definir block_size
    let maze = load_maze("./maze.txt");
    let block_size = 100;

    let mut last_time = Instant::now();
    let mut frame_count = 0;
    let mut fps_text = String::new();

    while window.is_open() {
        // Escucha de inputs
        if window.is_key_down(Key::Escape) {
            break;
        }
        if window.is_key_down(Key::M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }

        // Procesar eventos
        process_events(&window, &mut player, &maze, block_size);

        framebuffer.clear();

        if mode == "2D" {
            render2d(&mut framebuffer, &player);
        } else {
            render3d(&mut framebuffer, &player);
        }

        // Calcular FPS
        frame_count += 1;
        let current_time = Instant::now();
        let elapsed = current_time.duration_since(last_time);

        if elapsed >= Duration::from_secs(1) {
            let fps = (frame_count as f64 / elapsed.as_secs_f64()).round() as u64;
            fps_text = format!("FPS: {}", fps);
            last_time = current_time;
            frame_count = 0;
        }

        // Dibujar el texto de FPS en cada frame
        framebuffer.draw_text(&fps_text, 10, 10, Scale::uniform(32.0), 0xFFFFFF);

        // Actualiza la ventana con el contenido del framebuffer
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(Duration::from_millis(16));
    }
}
