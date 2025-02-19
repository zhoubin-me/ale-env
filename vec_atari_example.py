import rs_ale
import time
import random
import numpy as np
steps = 20000

now = time.time()
envs = rs_ale.VecAtari(16, "breakout", 108000, True, 42)
action_space = envs.action_space()
print(action_space)
result = envs.reset()
_, obs, _, _, _, _, _ = zip(*result)
rewards = 0
for step in range(steps):
    actions = [random.choice(action_space) for _ in range(32)]
    result = envs.step(actions)
    i, obs, reward, terminal, truncation, life_loss, score = zip(*result)
    rewards += np.array(reward)

print(rewards)
print(time.time() - now)
steps = steps * 32
elapsed = time.time() - now
fps = steps / elapsed
print(f"FPS: {fps:.2f}")