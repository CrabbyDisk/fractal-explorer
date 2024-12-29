use std::iter::successors;
use std::ops::Range;

use gloo::net::websocket::Message;
use num_complex::Complex64;
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

struct App {
    min_real: f64,
    max_real: f64,
    min_imag: f64,
    max_imag: f64,
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        // let result = grid_to_string(self.tiles.as_slice());
        let result = grid_to_string(&mandelbrot(
            128,
            128,
            self.min_real..self.max_real,
            self.min_imag..self.max_imag,
            2000,
        ));
        html!(
        <>
        <p>{result}</p>
            <div class="game-buttons">
                 <button class="game-button" onclick={ctx.link().callback(|_| Msg::Random)}>{ "[ Random ]" }</button>
                <button class="game-button" onclick={ctx.link().callback(|_| Msg::ZoomIn)}>{ "[ Zoom in ]" }</button>
                <button class="game-button" onclick={ctx.link().callback(|_| Msg::ZoomOut)}>{ "[ Zoom out ]" }</button>
                <button class="game-button" onclick={ctx.link().callback(|_| Msg::Move(Dir::Up))}>{ "Stop" }</button>
             </div>
        </>
        )
    }
    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            min_real: -2.0,
            max_real: 1.0,
            min_imag: -1.5,
            max_imag: 1.5,
        }
    }
    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Random => true,
            Msg::ZoomIn => true,
            Msg::ZoomOut => true,
            Msg::Move(x) => true,
        }
    }
}

impl App {
    fn zoom(factor: f64) {}
}

fn grid_to_string(grid: &[Vec<bool>]) -> String {
    let rows = grid.len();
    let columns = grid[0].len();

    let mut chunks = String::new();
    for row_start in (0..rows).step_by(4) {
        for col_start in (0..columns).step_by(2) {
            // Collect the sub-grid/chunk from grid

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
    width: u32,
    height: u32,
    real_range: Range<f64>,
    imag_range: Range<f64>,
    max_iters: usize,
) -> Vec<Vec<bool>> {
    let real_step = (real_range.end - real_range.start) / width as f64;
    let imag_step = (imag_range.end - imag_range.start) / height as f64;

    (0..height)
        .map(|j| {
            (0..width)
                .map(move |i| {
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
                .collect()
        })
        .collect()
}
fn main() {
    yew::Renderer::<App>::new().render();
}
