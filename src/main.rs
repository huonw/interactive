#![feature(globs)]
extern crate shader_version;
extern crate input;
extern crate event;
extern crate image;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;
use opengl_graphics::{ Gl,Texture };
use sdl2_window::Sdl2Window;
use event::{
    EventIterator,
    EventSettings,
    WindowSettings,
};
use image::GenericImage;


use std::{rand, mem};

#[deriving(PartialEq, Show, Clone, Rand)]
struct CellValue {
    x: f32
}

const ALIVE: CellValue = CellValue { x: 1.0 };
const DEAD: CellValue = CellValue { x: 0.0 };

fn update(cells: &Vec<Vec<CellValue>>, output: &mut Vec<Vec<CellValue>>) {
    let width = cells.len();
    let height = (*cells)[0].len();
    for i in range(0, width) {
        for j in range(0, height) {
            let mut neighbours = 0.0;
            for ii in range(-1i, 1 + 1) {
                for jj in range(-1i, 1 + 1) {
                    if ii == 0 && jj == 0 { continue }

                    neighbours += (*cells)[(i + ii as uint) % width][(j + jj as uint) % height].x;
                }
            }

            let old = (*cells)[i][j].x;
            let is_alive = 2.8 - old <= neighbours && neighbours <= 3.2;

            (*output)[i][j].x = if is_alive { 0.9 * old + 0.1 } else { 0.98 * old };
        }
    }
}

fn main() {
    let opengl = shader_version::opengl::OpenGL_3_2;
    let (width, height) = (300, 300);
    let (mut window_width, mut window_height) = (300, 300);

    let mut cells = Vec::from_fn(width as uint, |_|
                                 Vec::from_fn(height as uint,
                                              |_| {
                                                  if rand::random::<f32>() < 0.2 { ALIVE }
                                                  else { DEAD }
                                              }));
    let mut other_cells = cells.clone();

    let mut window = Sdl2Window::new(
        opengl,
        WindowSettings {
            title: "Paint".to_string(),
            size: [window_width, window_height],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
        );
    let mut image = image::ImageBuf::new(width, height);
    let mut draw = false;
    let mut texture = Texture::from_image(&image);
    let event_settings = EventSettings {
        updates_per_second: 60,
        max_frames_per_second: 120,
    };
    let ref mut gl = Gl::new(opengl);

    for e in EventIterator::new(&mut window, &event_settings) {
        use event::{ MouseCursorEvent, PressEvent, ReleaseEvent, RenderEvent, ResizeEvent, UpdateEvent };
        e.resize(|w, h| {
            window_width = w;
            window_height = h;
        });
        e.update(|_args| {
            update(&cells, &mut other_cells);
            mem::swap(&mut cells, &mut other_cells);
        });

        e.render(|args| {
            use graphics::*;
            gl.viewport(0, 0, args.width as i32, args.height as i32);
            let c = Context::abs(args.width as f64, args.height as f64);
            c.rgb(1.0, 1.0, 1.0).draw(gl);

            for (i,row) in cells.iter().enumerate() {
                for (j,cell) in row.iter().enumerate() {
                    let value = 255 - (cell.x * 255.0) as u8;
                    let colour = image::Rgba(value, value, value, 255);
                    image.put_pixel(i as u32, j as u32, colour);
                }
            }
            texture.update(&image);


            c.image(&texture).scale(args.width as f64 / width as f64,
                                    args.height as f64 / height as f64).draw(gl);

            window_width = args.width;
            window_height = args.height;
        });
        e.press(|button| {
            if button == input::Mouse(input::mouse::Left) {
                draw = true
            }
        });
        e.release(|button| {
            if button == input::Mouse(input::mouse::Left) {
                draw = false
            }
        });
        if draw {
            e.mouse_cursor(|x, y| {
                let (x, y) = ((x / window_width as f64 * width as f64) as uint,
                              (y / window_height as f64 * width as f64) as uint);
                if x < width as uint && y < height as uint {
                    cells[x][y].x = 1.0 - cells[x][y].x;
                }
            });
        }
    }
}
