use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use piston_window::{clear, Context, G2d, Glyphs, MouseButton, rectangle, text, Transformed};

use crate::algorithms::{Algorithm, AlgorithmType};
use crate::algorithms::a_star::AStar;
use crate::algorithms::breadth_first_search::BFS;
use crate::algorithms::depth_first_search::DFS;
use crate::algorithms::dijkstra::Dijkstra;
use crate::algorithms::greedy_best_first_search::GBFS;
use crate::cell::{CellCoordinates, CellState};
use crate::colors::{BLOCKED_CELL_COLOR, CHOSEN_CELL_COLOR, EMPTY_CELL_COLOR, EMPTY_FIELD_COLOR, END_CELL_COLOR, START_CELL_COLOR, VISITED_CELL_COLOR};
use crate::field::Field;
use crate::settings::{Settings, Vec2f};
use crate::state::{SharedState, State};

struct Alg(Arc<Mutex<dyn Algorithm + Send + Sync>>);

pub struct App {
    pub pathfinder_handler: Option<JoinHandle<()>>,
    algorithm: Alg,
    settings: Settings,
    state: SharedState,
    mouse_coordinates: Vec2f,
    selected_cell: Option<CellCoordinates>,
}

impl App {
    pub fn new(settings: Settings, algorithm_type: &AlgorithmType) -> App {
        let mut field = Field::new(settings.cells_number);
        field.make_noise();
        field.set_prices();

        let state = SharedState::new(State::new(field, 1.0));
        let algorithm: Alg = match algorithm_type {
            AlgorithmType::BFS => Alg(Arc::new(Mutex::new(BFS))),
            AlgorithmType::DFS => Alg(Arc::new(Mutex::new(DFS))),
            AlgorithmType::GBFS => Alg(Arc::new(Mutex::new(GBFS))),
            AlgorithmType::Dijkstra => Alg(Arc::new(Mutex::new(Dijkstra))),
            AlgorithmType::AStar => Alg(Arc::new(Mutex::new(AStar))),
        };

        App {
            pathfinder_handler: None,
            algorithm,
            state,
            settings,
            mouse_coordinates: Vec2f {
                raw_x: 0.0,
                raw_y: 0.0,
            },
            selected_cell: None,
        }
    }

    pub fn start(&mut self) {
        let a = Arc::clone(&self.algorithm.0);
        let s = self.state.clone();
        let algorithm_thread = thread::Builder::new()
            .name("algorithm".to_string())
            .spawn(move || {
                let b = a.lock().unwrap();
                b.search(s);
            })
            .unwrap();
        self.pathfinder_handler = Some(algorithm_thread);
    }

    pub fn render(&mut self, context: Context, g2d: &mut G2d, glyphs: &mut Glyphs) {
        clear(EMPTY_FIELD_COLOR, g2d);

        for x in 0..self.settings.cells_number {
            for y in 0..self.settings.cells_number {
                let cell = self.state.get().field().get_cell(x, y);

                let color: [f32; 4] = match cell.get().get_state() {
                    CellState::Blocked => BLOCKED_CELL_COLOR,
                    CellState::Visited => VISITED_CELL_COLOR,
                    CellState::Chosen => CHOSEN_CELL_COLOR,
                    CellState::Empty => EMPTY_CELL_COLOR,
                    CellState::End => END_CELL_COLOR,
                    CellState::Start => START_CELL_COLOR,
                };

                let cell_raw_x = (x as f64) * self.settings.cell_size.raw_x;
                let cell_raw_y = (y as f64) * self.settings.cell_size.raw_y;

                rectangle(
                    color,
                    [
                        cell_raw_x,
                        cell_raw_y,
                        self.settings.cell_size.raw_x,
                        self.settings.cell_size.raw_y,
                    ],
                    context.transform,
                    g2d,
                );

                let transform = context.transform.trans(
                    cell_raw_x + self.settings.cell_offset.raw_x,
                    cell_raw_y + self.settings.cell_offset.raw_y,
                );

                text::Text::new_color(piston_window::color::BLACK, self.settings.font_size)
                    .draw(
                        &*cell.get().cost.to_string(),
                        glyphs,
                        &context.draw_state,
                        transform,
                        g2d,
                    )
                    .unwrap();
            }
        }

        for n in 1..self.settings.cells_number {
            let border_width = 1.0;
            //vertical lines
            rectangle(
                BLOCKED_CELL_COLOR,
                [
                    (n as f64) * self.settings.cell_size.raw_x - border_width,
                    0.0,
                    border_width,
                    self.settings.window_size.raw_y,
                ],
                context.transform,
                g2d,
            );
            //horizontal lines
            rectangle(
                BLOCKED_CELL_COLOR,
                [
                    0.0,
                    (n as f64) * self.settings.cell_size.raw_y - border_width,
                    self.settings.window_size.raw_x,
                    border_width,
                ],
                context.transform,
                g2d,
            );
        }
    }

    pub fn on_mouse_click(&mut self, button: &MouseButton) {
        if let &MouseButton::Left = button {
            let x = (self.mouse_coordinates.raw_x / self.settings.cell_size.raw_x) as u16;
            let y = (self.mouse_coordinates.raw_y / self.settings.cell_size.raw_y) as u16;

            println!(
                "COLOR: {:?}", self.state.get().field().get_cell(x, y).get().get_state()
            );
        }
    }

    pub fn on_mouse_move(&mut self, args: &[f64; 2]) {
        self.mouse_coordinates.raw_x = args[0];
        self.mouse_coordinates.raw_y = args[1];
    }
}
