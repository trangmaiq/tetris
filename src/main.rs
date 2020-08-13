extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use sdl2::render::{TextureCreator, Texture, Canvas};
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowContext};
use sdl2::image::{INIT_PNG, INIT_JPG, LoadTexture};
use std::io;
use std::fs::File;
use std::io::{Write, Read};
use sdl2::version::version;

mod tetrimino;

use tetrimino::*;

const TEXTURE_SIZE: u32 = 32;

enum TextureColor {
    BlueR60G136B207,
    GreenR73G196B137,
}

// Tetris will hold all the game's information:
// - Game map
// - Current level
// - Score
// - Number of lines
// - The current tetrimino
// - Some potential other information ( such as a ghost, or the preview of the next tetrimino )
struct Tetris {
    game_map: Vec<Vec<u8>>,
    current_level: u32,
    score: u32,
    nb_lines: u32,
    current_piece: Option<Tetrimino>,
}

impl Tetris {
    fn new() -> Tetris {
        let mut game_map = Vec::new();
        // We know that a `tetris` map has a width of 10 blocks and a height of 16 blocks.
        // This loop create our game map by looping over the number of lines
        // and generating an empty vector of 10 blocks, which will be a line.
        for _ in 0..16 {
            game_map.push(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }
        // Apart of game map, everything else is very straightforward
        Tetris {
            game_map,
            current_level: 1,
            score: 0,
            nb_lines: 0,
            current_piece: None,
        }
    }

fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context
        .video()
        .expect("could not get video subsystem");

    sdl2::image::init(INIT_PNG | INIT_JPG)
        .expect("could not initialize image context");

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
    let blue_square_texture: Texture = create_square_texture_rect(
        &mut canvas,
        &texture_creator,
        TextureColor::BlueR60G136B207,
        TEXTURE_SIZE).expect("failed to create blue square texture");
    let green_square_texture: Texture = create_square_texture_rect(
        &mut canvas,
        &texture_creator,
        TextureColor::GreenR73G196B137,
        TEXTURE_SIZE).expect("failed to create green square texture");

    let image_texture = texture_creator
        .load_texture("assets/rust_does_not_compile.png").expect("could not load image");

    let timer = SystemTime::now();

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
        canvas.set_draw_color(Color::RGB(156, 40, 8));
        canvas.clear();

        canvas.copy(&image_texture, None, None).expect("render failed");

        // let display_green = match timer.elapsed() {
        //     Ok(elapsed) => elapsed.as_secs() % 2 == 0,
        //     Err(_) => {
        //         // in case of error, currently, we do nothing
        //         true
        //     }
        // };
        //
        // let square_texture = if display_green {
        //     &green_square_texture
        // } else {
        //     &blue_square_texture
        // };
        //
        // canvas
        //     .copy(
        //         &square_texture,
        //         None,
        //         Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE),
        //     )
        //     .expect("could not copy texture into window");
        canvas.present();

        // we sleep enough to get ~60fps. If we don't call this, the program will take
        // 100% of a CPU time
        sleep(Duration::new(0, 1_000_000_000u32 / 60))
    }
}

fn create_square_texture_rect<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    color: TextureColor,
    size: u32,
) -> Option<Texture<'a>> {
    if let Ok(mut square_texture) = texture_creator
        .create_texture_target(None, size, size) {
        canvas.with_texture_canvas(&mut square_texture, |texture| {
            match color {
                TextureColor::BlueR60G136B207 =>
                    texture.set_draw_color(Color::RGB(60, 136, 207)),
                TextureColor::GreenR73G196B137 =>
                    texture.set_draw_color(Color::RGB(73, 196, 137))
            }
            texture.clear()
        }).expect("failed to color a texture");
        Some(square_texture)
    } else { None }
}

fn slice_to_string(slice: &[u32]) -> String {
    slice
        // Here we create an iterator from our slice
        // A really important and fundamental thing to note about iterators is in Rust;
        // they're lazy.
        // Create an iterator doesn't cost anything more than the size of type.
        // (generally a structure containing a pointer and an index).
        // Until the next() method is called, nothing happens.
        .iter()
        // We call the iterator's map method.
        // What it does is simple: it converts the current type into another one.
        // Really important to note: at this point, the iterator still hasn't done anything/
        // Keep in mid that nothing is done util the next() method is called.
        .map(|highscore| highscore.to_string())
        // And now we call the collect() method.
        // It'll call the next() method of our iterator as long as
        // it didn't get all elements and store them into a Vec.
        // This is where map() method will be called on every element of out iterator
        .collect::<Vec<String>>()
        // And finally the last step: This method (as its name indicates) joins all the elements
        // of the Vec into a String separated by the given &str (so, " " in our case).
        .join(" ")
}

fn save_highscores_and_lines(highscores: &[u32], number_of_lines: &[u32]) -> bool {
    let s_highscores = slice_to_string(highscores);
    let s_number_of_lines = slice_to_string(number_of_lines);

    // The is_ok() method call just informs the caller of the save_highscores_and_lines() function
    // if everything has been saved as expected or not.
    write_into_file(format!("{}\n{}\n", s_highscores,
                            s_number_of_lines).as_ref(), "scores.txt").is_ok()
}

fn write_into_file(content: &str, filename: &str) -> io::Result<()> {
    // try! marco can be replaced with ? operator
    let mut f = File::create(filename)?;
    f.write_all(content.as_bytes())
}

fn line_to_slice(line: &str) -> Vec<u32> {
    line.split(" ").filter_map(|nb| nb.parse::<u32>().ok()).collect()
}

fn load_highscores_and_lines() -> Option<(Vec<u32>, Vec<u32>)> {
    if let Ok(content) = read_from_file("scores.txt") {
        let mut lines = content.splitn(2, "\n").map(|line|
            line_to_slice(line)).collect::<Vec<_>>();
        if lines.len() == 2 {
            let (number_lines, highscores) = (lines.pop().unwrap(),
                                              lines.pop().unwrap());
            Some((highscores, number_lines))
        } else {
            None
        }
    } else {
        None
    }
}

// This time, it only tales a filename as an argument
// and returns a String if the reading was successful.
fn read_from_file(filename: &str) -> io::Result<String> {
    let mut f = File::open(filename)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}
