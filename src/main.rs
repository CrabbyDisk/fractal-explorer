use std::array;
use std::ops::Range;
use std::{f64, iter::successors};

use num_complex::Complex64;
use rayon::iter::{ParallelBridge, ParallelIterator};
use wgpu::util::DeviceExt;
use yew::{html, Component};

enum Dir {
    Up,
    Down,
    Left,
    Right,
}
enum Msg {
    Random,
    ZoomOut,
    ZoomIn,
    Move(Dir),
}

const WIDTH: usize = 128;
const HEIGHT: usize = 128;

const MIN_REAL: f64 = -2.0;
const MAX_REAL: f64 = 1.0;
const MIN_IMAG: f64 = -1.5;
const MAX_IMAG: f64 = 1.5;
const ITERATIONS: usize = 1000;

struct App {
    center: Complex64,
    zoom_factor: i32,
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        // let result = grid_to_string(self.tiles.as_slice());

        let zoomed = get_zoomed(self.center, self.zoom_factor);
        let result = grid_to_string(mandelbrot(zoomed.0, zoomed.1, ITERATIONS));
        html!(
        <>
        <h1> {"fractal-explorer"}</h1>
        <p>{result}</p>
            <div class="game-buttons">
                 <button class="game-button" onclick={ctx.link().callback(|_| Msg::Random)}>{ "[ Random ]" }</button>
                <button class="game-button" onclick={ctx.link().callback(|_| Msg::ZoomIn)}>{ "[ Zoom in ]" }</button>
                <button class="game-button" onclick={ctx.link().callback(|_| Msg::ZoomOut)}>{ "[ Zoom out ]" }</button>
                <button class="game-button" onclick={ctx.link().callback(|_| Msg::Move(Dir::Up))}>{ "[ Move up ]" }</button>
                <button class="game-button" onclick={ctx.link().callback(|_| Msg::Move(Dir::Down))}>{ "[ Move down ]" }</button>
                <button class="game-button" onclick={ctx.link().callback(|_| Msg::Move(Dir::Left))}>{ "[ Move left ]" }</button>
                <button class="game-button" onclick={ctx.link().callback(|_| Msg::Move(Dir::Right))}>{ "[ Move right ]" }</button>
             </div>
        </>
        )
    }
    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            zoom_factor: 0,
            center: Complex64::new((MIN_REAL + MAX_REAL) / 2.0, (MIN_IMAG + MAX_IMAG) / 2.0),
        }
    }
    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Random => true,
            Msg::ZoomIn => {
                self.zoom_factor += 1;
                true
            }
            Msg::ZoomOut => {
                self.zoom_factor -= 1;
                true
            }
            Msg::Move(x) => {
                self.r#move(x);
                true
            }
        }
    }
}

impl App {
    fn r#move(&mut self, dir: Dir) {
        let scaled_move = 2.0_f64.powi(-self.zoom_factor);
        match dir {
            Dir::Up => {
                self.center.im -= scaled_move;
            }
            Dir::Down => {
                self.center.im += scaled_move;
            }
            Dir::Left => {
                self.center.re -= scaled_move;
            }
            Dir::Right => {
                self.center.re += scaled_move;
            }
        };
    }
}

fn grid_to_string(grid: [[bool; WIDTH]; HEIGHT]) -> String {
    let rows = grid.len();
    let columns = grid[0].len();

    let mut chunks = String::new();
    for row_start in (0..rows).step_by(4) {
        for col_start in (0..columns).step_by(2) {
            let chunk = grid
                .iter()
                .take(std::cmp::min(row_start + 4, rows))
                .skip(row_start)
                .fold(Vec::new(), |mut acc: Vec<[bool; 2]>, x| {
                    acc.push(
                        x[col_start..std::cmp::min(col_start + 2, columns)]
                            .try_into()
                            .unwrap(),
                    );
                    acc
                });
            let thing: [[bool; 2]; 4] = [chunk[0], chunk[1], chunk[2], chunk[3]];

            chunks.push(braille_from_8dot_grid(thing));
        }
        chunks.push_str("\n\r");
    }
    chunks
}

fn braille_from_8dot_grid(grid: [[bool; 2]; 4]) -> char {
    // Flatten the 2x4 grid into a single list of 8 booleans
    let mut braille_value = 0;

    // Map each grid position to its respective bit in the Braille encoding
    for row in 0..4 {
        for col in 0..2 {
            let bit_position = match (row, col) {
                (0, 0) => 0, // Dot 1
                (1, 0) => 1, // Dot 2
                (2, 0) => 2, // Dot 3
                (0, 1) => 3, // Dot 4
                (1, 1) => 4, // Dot 5
                (2, 1) => 5, // Dot 6
                (3, 0) => 6, // Dot 7
                (3, 1) => 7, // Dot 8
                _ => unreachable!(),
            };

            if grid[row][col] {
                braille_value |= 1 << bit_position; // Set the corresponding bit
            }
        }
    }

    // Base Unicode value for Braille patterns
    let braille_unicode_base = 0x2800;

    // Add the computed value to the base to get the Unicode character
    std::char::from_u32(braille_unicode_base + braille_value).unwrap_or('?')
}

fn mandelbrot(
    real_range: Range<f64>,
    imag_range: Range<f64>,
    max_iters: usize,
) -> [[bool; WIDTH]; HEIGHT] {
    let real_step = (real_range.end - real_range.start) / WIDTH as f64;
    let imag_step = (imag_range.end - imag_range.start) / HEIGHT as f64;

    array::from_fn(|j| {
        array::from_fn(move |i| {
            let c = Complex64::new(
                real_range.start + i as f64 * real_step, // Real part of c
                imag_range.start + j as f64 * imag_step, // Imaginary part of c
            );

            successors(Some(Complex64::ZERO), |z| {
                if z.norm_sqr() > 4.0 {
                    None
                } else {
                    Some(z * z + c)
                }
            })
            .nth(max_iters)
            .is_some()
        })
    })
}
/* async fn gpu_mandelbrot(
    real_range: Range<f64>,
    imag_range: Range<f64>,
    max_iters: usize,
) -> [[bool; WIDTH]; HEIGHT] {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();

    // `request_device` instantiates the feature specific connection to the GPU, defining some parameters,
    //  `features` being the available features.
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults(),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
            },
            None,
        )
        .await
        .unwrap();
    let cs_module = device.create_shader_module(wgpu::include_wgsl!("mandelbrot.wgsl"));

    let size = size_of::<[[bool; WIDTH]; HEIGHT]>() as wgpu::BufferAddress;

    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
  let storage_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Storage Buffer"),
        size,
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

}
 */
fn get_zoomed(center: Complex64, zoom_factor: i32) -> (Range<f64>, Range<f64>) {
    let calc_factor = 2_f64.powi(zoom_factor);
    let new_real_width = (MAX_REAL - MIN_REAL) / calc_factor;
    let new_imag_height = (MAX_IMAG - MIN_IMAG) / calc_factor;
    (
        center.re - new_real_width / 2.0..center.re + new_real_width / 2.0,
        center.im - new_imag_height / 2.0..center.im + new_imag_height / 2.0,
    )
}
fn main() {
    yew::Renderer::<App>::new().render();
}
