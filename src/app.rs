use piston_window::{clear, rectangle, Context, G2d, MouseButton};

use crate::cell::{CellCoordinates, CellState};
use crate::field::{
    Field, BLOCKED_CELL_COLOR, CHOSEN_CELL_COLOR, EMPTY_CELL_COLOR, EMPTY_FIELD_COLOR,
    VISITED_CELL_COLOR,
};
use crate::pathfinder::pathfinder_a_star;
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
            mouse_coordinates: Vec2f {
                raw_x: 0.0,
                raw_y: 0.0,
            },
            selected_cell: None,
        }
    }

    pub fn randomize_blocks_on_field(&mut self) {
        self.field.make_noise();
    }

    pub fn render_field(&mut self, context: Context, g2d: &mut G2d) {
        clear(EMPTY_FIELD_COLOR, g2d);

        for x in 0..self.settings.cells_number {
            for y in 0..self.settings.cells_number {
                let cell = self.field.get_cell(x, y);

                let color: [f32; 4] = match cell.borrow_mut().cell_state {
                    CellState::Blocked => BLOCKED_CELL_COLOR,
                    CellState::Visited => VISITED_CELL_COLOR,
                    CellState::Chosen => CHOSEN_CELL_COLOR,
                    CellState::Empty => EMPTY_CELL_COLOR,
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
            let mut cell = self.field.get_cell(coordinates.x, coordinates.y);
            cell.borrow_mut().cell_state = CellState::Chosen;
            rectangle(
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

            let cell_ref_start = self.field.get_cell(x, y);
            let cell_ref_end = self.field.get_cell(19, 19);

            let res = pathfinder_a_star(cell_ref_start, cell_ref_end, &mut self.field);

            if let Some(cells) = res {
                cells
                    .iter()
                    .map(|tile| {
                        tile.0.borrow_mut().cell_state = CellState::Chosen;
                    })
                    .count();
            }
        }
    }

    pub fn on_mouse_move(&mut self, args: &[f64; 2]) {
        self.mouse_coordinates.raw_x = args[0];
        self.mouse_coordinates.raw_y = args[1];
    }
}
