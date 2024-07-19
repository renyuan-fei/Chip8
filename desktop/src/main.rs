use chip8_core::*;
use std::env;
use std::fs::File;
use std::io::Read;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;
const TICKS_PER_FRAME: usize = 10;

fn draw_screen(emu: &Emu, canvas: &mut Canvas<Window>)
{
    // Clear canvas as black
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buf = emu.get_display();

    // Now set draw color to white, iterate through each point and see if it should be drawn
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    for (i, pixel) in screen_buf.iter().enumerate() {
        if *pixel {
            // Convert our 1D array's index into a 2D (x,y) position 
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;

            // Draw a rectangle at (x,y), scaled up by our SCALE value
            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}

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
    
    let mut chip8 = Emu::new();
    
    // read data from file and load into Emu
    let mut rom = File::open(&args[1]).expect("Unable to open file");
    let mut buffer  = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();
    chip8.load(&buffer);
    
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

        for _ in 0..TICKS_PER_FRAME {
            chip8.tick();
        }
        chip8.tick_timers();
        draw_screen(&chip8, &mut canvas);
    }
}
