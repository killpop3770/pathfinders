use piston_window::{clear, rectangle, text, Context, DrawState, G2d, Glyphs, Transformed};

use crate::config::constants::{
    BLOCKED_CELL_COLOR, EMPTY_CELL_COLOR, EMPTY_FIELD_COLOR, END_CELL_COLOR, PATH_CELL_COLOR,
    START_CELL_COLOR, VISITED_CELL_COLOR,
};
use crate::domain::cell::{AlgoState, Cell, CellType, VisualState};
use crate::domain::field::Field;

pub fn get_render_color(cell: &Cell) -> [f32; 4] {
    match (
        cell.get_visual_state(),
        cell.get_algo_state(),
        cell.get_cell_type(),
    ) {
        (VisualState::Start, _, _) => START_CELL_COLOR,
        (VisualState::End, _, _) => END_CELL_COLOR,
        (VisualState::Path, _, _) => PATH_CELL_COLOR,
        (_, AlgoState::Visited, _) => VISITED_CELL_COLOR,
        (VisualState::None, AlgoState::Unvisited, CellType::Blocked) => BLOCKED_CELL_COLOR,
        (VisualState::None, AlgoState::Unvisited, CellType::Empty) => EMPTY_CELL_COLOR,
    }
}

pub struct Renderer {
    cell_size: f64,
    font_size: u32,
    cell_offset_x: f64,
    cell_offset_y: f64,
}

impl Renderer {
    pub fn new(cell_size: f64, font_size: u32, cell_offset_x: f64, cell_offset_y: f64) -> Self {
        Self {
            cell_size,
            font_size,
            cell_offset_x,
            cell_offset_y,
        }
    }

    pub fn render_field(
        &self,
        context: Context,
        g2d: &mut G2d,
        glyphs: &mut Glyphs,
        field: &Field,
        show_costs: bool,
    ) {
        clear(EMPTY_FIELD_COLOR, g2d);

        let size = field.get_size();

        for x in 0..size {
            for y in 0..size {
                self.render_cell(context, g2d, glyphs, field, x, y, show_costs);
            }
        }

        self.render_grid(context, g2d, size);
    }

    #[allow(clippy::too_many_arguments)]
    fn render_cell(
        &self,
        context: Context,
        g2d: &mut G2d,
        glyphs: &mut Glyphs,
        field: &Field,
        x: usize,
        y: usize,
        show_costs: bool,
    ) {
        let cell = field.cell(x, y);
        let color = get_render_color(cell);

        let cell_x = (x as f64) * self.cell_size;
        let cell_y = (y as f64) * self.cell_size;

        rectangle(
            color,
            [cell_x, cell_y, self.cell_size, self.cell_size],
            context.transform,
            g2d,
        );

        if show_costs && cell.get_cell_type() != CellType::Blocked {
            self.render_cell_text(context, g2d, glyphs, cell_x, cell_y, &cell.cost.to_string());
        }
    }

    fn render_cell_text(
        &self,
        context: Context,
        g2d: &mut G2d,
        glyphs: &mut Glyphs,
        cell_x: f64,
        cell_y: f64,
        text_content: &str,
    ) {
        let transform = context
            .transform
            .trans(cell_x + self.cell_offset_x, cell_y + self.cell_offset_y);

        let _ = text::Text::new_color(piston_window::color::BLACK, self.font_size).draw(
            text_content,
            glyphs,
            &DrawState::default(),
            transform,
            g2d,
        );
    }

    fn render_grid(&self, context: Context, g2d: &mut G2d, size: usize) {
        let border_width = 1.0;
        let window_size = (size as f64) * self.cell_size;

        for n in 1..size {
            let pos = (n as f64) * self.cell_size;

            rectangle(
                BLOCKED_CELL_COLOR,
                [pos - border_width, 0.0, border_width, window_size],
                context.transform,
                g2d,
            );

            rectangle(
                BLOCKED_CELL_COLOR,
                [0.0, pos - border_width, window_size, border_width],
                context.transform,
                g2d,
            );
        }
    }
}
