pub use crate::atari::Atari;
use rand;
use rand::Rng;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

pub struct VecAtari {
    envs: Vec<Arc<Mutex<Atari>>>,
    pool: ThreadPool,
    action_space: Vec<i32>,
    last_obs: Vec<Vec<u8>>,
    sender: mpsc::Sender<(usize, Vec<u8>, i32, bool, bool, bool)>,
    receiver: mpsc::Receiver<(usize, Vec<u8>, i32, bool, bool, bool)>,
    fire_reset: bool,
}

impl VecAtari {
    pub fn new(num_envs: usize, game: &str, max_frames: u32, gray_scale: bool, seed: i32) -> Self {
        let pool = ThreadPool::new(num_envs);
        let envs: Vec<Arc<Mutex<Atari>>> = (0..num_envs)
            .map(|i| {
                Arc::new(Mutex::new(Atari::new(
                    game,
                    max_frames,
                    gray_scale,
                    Some(seed + i as i32),
                )))
            })
            .collect();
        let action_space = envs[0].lock().unwrap().get_action_set();
        let last_obs = envs
            .iter()
            .map(|env| {
                let mut env = env.lock().unwrap();
                env.reset();
                env.obs()
                    .chunks(env.screen_dim().1)
                    .step_by(2)
                    .map(|x| x.iter().step_by(2))
                    .flatten()
                    .copied()
                    .collect::<Vec<u8>>()
            })
            .collect();
        let (sender, receiver) = mpsc::channel();
        let fire_reset = action_space.contains(&1);
        Self {
            envs,
            pool,
            action_space,
            last_obs,
            sender,
            receiver,
            fire_reset,
        }
    }

    pub fn step(&mut self, actions: Vec<i32>) -> Vec<(i32, i32, bool, bool)> {
        let fire_reset = self.fire_reset;

        for (i, (env, &action)) in self.envs.iter().zip(&actions).enumerate() {
            let env = env.clone();
            let sender = self.sender.clone();
            self.pool.execute(move || {
                let mut env = env.lock().unwrap();
                let (mut reward, mut terminal, mut truncation, mut life_loss) = (0, false, false, false);
                // let (mut reward, mut terminal, mut truncation, mut life_loss) = env.step(action);

                // for _ in 0..4 {
                //     let (r, t, tr, l) = env.step(action);
                //     reward += r;
                //     terminal = terminal || t;
                //     truncation = truncation || tr;
                //     life_loss = life_loss || l;
                //     if terminal || truncation || life_loss {
                //         break;
                //     }
                // }

                if terminal || truncation {
                    env.reset();
                }

                if terminal || truncation || life_loss {
                    let steps = rand::thread_rng().gen_range(0..30);
                    for _ in 0..steps {
                        env.step(0);
                    }
                    if fire_reset {
                        env.step(1);
                        env.step(2);
                    }
                }
                let obs = env
                    .obs()
                    .chunks(env.screen_dim().1)
                    .step_by(2)
                    .map(|x| x.iter().step_by(2))
                    .flatten()
                    .copied()
                    .collect::<Vec<u8>>();

                sender
                    .send((i, obs, reward, terminal, truncation, life_loss))
                    .unwrap();
            });
        }

        let mut obs_batch: Vec<(i32, i32, bool, bool)> = Vec::new();
        obs_batch.resize(self.envs.len(), (0, 0, false, false));

        for _ in 0..self.envs.len() {
            let (i, next_obs, reward, terminal, truncation, life_loss) =
                self.receiver.recv().unwrap();
            obs_batch[i] = (actions[i], reward, terminal || life_loss, truncation);
            self.last_obs[i] = next_obs;
        }

        obs_batch
    }

    pub fn action_space(&self) -> &Vec<i32> {
        &self.action_space
    }

    pub fn last_obs(&self) -> &Vec<Vec<u8>> {
        &self.last_obs
    }
}
