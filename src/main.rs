extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread::sleep;
use std::time::Duration;
use sdl2::render::{TextureCreator, Texture};
use sdl2::rect::Rect;

const TEXTURE_SIZE: u32 = 32;

fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context
        .video()
        .expect("could not get video subsystem");
    let window = video_subsystem
        .window("rust-sdl2 demo video", 800, 600)
        // position_centered gets the window in the middle of the screen
        .position_centered()
        // opengl makes the SDL use opengl to render
        .opengl()
        // build creates a window by applying all previously received parameters
        .build()
        // expect panics with the given message if an error occurred
        .expect("failed to create window");

    let mut canvas = window
        // into_canvas transforms the window into a canvas so that we can manipulate it more easily
        .into_canvas()
        // target_texture actives texture rendering support
        .target_texture()
        // present_vsync enables the v-sync ( also known as vertical synchronization ) limit
        .present_vsync()
        // build creates the canvas by applying all previously set parameters
        .build()
        .expect("failed to convert window to canvas");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let mut square_texture: Texture = texture_creator
        .create_texture_target(None, TEXTURE_SIZE, TEXTURE_SIZE)
        .expect("failed to create texture");

    canvas
        .with_texture_canvas(&mut square_texture, |texture| {
            // set_draw_color sets the color to be used when drawing occurs
            texture.set_draw_color(Color::RGB(0, 255, 0));
            // clear washes/clears the texture so it will be filled with green
            texture.clear();
        })
        .expect("failed to color a texture");

    let mut event_pump = sdl_context.
        event_pump()
        .expect("failed to get sdl event pump");

    // We added a label `running` to the main loop.
    // The point is to able to break directly an upper loop without having to set a variable
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                    { break 'running; }
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();

        canvas
            .copy(
                &square_texture,
                None,
                Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE),
            )
            .expect("could not copy texture into window");
        canvas.present();

        // we sleep enough to get ~60fps. If we don't call this, the program will take
        // 100% of a CPU time
        sleep(Duration::new(0, 1_000_000_000u32 / 60))
    }
}
