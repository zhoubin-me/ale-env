
use ale_env::Atari;
use rand::rngs::StdRng;
use rand::{thread_rng, Rng, SeedableRng};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use std::time::Instant;
use colored::*;

#[test]
fn test_atari() {
    let seed = 42;
    let steps = 10000;
    let mut env = Atari::new("breakout", 100, true, Some(seed));
    let mut acc_stat = (0, false, false, false);
    let mut rng = StdRng::seed_from_u64(seed as u64);

    let n = env.get_action_set().len();
    env.reset();

    let start = Instant::now();
    for _ in 0..100000 {
        let stat = env.step(rng.gen_range(0..n));
        acc_stat = (acc_stat.0 + stat.0, acc_stat.1 | stat.1, acc_stat.2 | stat.2, acc_stat.3 | stat.3);
        if stat.1 {
            env.reset();
        }
    }
    let duration = start.elapsed();
    println!("{}: {:?}, {}: {:.0}",
        "Parallel env, Time elapsed".blue().bold(), 
        duration, 
        "FPS".blue().bold(), 
        steps as f32 / duration.as_secs_f32());
    
    assert!(acc_stat.0 > 0, "no reward from random policy");
    assert!(acc_stat.1, "no terminal");
    assert!(acc_stat.2, "no truncation");
    assert!(acc_stat.3, "no life loss");

    env.close();
}


#[test]
fn test_parallel_atari() {
    let num_envs = 16;
    let seed = 42;
    let steps = 10000;
    let mut envs = vec![];
    for i in 0..num_envs {
        let mut env = Atari::new("breakout", 108_000, true, Some(seed+i));
        env.reset();
        envs.push(env);
    }
    let action_dim = envs[0].get_action_set().len();

    let start = Instant::now();
    for _ in 0..steps {
        envs.par_iter_mut().for_each(|env| {
            let action = thread_rng().gen_range(0..action_dim);
            let (reward, terminal, truncation, life_loss) = env.step(action);
            if terminal {
                env.reset();
            }
        });
    }
    let duration = start.elapsed();
    println!("{}: {:?}, {}: {:.0}",
        "Parallel env, Time elapsed".blue().bold(), 
        duration, 
        "FPS".blue().bold(), 
        (steps * num_envs) as f32 / duration.as_secs_f32());

}
