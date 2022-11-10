use std::{fs::{read_to_string, remove_file}, env};

use ruscii::{app::{App, State}, terminal::Window, drawing::Pencil, spatial::Vec2, keyboard::{KeyEvent, Key}};
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Provide 1 file to shred!");
        return;
    }

    let file = &args[1];

    let mut app = App::new();

    let mut rng = rand::thread_rng();
    let data = read_to_string(file).unwrap();
    let mut lines : Vec<Vec<(char,u32)>>= data.lines().map(|x| x.chars().map(|c| (c, rng.gen::<u32>() % 100)).collect()).collect();

    let mut tick = 0;
    
    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }

        for line in lines.iter_mut() {
            for (char, life) in line.iter_mut() {
                if tick > *life {
                    *char = ' ';
                }
            }
        }



        let mut pencil = Pencil::new(window.canvas_mut());
        for (i, line) in lines.iter().enumerate() {
            let str : String = line.iter().map(|x| x.0).collect();
            pencil.draw_text(&str, Vec2::xy(0,i));
        }

        tick += 1;

        if tick > 100 {
            app_state.stop();
        }
    });

    remove_file(file).unwrap();
    println!("file deleted");
}
