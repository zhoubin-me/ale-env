import rs_ale
import time
import random
import numpy as np
steps = 20000



env = rs_ale.Atari("breakout", 108_000, True, 42)
action_set = env.get_action_set()
env.reset()
print(action_set)
now = time.time()
for step in range(steps):
    obs = env.obs()
    action = random.choice(action_set)
    rew, terminal, trunc, life_loss = env.step(action)
    if terminal or trunc:
        env.reset()
print(time.time() - now)
steps = steps * 1  # total steps (num_iterations * num_envs)
elapsed = time.time() - now
fps = steps / elapsed
print(f"FPS: {fps:.2f}")
print("=" * 100)


