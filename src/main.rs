mod grid;
mod cell;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;
use crate::grid::Grid;


const WIDTH: u32 = 400;
const HEIGHT: u32 = 250;

const CELL_SIZE: f64 = 2.0;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * CELL_SIZE, HEIGHT as f64 * CELL_SIZE);
        WindowBuilder::new()
            .with_title("Hedgehog")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut screen = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut grid = Grid::new(WIDTH as usize, HEIGHT as usize);

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            grid.update_grid();
            grid.draw_grid(screen.frame_mut());
            if let Err(_err) = screen.render() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                if let Err(_err) = screen.resize_surface(size.width, size.height) {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
        }

        window.request_redraw();
    });

}
