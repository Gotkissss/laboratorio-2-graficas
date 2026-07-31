use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 800;

const FB_WIDTH: usize = 100;
const FB_HEIGHT: usize = 100;

const CELL_SIZE: i32 = WINDOW_WIDTH / FB_WIDTH as i32;

const ALIVE: Color = Color::WHITE;
const DEAD: Color = Color::BLACK;

struct Framebuffer {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Framebuffer {
    fn new(width: usize, height: usize, background: Color) -> Self {
        Framebuffer {
            width,
            height,
            pixels: vec![background; width * height],
        }
    }

    fn point(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }

    fn get_color(&self, x: usize, y: usize) -> Color {
        self.pixels[y * self.width + x]
    }
}

fn render(d: &mut RaylibDrawHandle, fb: &Framebuffer) {
    for y in 0..fb.height {
        for x in 0..fb.width {
            let color = fb.get_color(x, y);
            d.draw_rectangle(
                x as i32 * CELL_SIZE,
                y as i32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                color,
            );
        }
    }
}

fn count_alive_neighbors(fb: &Framebuffer, x: usize, y: usize) -> u8 {
    let mut count = 0;
    for dy in -1i32..=1 {
        for dx in -1i32..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && ny >= 0 && (nx as usize) < fb.width && (ny as usize) < fb.height {
                if fb.get_color(nx as usize, ny as usize) == ALIVE {
                    count += 1;
                }
            }
        }
    }
    count
}

// Calcula el siguiente estado completo aplicando las 4 reglas de Conway.
// Cada celda se pinta con point(), viva o muerta, así que no hace falta
// limpiar el framebuffer aparte: la lógica ya cubre los 100x100 pixeles.
fn next_generation(current: &Framebuffer) -> Framebuffer {
    let mut next = Framebuffer::new(current.width, current.height, DEAD);

    for y in 0..current.height {
        for x in 0..current.width {
            let neighbors = count_alive_neighbors(current, x, y);
            let alive_now = current.get_color(x, y) == ALIVE;

            let alive_next = match (alive_now, neighbors) {
                (true, 2) | (true, 3) => true,   // survival
                (false, 3) => true,              // reproduction
                _ => false,                      // under/overpopulation
            };

            next.point(x, y, if alive_next { ALIVE } else { DEAD });
        }
    }

    next
}

// ---------- Organismos ----------
// Still lifes
fn place_block(fb: &mut Framebuffer, x: usize, y: usize) {
    for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
        fb.point(x + dx, y + dy, ALIVE);
    }
}

fn place_beehive(fb: &mut Framebuffer, x: usize, y: usize) {
    for (dx, dy) in [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)] {
        fb.point(x + dx, y + dy, ALIVE);
    }
}

fn place_loaf(fb: &mut Framebuffer, x: usize, y: usize) {
    for (dx, dy) in [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3)] {
        fb.point(x + dx, y + dy, ALIVE);
    }
}

fn place_boat(fb: &mut Framebuffer, x: usize, y: usize) {
    for (dx, dy) in [(0, 0), (1, 0), (0, 1), (2, 1), (1, 2)] {
        fb.point(x + dx, y + dy, ALIVE);
    }
}

fn place_tub(fb: &mut Framebuffer, x: usize, y: usize) {
    for (dx, dy) in [(1, 0), (0, 1), (2, 1), (1, 2)] {
        fb.point(x + dx, y + dy, ALIVE);
    }
}

// Oscillators
fn place_blinker(fb: &mut Framebuffer, x: usize, y: usize) {
    for (dx, dy) in [(0, 0), (1, 0), (2, 0)] {
        fb.point(x + dx, y + dy, ALIVE);
    }
}

fn place_toad(fb: &mut Framebuffer, x: usize, y: usize) {
    for (dx, dy) in [(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)] {
        fb.point(x + dx, y + dy, ALIVE);
    }
}

fn place_beacon(fb: &mut Framebuffer, x: usize, y: usize) {
    for (dx, dy) in [(0, 0), (1, 0), (0, 1), (3, 2), (2, 3), (3, 3)] {
        fb.point(x + dx, y + dy, ALIVE);
    }
}

// Spaceships
fn place_glider(fb: &mut Framebuffer, x: usize, y: usize) {
    for (dx, dy) in [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)] {
        fb.point(x + dx, y + dy, ALIVE);
    }
}

fn place_lwss(fb: &mut Framebuffer, x: usize, y: usize) {
    for (dx, dy) in [
        (1, 0), (2, 0), (3, 0), (4, 0),
        (0, 1), (4, 1),
        (4, 2),
        (0, 3), (3, 3),
    ] {
        fb.point(x + dx, y + dy, ALIVE);
    }
}

fn setup_initial_pattern(fb: &mut Framebuffer) {
    // Grid de gliders viajando por toda la pantalla
    for row in 0..5 {
        for col in 0..5 {
            place_glider(fb, 5 + col * 18, 5 + row * 18);
        }
    }

    // Still lifes repartidos
    place_block(fb, 90, 5);
    place_beehive(fb, 90, 15);
    place_loaf(fb, 90, 25);
    place_boat(fb, 90, 35);
    place_tub(fb, 90, 45);

    // Osciladores repartidos
    for row in 0..4 {
        place_blinker(fb, 92, 55 + row * 5);
    }
    place_toad(fb, 60, 90);
    place_beacon(fb, 70, 90);

    // Más spaceships en otras zonas
    place_lwss(fb, 10, 92);
    place_lwss(fb, 30, 92);
    place_lwss(fb, 50, 92);
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Conway's Game of Life")
        .build();

    rl.set_target_fps(10);

    let mut fb = Framebuffer::new(FB_WIDTH, FB_HEIGHT, DEAD);
    setup_initial_pattern(&mut fb);

    while !rl.window_should_close() {
        fb = next_generation(&fb);

        let mut d = rl.begin_drawing(&thread);
        render(&mut d, &fb);
    }
}