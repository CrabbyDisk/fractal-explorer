use gloo::timers::callback::Interval;
use rand::Rng;
use yew::html::Scope;
use yew::{classes, html, Component, Context, Html};

pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}
pub enum Msg {
    Random,
    Zoom,
    Dezoom,
    Move(Dir),
}

pub struct App {
    active: bool,
    cells: Vec<bool>,
    cells_width: usize,
    _interval: Interval,
}

impl App {
    pub fn random_mutate(&mut self) {
        for cell in self.cells.iter_mut() {
            *cell = rand::thread_rng().gen()
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| Msg::Random);
        let interval = Interval::new(200, move || callback.emit(()));

        let (cells_width, cells_height) = (53, 40);

        Self {
            active: false,
            cells: vec![true; cells_width * cells_height],
            cells_width,
            _interval: interval,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Random => {
                self.random_mutate();
                log::info!("Random");
                true
            }
            Msg::Zoom => {
                self.active = true;
                log::info!("Zoom");
                false
            }
            Msg::Dezoom => true,
            Msg::Move(_) => {
                self.active = false;
                log::info!("Stop");
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cell_rows = self
            .cells
            .chunks(self.cells_width)
            .map(|x: &[bool]| x.to_vec())
            .collect::<Vec<Vec<bool>>>();
        let render = dbg!(grid_to_string(cell_rows));

        html! {
            <div>
                <section class="game-container">
                    <header class="app-header">
                        <h1 class="app-title">{ "Fractal Explorer" }</h1>
                    </header>
                    <section class="game-area">
                        <div class="game-of-life">
        {render}
                        </div>
                        <div class="game-buttons">
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Random)}>{ "[ Random ]" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Zoom)}>{ "[ Zoom in ]" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Dezoom)}>{ "[ Zoom out ]" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Move(Dir::Up))}>{ "Stop" }</button>
                        </div>
                    </section>
                </section>
                <footer class="app-footer">
                    <strong class="footer-text">
                      { "Game of Life - a yew experiment " }
                    </strong>
                    <a href="https://github.com/yewstack/yew" target="_blank">{ "source" }</a>
                </footer>
            </div>
        }
    }
}

fn wrap(coord: isize, range: isize) -> usize {
    let result = if coord < 0 {
        coord + range
    } else if coord >= range {
        coord - range
    } else {
        coord
    };
    result as usize
}

fn grid_to_string(grid: Vec<Vec<bool>>) -> String {
    let rows = grid.len();
    let columns = grid[0].len();
    let mut chunks = String::new();
    for row_start in (0..rows).step_by(4) {
        for col_start in (0..columns).step_by(2) {
            let mut chunk = Vec::new();

            // Collect the sub-grid/chunk from grid
            for r in row_start..std::cmp::min(row_start + 4, rows) {
                let row = grid[r][col_start..std::cmp::min(col_start + 2, columns)].to_vec();
                chunk.push(row);
            }
            let thing: [[bool; 2]; 4] = [
                chunk[0].clone().try_into().unwrap(),
                chunk[1].clone().try_into().unwrap(),
                chunk[2].clone().try_into().unwrap(),
                chunk[3].clone().try_into().unwrap(),
            ];

            chunks.push(grid_to_char(thing));
        }
        chunks.push('\n');
    }
    chunks
}

fn grid_to_char(grid: [[bool; 2]; 4]) -> char {
    let mut braille_value: u8 = 0;
    for row in 0..2 {
        for col in 0..4 {
            let bit_position = match (row, col) {
                (0, 0) => 0, // Dot 1
                (1, 0) => 1, // Dot 2
                (0, 1) => 3, // Dot 4
                (1, 1) => 4, // Dot 5
                (0, 2) => 2, // Dot 3
                (1, 2) => 5, // Dot 6
                (0, 3) => 6, // Dot 7
                (1, 3) => 7, // Dot 8
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
    std::char::from_u32(braille_unicode_base + braille_value as u32).unwrap_or('?')
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::trace!("Initializing yew...");
    yew::Renderer::<App>::new().render();
}
