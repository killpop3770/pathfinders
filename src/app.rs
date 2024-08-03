use piston_window::{clear, rectangle, Context, G2d, MouseButton};
use std::thread;
use std::thread::JoinHandle;

use crate::cell::{CellCoordinates, CellState};
use crate::field::{Field, BLOCKED_CELL_COLOR, CHOSEN_CELL_COLOR, EMPTY_CELL_COLOR, EMPTY_FIELD_COLOR, END_CELL_COLOR, VISITED_CELL_COLOR, START_CELL_COLOR};
use crate::pathfinder::{Algorithm};
use crate::settings::{Settings, Vec2f};
use crate::state::{SharedState, State};

pub struct App {
    pathfinder_handler: JoinHandle<()>,
    settings: Settings,
    state: SharedState,
    mouse_coordinates: Vec2f,
    selected_cell: Option<CellCoordinates>,
}

impl App {
    pub fn new(settings: Settings, algorithm: Box<dyn Algorithm + Send>) -> App {
        let mut field = Field::new(settings.cells_number);
        field.make_noise();

        let state = SharedState::new(State::new(field, 0.5));
        let state_copy = state.clone();

        let algorithm_thread = thread::Builder::new()
            .name("algorithm".to_string())
            .spawn(move || {
                algorithm.search(state_copy);
            })
            .unwrap();

        App {
            pathfinder_handler: algorithm_thread,
            state,
            settings,
            mouse_coordinates: Vec2f {
                raw_x: 0.0,
                raw_y: 0.0,
            },
            selected_cell: None,
        }
    }

    pub fn start(&mut self) {}

    pub fn update(&mut self, context: Context, g2d: &mut G2d) {}

    pub fn render(&mut self, context: Context, g2d: &mut G2d) {
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

                rectangle(
                    color,
                    [
                        (x as f64) * self.settings.cell_size.raw_x,
                        (y as f64) * self.settings.cell_size.raw_y,
                        self.settings.cell_size.raw_x,
                        self.settings.cell_size.raw_y,
                    ],
                    context.transform,
                    g2d,
                );
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

        // if let Some(ref coordinates) = self.selected_cell {
        //     let cell = self.state.get().field.get_cell(coordinates.x, coordinates.y);
        //     cell.lock().unwrap().state = CellState::Chosen;
        //     rectangle(
        //         CHOSEN_CELL_COLOR,
        //         [
        //             (coordinates.x as f64) * self.settings.cell_size.raw_x,
        //             (coordinates.y as f64) * self.settings.cell_size.raw_y,
        //             self.settings.cell_size.raw_x,
        //             self.settings.cell_size.raw_y,
        //         ],
        //         context.transform,
        //         g2d,
        //     )
        // }
    }

    pub fn on_mouse_click(&mut self, button: &MouseButton) {
        if let &MouseButton::Left = button {
            let x = (self.mouse_coordinates.raw_x / self.settings.cell_size.raw_x) as u16;
            let y = (self.mouse_coordinates.raw_y / self.settings.cell_size.raw_y) as u16;

            println!(
                "COLOR: {:?}",
                self.state.get().field().get_cell(x, y).get().get_state()
            );
        }
    }

    pub fn on_mouse_move(&mut self, args: &[f64; 2]) {
        self.mouse_coordinates.raw_x = args[0];
        self.mouse_coordinates.raw_y = args[1];
    }
}
