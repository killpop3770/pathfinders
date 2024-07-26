use piston_window::{clear, Context, G2d, MouseButton, rectangle};

use crate::field::{BLOCKED_CELL_COLOR, CellCoordinates, CellState, CHOSEN_CELL_COLOR, EMPTY_CELL_COLOR, EMPTY_FIELD_COLOR, Field};
use crate::pathfinder::check_neighbors;
use crate::settings::{Settings, Vec2f};

pub struct App {
    settings: Settings,
    field: Field,
    mouse_coordinates: Vec2f,
    selected_cell: Option<CellCoordinates>,
}

impl App {
    pub fn new(settings: Settings) -> App {
        App {
            field: Field::new(settings.cells_number),
            settings,
            mouse_coordinates: Vec2f { raw_x: 0.0, raw_y: 0.0 },
            selected_cell: None,
        }
    }

    pub fn render_field(&mut self, context: Context, g2d: &mut G2d) {
        clear(EMPTY_FIELD_COLOR, g2d);

        for x in 0..self.settings.cells_number {
            for y in 0..self.settings.cells_number {
                let cell = self.field.get_cell(x, y);

                let color;
                if cell.cell_type == CellState::Chosen {
                    color = CHOSEN_CELL_COLOR;
                } else {
                    color = EMPTY_CELL_COLOR;
                }

                rectangle(
                    color,
                    [
                        (x as f64) * self.settings.cell_size.raw_x,
                        (y as f64) * self.settings.cell_size.raw_y,
                        self.settings.cell_size.raw_x,
                        self.settings.cell_size.raw_y
                    ],
                    context.transform,
                    g2d,
                );
            }
        }

        for n in 1..self.settings.cells_number {
            let border_width = 1.0;
            //vertical line
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
            //horizontal line
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

        if let Some(ref coordinates) = self.selected_cell {
            let cell = self.field.get_cell(coordinates.x, coordinates.y);
            cell.cell_type = CellState::Chosen;
            rectangle
                (
                    CHOSEN_CELL_COLOR,
                    [
                        (coordinates.x as f64) * self.settings.cell_size.raw_x,
                        (coordinates.y as f64) * self.settings.cell_size.raw_y,
                        self.settings.cell_size.raw_x,
                        self.settings.cell_size.raw_y,
                    ],
                    context.transform,
                    g2d,
                )
        }
    }

    pub fn on_mouse_click(&mut self, button: &MouseButton) {
        if let &MouseButton::Left = button {
            let x = (self.mouse_coordinates.raw_x / self.settings.cell_size.raw_x) as u16;
            let y = (self.mouse_coordinates.raw_y / self.settings.cell_size.raw_y) as u16;
            println!("{:?}", self.field.get_cell(x, y).cell_coordinates);
            println!("{:?}", check_neighbors(CellCoordinates { x, y }, &self.field));

            // println!("{} {}",
            //          (self.mouse_coordinates.x / self.settings.cell_size.x) as u16,
            //          (self.mouse_coordinates.y / self.settings.cell_size.y) as u16,
            // );

            // self.selected_cell = Some(CellCoordinates {
            //     x: (self.mouse_coordinates.x / self.settings.cell_size.x) as u16,
            //     y: (self.mouse_coordinates.y / self.settings.cell_size.y) as u16,
            // });
        }
    }

    pub fn on_mouse_move(&mut self, args: &[f64; 2]) {
        self.mouse_coordinates.raw_x = args[0];
        self.mouse_coordinates.raw_y = args[1];
    }
}