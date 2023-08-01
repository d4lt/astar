extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::{MouseButton, MouseCursorEvent, WindowSettings};

use graphics::grid::Grid;
use graphics::{color::*, grid, line, rectangle, types::Color, Context};
use graphics::{draw_state, Graphics, Transformed};

mod grid_set;

use grid_set::*;

const BG_COLOR: Color = [0.094, 0.094, 0.094, 1.0];
const GREY: Color = [0.7, 0.7, 0.7, 1.0];

pub enum AppState {
    Setup,
    Playing,
    Paused,
    Finish,
}

pub struct Astar {
    pub gl: GlGraphics,
    pub state: AppState,
    grid: Grid,
    grid_set: Vec<Node>,
}

impl Astar {
    fn new(gl: GlGraphics, grid: Grid) -> Self {
        let total_nodes = grid.rows * grid.cols;
        let mut grid_set: Vec<Node> = Vec::with_capacity(total_nodes as usize);

        for _ in 0..total_nodes {
            let new_node = Node { state: NodeState::Blank };
            grid_set.push(new_node);
        }

        Astar {
            gl,
            state: AppState::Setup,
            grid,
            grid_set,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        // let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        // let (win_x, win_y) = (args.window_size[0], args.window_size[1]);

        let grid_line = line::Line::new(WHITE, 0.5);

        self.gl.draw(args.viewport(), |c, gl| {
            gl.clear_color(BG_COLOR);

            self.grid.draw(&grid_line, &c.draw_state, c.transform, gl);

            for node in self.grid.cells() {
                let (col, row) = node;
                let (x, y) = (
                    (col as f64) * self.grid.units,
                    (row as f64) * self.grid.units,
                );
                let node_set = self.grid_set[(col + row) as usize];
                println!("{:?}", node_set);

                let node_color = match node_set.state {
                    NodeState::Start => BLUE,
                    NodeState::End => YELLOW,
                    NodeState::Blank => BG_COLOR,
                    NodeState::Obstacle => SILVER,
                    NodeState::Path => PURPLE,
                    NodeState::Open => GREEN,
                    NodeState::Closed => RED,
                };

                rectangle(
                    node_color,
                    [self.grid.units - grid_line.radius; 4],
                    c.transform.trans(x, y),
                    gl,
                );
            }
            rectangle(RED, [self.grid.units; 4], c.transform.trans(0.0, 0.0), gl);
        });
    }

    fn toggle_node(&mut self, mouse_pos: [f64; 2]) {
        let (row, col) = (
            (mouse_pos[1] / self.grid.units) as u32,
            (mouse_pos[0] / self.grid.units) as u32,
        );
        let mut node: &mut Node = &mut self.grid_set[(row + col) as usize];
        node.state = NodeState::Obstacle;
    }
}

fn main() {
    let opengl = OpenGL::V3_3;

    let grid_size: u32 = 40;
    let node_size: f64 = 20.0;

    let window_size = grid_size * node_size as u32;

    let mut window: GlutinWindow = WindowSettings::new("a-star", [window_size, window_size])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let grid = grid::Grid {
        cols: grid_size,
        rows: grid_size,
        units: node_size,
    };

    let mut astar = Astar::new(GlGraphics::new(opengl), grid);

    let mut mouse_pos: [f64; 2] = [0.0, 0.0];
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            astar.render(&args);
        }

        if let Some(pos) = e.mouse_cursor_args() {
            let mp = &mut mouse_pos;
            *mp = pos;
        }

        if let Some(Button::Mouse(button)) = e.press_args() {
            match button {
                MouseButton::Left => astar.toggle_node(mouse_pos),
                _ => (),
            }
        }

        // if let Some(args) = e.update_args() {
        //
        // }
    }
}
