use ale_env::Atari;
use image::{save_buffer, ColorType};
use rand::rngs::StdRng;
use rand::{thread_rng, Rng, SeedableRng};
use std::time::Instant;
use std::{env, fs};

fn main() {
    let seed = 42;
    let steps = 10000;

    let mut env = Atari::new("breakout", 108_000, false, Some(seed));
    let n = env.get_action_set().len();
    let mut images = vec![];
    env.reset();
    images.push(env.obs());
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let mut total_reward = 0;

    let start = Instant::now();
    for _ in 0..steps {
        let (reward, terminal, truncation, life_loss) = env.step(rng.gen_range(0..n));
        images.push(env.obs());
        if terminal {
            env.reset();
        }
        total_reward += reward;
    }

    let duration = start.elapsed();
    println!(
        "Time elapsed: {:?}, FPS: {:.0}",
        duration,
        steps as f32 / duration.as_secs_f32()
    );
    println!("action set:{:?}", env.get_action_set());
    println!("Total reward: {}", total_reward);

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
            ColorType::Rgb8,
        )
        .expect("Cannot save image");
    }
}
