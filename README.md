# Atari Learning Environment

A rust wrapper of [Arcade Learning Environment](https://github.com/Farama-Foundation/Arcade-Learning-Environment).


# Build and run
Clone this repo:
```
git clone https://github.com/zhoubin-me/ale-env.git --recursive
cd ale-env
```

Download roms first:
```bash
bash scripts/download_roms.sh
```

Run breakout examples under ```examples/screenshots```, and generate video from frames:
```bash
cargo run --release --example screenshots
bash scripts/convert_frames_to_video.sh
```

Run test for FPS benchmark
```bash
cargo test -- --nocapture
```





