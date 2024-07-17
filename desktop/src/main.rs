use chip8_core::*;
use std::env;
use sdl2::event::Event;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

fn main() {
    // _ means that type of Vector is not sure
    // it depends on the type of the arguments
    let args: Vec<_> = env::args().collect();
    
    if args.len() != 2 { 
        println!("Usage: cargo run path/to/game");
        return; 
    }

    // Start the SDL2 context. This is a handle to the library's functionality.
    let sdl_context = sdl2::init().unwrap();

    // Get the Video subsystem from the SDL2 context. 
    // This subsystem allows you to manage the video feature.
    let video_subsystem = sdl_context.video().unwrap();

    // Create a window with the title "Chip-8 Emulator". 
    // The window will have a width as WINDOW_WIDTH and height as WINDOW_HEIGHT. 
    // Set the position of the window to centered, and use opengl as the window's backend.
    let window = video_subsystem
        .window("Chip-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    // Turn our window into a 2D rendering context. 
    // `present_vsync()` enables vsync, which aims to limit the framerate to the screen's refresh rate.
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .unwrap();

    // Clear the current rendering context with the drawing color.
    canvas.clear();

    // Make the rendered content visible to the user.
    canvas.present();


    // SDL provides this method to poll for events every loop.
    // Should use poll_iter to get all available events
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    
    // ‘gameloop is a loop label， it can let us easy to break the specific loop
    'gameloop: loop {
        
        // Iterate over all available events, processing each one.
        for evt in event_pump.poll_iter() {
            // Use a match expression to handle different types of events.
            match evt {
                Event::Quit {..} => {
                    // In the here, we can break specific loop by loop label 'gameloop
                    break 'gameloop;
                },
                // For all other types of events, we don't do anything and just continue looping.
                _ => ()
            }
        }
    }
}
