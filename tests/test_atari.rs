use ale_env::Atari;
use colored::*;
use rand::seq::SliceRandom;
use core::num;
use rand::rngs::StdRng;
use rand::{thread_rng, Rng, SeedableRng};
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
    let mut images = vec![];
    env.reset();
    let start = Instant::now();
    for _ in 0..steps {
        let action = action_set.choose(&mut rng).expect("Random action failed");
        let (reward, terminal, truncation, life_loss) = env.step(*action);
        total_reward += reward;
        if terminal {
            env.reset();
        }
        images.push(env.obs());
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
fn test_parallel_atari() {
    let num_envs = 16;
    let seed = 42;
    let steps = 10000;
    let mut envs = vec![];
    for i in 0..num_envs {
        let mut env = Atari::new("breakout", 108_000, true, Some(seed + i));
        env.reset();
        envs.push(env);
    }
    let action_set = envs[0].get_action_set();
    let mut total_reward = 0;
    let mut images = vec![];
    let start = Instant::now();
    for _ in 0..steps {
        let data = envs
            .par_iter_mut()
            .map(|env| {
                let action = action_set.choose(&mut thread_rng()).expect("Random action fail");
                let (reward, terminal, truncation, life_loss) = env.step(*action);
                if terminal {
                    env.reset();
                }
                (reward, env.obs())
            })
            .collect::<Vec<(i32, Vec<u8>)>>();
        total_reward += data.iter().map(|x| x.0).sum::<i32>();
        images.extend(data.iter().map(|x| x.1.clone()));
    }
    let duration = start.elapsed();
    println!(
        "{}: time elapsed {:?}, average fps {:.0}, {:.0}",
        "Parallel env:".blue().bold(),
        duration,
        (steps * num_envs) as f32 / duration.as_secs_f32(),
        total_reward
    );
}
