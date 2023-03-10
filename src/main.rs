#![deny(clippy::all)]
#![forbid(unsafe_code)]

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

pub const WIDTH: u32 = 320;
pub const HEIGHT: u32 = 240;
// const DEFAULT_SATURATION: u8 = 192;



pub mod world;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 4.0, HEIGHT as f64 * 4.0);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };


    let mut world = world::World::new();
    world.generate_world(false);

    let mut looops: u64 = 0;
    let mut paused: bool = false;
    let mut game_speed: u64 = 16;
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        looops += 1;

        if looops%game_speed==1 && !paused{
            world.update();
        }

        world.draw(pixels.get_frame_mut());

        if let Event::RedrawRequested(_) = event {            
            if let Err(err) = pixels.render() {
                error!("pixels.render() failed: {err}");
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            // Pause and frame skip
            if input.key_pressed(VirtualKeyCode::Space){
                paused = !paused;
            }
            if input.key_pressed(VirtualKeyCode::Right) && paused{
                world.update();
            }
            // Game speed
            if input.key_held(VirtualKeyCode::Down){
                game_speed += 1;
            }

            if input.key_held(VirtualKeyCode::Up){
                if game_speed > 2{
                    game_speed -= 1;
                }
            }


            // World generation
            if input.key_pressed(VirtualKeyCode::R){
                world.generate_world(false);
            }
            if input.key_pressed(VirtualKeyCode::E){
                world.generate_world(true);
            }

            // Drawing
            if input.key_pressed(VirtualKeyCode::A) && paused{
                world.change_cell(input.mouse());
            }


            


            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    error!("pixels.resize_surface() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            // Update internal state and request a redraw
            window.request_redraw();

        }

    });
}