
use std::{fs, env};
use ale_env::{Atari, BundledRom};
use rand::{thread_rng, Rng};
use image::{save_buffer, ColorType};
fn main () {

    let mut env = Atari::new(
        BundledRom::Breakout,
        108_000,
        false,
        Some(42)
    );
    let n = env.action_dim();
    let mut images = vec![];

    env.reset();
    images.push(env.obs());

    for step in 0..3000 {
        let action = thread_rng().gen_range(0..n);
        let info = env.step(action);
        images.push(env.obs());
        if step == 2999 {
            dbg!(info);
        }
    }
    dbg!(env.get_action_set());

    let (height, width) = env.screen_dim();
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let new_dir = current_dir.join("examples").join("frames");
    fs::create_dir_all(&new_dir).expect("Cannot create directory");
    for (i, x) in images.iter().enumerate() {
        save_buffer(
            new_dir.join(format!("{i:05}.png")), 
            x, 
            width as u32, 
            height as u32, 
            ColorType::Rgb8).expect("Cannot save image");
    }
}