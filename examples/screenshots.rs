
use std::{fs, env};
use ale_env::{Atari, BundledRom};
use rand::{thread_rng, Rng};
use image::{save_buffer, ColorType};
use std::time::Instant;

fn main () {

    let mut env = Atari::new(
        BundledRom::Breakout,
        108_000,
        false,
        Some(42)
    );
    let n = env.action_dim();
    let steps = 10000;
    let mut images = vec![];
    env.reset();
    images.push(env.obs());
    

    let start = Instant::now();
    for _ in 0..steps {
        let action = thread_rng().gen_range(0..n);
        env.step(action);
        images.push(env.obs());
    }
    let duration = start.elapsed();
    println!("FPS: {:.0}", steps as f32 / duration.as_secs_f32());
    println!("action set:{:?}", env.get_action_set());

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