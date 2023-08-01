extern crate piston;
extern crate graphics; 
extern crate glutin_window;
extern crate opengl_graphics;

use opengl_graphics::{ GlGraphics, OpenGL, TextureSettings, Filter, GlyphCache };
use piston::{WindowSettings, MouseCursorEvent, MouseButton,} ;
use piston::event_loop::{Events, EventSettings};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, 
    UpdateEvent, PressEvent, Button};
use glutin_window::GlutinWindow;

use graphics::grid::Grid;
use graphics::{Graphics, Transformed, draw_state};
use graphics::{line , grid, types::Color, rectangle, Context, color::*};

mod grid_set;

use grid_set::*;

const BG_COLOR: Color = [0.094, 0.094, 0.094, 1.0];
const GREY: Color = [0.7, 0.7, 0.7, 1.0];


pub enum AppState {
    Setup,
    Playing,
    Paused,
    Finish
}


pub struct Astar {
    pub gl: GlGraphics,
    pub state: AppState,
    grid: Grid,
    grid_set: Vec<Node>
}

impl Astar {
    fn new(gl: GlGraphics, grid: Grid) -> Self {
        let mut grid_set: Vec<Node> = Vec::with_capacity( (grid.rows * grid.cols) as usize);
        grid_set.iter_mut().for_each(|mut node| {
            node.state = NodeState::Blank;
        });
        
        Astar { 
            gl, 
            state: AppState::Setup,
            grid,
            grid_set 
        }  
    }

    fn render(&mut self, args: &RenderArgs) {
        
        // let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        // let (win_x, win_y) = (args.window_size[0], args.window_size[1]);

        let grid_line = line::Line::new(WHITE, 0.5);

            self.gl.draw(args.viewport(), |c, gl| {
                gl.clear_color(BG_COLOR);

                self.grid.draw(&grid_line, &c.draw_state, c.transform, gl);

                for node in self.grid.cells(){
                    let (col, row) = node;                 
                    let (x, y) = ((col as f64)*self.grid.units, (row as f64)*self.grid.units);

                    let node_color = match self.grid_set[(col + row) as usize].state {
                       grid_set::NodeState::Start => BLUE, 
                       grid_set::NodeState::End => YELLOW, 
                       grid_set::NodeState::Blank => BG_COLOR, 
                       grid_set::NodeState::Obstacle => SILVER, 
                       grid_set::NodeState::Path => PURPLE, 
                       grid_set::NodeState::Open => GREEN, 
                       grid_set::NodeState::Closed => RED, 

                    };
                    
                    rectangle(node_color, [self.grid.units-grid_line.radius ; 4], c.transform.trans(x, y), gl);
                };
            });
    }

    fn toggle_node(&mut self, mouse_pos: [f64; 2] ) {
        let (node_row, node_col) = ((mouse_pos[1]/self.grid.units) as u32, (mouse_pos[0]/self.grid.units) as u32 );

        let node_pos = self.grid.cell_position( (node_col,node_row) );
        println!("Mouse clicked: {:?}", node_pos);
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
        units: node_size 
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

        if let Some( Button::Mouse(button) ) = e.press_args(){
            match button {
                MouseButton::Left => astar.toggle_node(mouse_pos),
                _ => ()
            }
        }

        // if let Some(args) = e.update_args() {
        //     
        // }
    }

}
