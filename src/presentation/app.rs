use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

use piston_window::{Context, G2d, Glyphs, MouseButton};

use crate::algorithms::{AStar, Algorithm, AlgorithmType, Dijkstra, BFS, DFS, GBFS};
use crate::config::settings::AppConfig;
use crate::domain::cell::CellType;
use crate::domain::field::Field;
use crate::presentation::renderer::Renderer;
use crate::state::shared_state::{SharedState, State};

type Alg = Arc<dyn Algorithm + Send + Sync>;

pub struct App {
    pub pathfinder_handler: Option<JoinHandle<()>>,
    algorithm: Alg,
    config: AppConfig,
    state: SharedState,
    renderer: Renderer,
    mouse_x: f64,
    mouse_y: f64,
    show_costs: bool,
}

impl App {
    pub fn new(config: AppConfig, algorithm_type: &AlgorithmType) -> App {
        let mut field = Field::new(config.cells_number());
        field.make_noise();
        field.set_prices();

        let state = SharedState::new(State::new(field, config.default_speed()));

        let algorithm: Alg = match algorithm_type {
            AlgorithmType::BFS => Arc::new(BFS::new(state.clone())),
            AlgorithmType::DFS => Arc::new(DFS::new(state.clone())),
            AlgorithmType::Dijkstra => Arc::new(Dijkstra::new(state.clone())),
            AlgorithmType::GBFS => Arc::new(GBFS::new(state.clone())),
            AlgorithmType::AStar => Arc::new(AStar::new(state.clone())),
        };

        let show_costs = algorithm.uses_cell_cost();

        let renderer = Renderer::new(
            config.cell_size() as f64,
            config.font_size(),
            config.cell_offset_x(),
            config.cell_offset_y(),
        );

        App {
            pathfinder_handler: None,
            algorithm,
            config,
            state,
            renderer,
            mouse_x: 0.0,
            mouse_y: 0.0,
            show_costs,
        }
    }

    pub fn start(&mut self) {
        if self.pathfinder_handler.is_some() {
            return;
        }

        {
            let mut guard = self.state.lock();
            guard.field_mut().reset();
        }
        self.state.set_stop_flag(false);

        let algo = Arc::clone(&self.algorithm);

        let handle = thread::spawn(move || {
            algo.search();
        });
        self.pathfinder_handler = Some(handle);
    }

    pub fn stop(&mut self) {
        self.state.set_stop_flag(true);

        if let Some(handle) = self.pathfinder_handler.take() {
            let _ = handle.join();
        }
    }

    pub fn render(&mut self, context: Context, g2d: &mut G2d, glyphs: &mut Glyphs) {
        let field_snapshot = {
            let guard = self.state.lock();
            guard.field().clone()
        };

        self.renderer
            .render_field(context, g2d, glyphs, &field_snapshot, self.show_costs);
    }

    pub fn on_mouse_click(&mut self, button: &MouseButton) {
        if let MouseButton::Left = button {
            if self.pathfinder_handler.is_some() {
                return;
            }

            let cell_size = self.config.cell_size() as f64;
            let x = (self.mouse_x / cell_size) as usize;
            let y = (self.mouse_y / cell_size) as usize;

            let max_size = self.config.cells_number() as usize;
            if x >= max_size || y >= max_size {
                return;
            }

            let mut guard = self.state.lock();
            let field = guard.field_mut();

            if let Some(cell) = field.get_mut(x, y) {
                let new_type = if cell.get_cell_type() == CellType::Empty {
                    CellType::Blocked
                } else {
                    CellType::Empty
                };
                cell.set_cell_type(new_type);
                field.reset();
            }
        }
    }

    pub fn on_mouse_move(&mut self, args: &[f64; 2]) {
        self.mouse_x = args[0];
        self.mouse_y = args[1];
    }
}
