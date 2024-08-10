use minifb::{Key, Window, WindowOptions, MouseMode};
use nalgebra_glm::{Vec2, distance};
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

mod audio;
use audio::AudioPlayer;

static WALL2: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/WALL2.jpg")));

static ENEMY: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/sprite.png")));

static SKY: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/sky1.png")));

static GRASS: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/grass.png")));


fn cell_to_texture_color(cell: char, tx: u32, ty: u32) -> u32 {
    //let wall_color = 0x30822e; // Color verde oscuro para las paredes
    let default_color = 0x000000;

    match cell {
        '+' | '-' | '|' | 'g' => WALL2.get_pixel_color(tx, ty),
        _ => default_color,
    }
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            let color = match cell {
                'g' => 0xFF0000, // Rojo para la salida
                _ => 0x000000,   // Negro para otras celdas
            };
            framebuffer.set_current_color(color);
            framebuffer.point(x, y);
        }
    }
}


fn render3d(framebuffer: &mut Framebuffer, player: &Player, z_buffer: &mut [f32]) {
    let maze = load_maze("./maze.txt");
    let num_rays = framebuffer.width;
    let block_size = 100;

    // Dibujar el fondo con texturas
    let hh = framebuffer.height as f32 / 2.0;

    // Dibujar la textura del cielo en la mitad superior
    for i in 0..num_rays {
        for j in 0..(framebuffer.height / 2) {
            let tx = (i as f32 / num_rays as f32 * SKY.width as f32) as u32;
            let ty = (j as f32 / (framebuffer.height / 2) as f32 * SKY.height as f32) as u32;
            let color = SKY.get_pixel_color(tx, ty);
            framebuffer.set_current_color(color);
            framebuffer.point(i, j);
        }
    }

    // Dibujar la textura del suelo en la mitad inferior
    for i in 0..num_rays {
        for j in (framebuffer.height / 2)..framebuffer.height {
            let tx = (i as f32 / num_rays as f32 * GRASS.width as f32) as u32;
            let ty = ((j - framebuffer.height / 2) as f32 / (framebuffer.height / 2) as f32 * GRASS.height as f32) as u32;
            let color = GRASS.get_pixel_color(tx, ty);
            framebuffer.set_current_color(color);
            framebuffer.point(i, j);
        }
    }

    for i in 0..num_rays {
        let current_ray = (i as f32 / num_rays as f32);
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let Intersect = cast_ray(framebuffer, &maze, player, a, block_size, false);

        let distance = Intersect.distance * (a - player.a).cos();
        let stake_height = (framebuffer.height as f32 / distance) * 70.0;
        let stake_top = (hh - (stake_height / 2.0)) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)) as usize;

        z_buffer[i] = distance;
    
        for y in stake_top..stake_bottom {
            let ty = (y as f32 - stake_top as f32) / (stake_bottom as f32 - stake_top as f32) * 128.0;
            let tx = Intersect.tx;
            let color = if Intersect.impact == 'g' {
                0x4c9141 // Verde para la salida
            } else {
                cell_to_texture_color(Intersect.impact, tx as u32, ty as u32)
            };
            framebuffer.set_current_color(color);
            framebuffer.point(i, y);
        }
    }
}


fn render2d(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./maze.txt");
    let block_size = 100;

    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col]);
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

fn render_minimap(framebuffer: &mut Framebuffer, player: &Player) {
    let minimap_size = 200; // Tamaño del minimapa
    let minimap_x = framebuffer.width - minimap_size - 100; // Posición X del minimapa
    let minimap_y = framebuffer.height - minimap_size - 10; // Posición Y del minimapa

    // Asegúrate de que el minimapa esté dentro de los límites del framebuffer
    if minimap_x < 0 || minimap_y < 0 {
        return; // No dibujar si el minimapa está fuera del framebuffer
    }

    // Dibujar el fondo del minimapa
    framebuffer.set_current_color(0x222222); // Color oscuro para el fondo del minimapa
    for x in minimap_x..minimap_x + minimap_size {
        for y in minimap_y..minimap_y + minimap_size {
            if x < framebuffer.width && y < framebuffer.height {
                framebuffer.point(x, y);
            }
        }
    }

    // Cargar el laberinto
    let maze = load_maze("./maze.txt");
    let block_size = 100; // Tamaño del bloque del mapa
    let scale = minimap_size as f32 / (maze.len() as f32 * block_size as f32);

    // Dibujar el laberinto en el minimapa
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            let cell_x = (col as f32 * block_size as f32 * scale) as usize;
            let cell_y = (row as f32 * block_size as f32 * scale) as usize;
            let mini_block_size = (block_size as f32 * scale) as usize;

            // Asegúrate de que las celdas del laberinto no se dibujen fuera de los límites del minimapa
            for dx in 0..mini_block_size {
                for dy in 0..mini_block_size {
                    let x = minimap_x + cell_x + dx;
                    let y = minimap_y + cell_y + dy;
                    if x < framebuffer.width && y < framebuffer.height {
                        let color = if maze[row][col] == 'g' {
                            0xFF0000 // Rojo para la salida
                        } else {
                            cell_to_texture_color(maze[row][col], 0, 0)
                        };
                        framebuffer.set_current_color(color);
                        framebuffer.point(x, y);
                    }
                }
            }
        }
    }

    // Dibujar la posición del jugador en el minimapa
    framebuffer.set_current_color(0xFF0000); // Color rojo para el jugador
    let player_x = (player.pos.x as f32 * scale) as usize;
    let player_y = (player.pos.y as f32 * scale) as usize;

    // Asegúrate de que la posición del jugador esté dentro del minimapa
    if minimap_x + player_x < framebuffer.width && minimap_y + player_y < framebuffer.height {
        framebuffer.point(minimap_x + player_x, minimap_y + player_y);
    }
}

fn render_enemy(framebuffer: &mut Framebuffer, player: &Player, pos: &Vec2, z_buffer: &mut [f32]) {
    // player_a
    let sprite_a = (pos.y - player.pos.y).atan2(pos.x - player.pos.x);
    // let sprite_a = - player.a;
    //
    if sprite_a < 0.0 {
      return;
    }
  
    let sprite_d = ((player.pos.x - pos.x).powi(2) + (player.pos.y - pos.y).powi(2)).sqrt();
    // let sprite_d = distance(player.pos, pos);
  
    if sprite_d < 10.0 {
      return;
    }
  
    let screen_height = framebuffer.height as f32;
    let screen_width = framebuffer.width as f32;
  
    let sprite_size = (screen_height / sprite_d) * 100.0;
    let start_x = (sprite_a - player.a) * (screen_height / player.fov) + (screen_width / 2.0) - (sprite_size / 2.0);
    let start_y = (screen_height / 2.0) - (sprite_size / 2.0);
  
    let end_x = ((start_x + sprite_size) as usize).min(framebuffer.width);
    let end_y = ((start_y + sprite_size) as usize).min(framebuffer.height);
    let start_x = start_x.max(0.0) as usize;
    let start_y = start_y.max(0.0) as usize;
  
    if end_x <= 0 {
      return;
    }
  
    if start_x < framebuffer.width && sprite_d < z_buffer[start_x] {
      for x in start_x..(end_x - 1) {
        for y in start_y..(end_y - 1) {
          let tx = ((x - start_x) * 128 / sprite_size as usize) as u32;
          let ty = ((y - start_y) * 128 / sprite_size as usize) as u32;
          let color = ENEMY.get_pixel_color(tx, ty);
          if color != 0x3a4041 { 
            framebuffer.set_current_color(color);
            framebuffer.point(x, y);
          }
          z_buffer[x] = sprite_d;
        }
      }
    }
  }
  
  fn render_enemies(framebuffer: &mut Framebuffer, player: &Player, z_buffer: &mut [f32]) {
    let enemies = vec![
      Vec2::new(250.0, 250.0),
      Vec2::new(250.0, 550.0),
    ];
  
    for enemy in &enemies {
      render_enemy(framebuffer, &player, enemy, z_buffer);
    }
  }

  fn has_won(player: &Player, goal_position: &Vec2, block_size: usize) -> bool {
    let player_block_x = (player.pos.x / block_size as f32).round() as usize;
    let player_block_y = (player.pos.y / block_size as f32).round() as usize;
    let goal_block_x = (goal_position.x / block_size as f32).round() as usize;
    let goal_block_y = (goal_position.y / block_size as f32).round() as usize;

    player_block_x == goal_block_x && player_block_y == goal_block_y
}


fn draw_victory_screen(framebuffer: &mut Framebuffer) {
    framebuffer.set_background_color(0x000000); // Fondo negro
    framebuffer.set_current_color(0x00FF00);    // Texto verde
    framebuffer.draw_text("¡Felicidades! Has completado el nivel.", 100, framebuffer.height / 2, Scale::uniform(48.0), 0x00FF00);
    framebuffer.draw_text("Presiona Esc para salir.", 100, framebuffer.height / 2 + 60, Scale::uniform(32.0), 0x00FF00);
}

fn get_goal_position(maze: &[Vec<char>], block_size: usize) -> Vec2 {
    let mut goal_position = Vec2::new(0.0, 0.0);
    for (row_idx, row) in maze.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == 'g' {
                goal_position = Vec2::new(col_idx as f32 * block_size as f32, row_idx as f32 * block_size as f32);
                return goal_position;
            }
        }
    }
    goal_position
}


fn main() {
    let window_width = 1400;
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

    let goal_position = get_goal_position(&maze, block_size);

    let mut last_time = Instant::now();
    let mut frame_count = 0;
    let mut fps_text = String::new();
    let mut last_mouse_x = window.get_mouse_pos(MouseMode::Clamp).unwrap_or((0.0, 0.0)).0;

    let audio_player = AudioPlayer::new("assets/audio1.mp3");

    // Manejo de pantallas
    let mut screen = "menu";

    while window.is_open() {
        // Escucha de inputs
        if window.is_key_down(Key::Escape) {
            break;
        }
        if window.is_key_down(Key::M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }

        framebuffer.clear();

        match screen {
            "menu" => {
                framebuffer.draw_text("Presiona ENTER para comenzar", 400, 450, Scale::uniform(32.0), 0xFFFFFF);
                if window.is_key_down(Key::Enter) {
                    screen = "game";
                }
            },
            "game" => {
                // Captura del movimiento del mouse
                if let Some((mouse_x, _)) = window.get_mouse_pos(MouseMode::Clamp) {
                    let mouse_delta_x = mouse_x - last_mouse_x;
                    player.a += mouse_delta_x * 0.005; // Cambiado a suma para invertir la rotación
                    last_mouse_x = mouse_x;
                }

                // Procesar eventos
                process_events(&window, &mut player, &maze, block_size);

                if mode == "2D" {
                    render2d(&mut framebuffer, &player);
                } else {
                    let mut z_buffer = vec![f32::INFINITY; framebuffer.width];
                    render3d(&mut framebuffer, &player, &mut z_buffer);
                    render_enemies(&mut framebuffer, &player, &mut z_buffer);
                }

                // Renderizar el minimapa
                render_minimap(&mut framebuffer, &player);

                // Verificar condición de victoria
                if has_won(&player, &goal_position, block_size) {
                    screen = "win";
                }
            },
            "win" => {
                draw_victory_screen(&mut framebuffer);
            },
            _ => {},
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