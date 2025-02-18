use ale_env::{Atari, VecAtari};
use colored::*;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{thread_rng, SeedableRng};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use std::time::Instant;

#[test]
fn test_atari() {
    let seed = 42;
    let steps = 10000;
    let mut env = Atari::new("breakout", 100, true, Some(seed));
    let mut rng = StdRng::seed_from_u64(seed as u64);

    let action_set = env.get_action_set();
    let mut total_reward = 0;
    env.reset();
    let start = Instant::now();
    for _ in 0..steps {
        let action = action_set.choose(&mut rng).expect("Random action failed");
        let (reward, terminal, truncation, life_loss) = env.step(*action);
        total_reward += reward;
        if terminal {
            env.reset();
        }
    }
    let duration = start.elapsed();
    println!(
        "{}: time elapsed {:?}, average fps {:.0}, {:.0}",
        "Single env:".blue().bold(),
        duration,
        steps as f32 / duration.as_secs_f32(),
        total_reward
    );
    env.close();
}

#[test]
fn test_vec_atari() {
    let num_envs = 16;
    let seed = 42;
    let steps = 10000;
    let mut envs = VecAtari::new(num_envs, "breakout", 108_000, true, seed);
    let now = Instant::now();
    let mut result = vec![];
    result.push(envs.reset());
    for _ in 0..steps {
        let actions = (0..num_envs)
            .map(|_| *envs.action_space().choose(&mut rand::thread_rng()).unwrap())
            .collect();
        result.push(envs.step(actions));
    }
    let duration = now.elapsed();
    println!(
        "{}: time elapsed {:?}, average fps {:.0}",
        "Vec Atari".blue().bold(),
        duration,
        (steps * num_envs) as f32 / duration.as_secs_f32()
    );
}
