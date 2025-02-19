pub use crate::atari::Atari;
use pyo3::prelude::*;
use rand;
use rand::Rng;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

#[pyclass]
pub struct VecAtari {
    envs: Vec<Arc<Mutex<Atari>>>,
    pool: ThreadPool,
    action_space: Vec<i32>,
    sender: mpsc::Sender<(usize, Vec<u8>, i32, bool, bool, bool, Option<i32>)>,
    receiver: Arc<Mutex<mpsc::Receiver<(usize, Vec<u8>, i32, bool, bool, bool, Option<i32>)>>>,
}

#[pymethods]
impl VecAtari {
    #[new]
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
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        Self {
            envs,
            pool,
            action_space,
            sender,
            receiver,
        }
    }

    pub fn step(
        &mut self,
        actions: Vec<i32>,
    ) -> Vec<(usize, Vec<u8>, i32, bool, bool, bool, Option<i32>)> {
        for (i, (env, &action)) in self.envs.iter().zip(&actions).enumerate() {
            let env = env.clone();
            let sender = self.sender.clone();
            self.pool.execute(move || {
                let mut env = env.lock().unwrap();
                let (reward, terminal, truncation, life_loss) = env.step(action);
                let score = match terminal || truncation {
                    true => {
                        let score = env.get_score();
                        env.reset();
                        Some(score)
                    }
                    false => None,
                };

                let obs = env.obs();
                sender
                    .send((i, obs, reward, terminal, truncation, life_loss, score))
                    .unwrap();
            });
        }

        let receiver = self.receiver.lock().unwrap();
        let mut result: Vec<(usize, Vec<u8>, i32, bool, bool, bool, Option<i32>)> =
            (0..self.envs.len())
                .map(|_| receiver.recv().unwrap())
                .collect();
        result.sort_by_key(|x| x.0);
        result
    }

    pub fn reset(&mut self) -> Vec<(usize, Vec<u8>, i32, bool, bool, bool, Option<i32>)> {
        for (i, env) in self.envs.iter().enumerate() {
            let mut env = env.lock().unwrap();
            env.reset();
            self.sender
                .send((i, env.obs(), 0, false, false, false, None))
                .unwrap();
        }

        let receiver = self.receiver.lock().unwrap();
        let mut result: Vec<(usize, Vec<u8>, i32, bool, bool, bool, Option<i32>)> =
            (0..self.envs.len())
                .map(|_| receiver.recv().unwrap())
                .collect();
        result.sort_by_key(|x| x.0);
        result
    }

    pub fn action_space(&self) -> &Vec<i32> {
        &self.action_space
    }
}

impl Drop for VecAtari {
    fn drop(&mut self) {
        for env in self.envs.iter() {
            let mut env = env.lock().unwrap();
            env.close();
        }
        self.pool.join();
    }
}
