use ale_env::{Atari, BundledRom};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

#[test]
fn test_atari() {
    let seed = 42;
    let mut env = Atari::new(BundledRom::Breakout, 1999, true, Some(seed));
    let mut acc_stat = (0, false, false, false);
    let mut rng = StdRng::seed_from_u64(seed as u64);

    let n = env.action_dim();
    env.reset();
    for _ in 0..2000 {
        let stat = env.step(rng.gen_range(0..n));
        acc_stat = (acc_stat.0 + stat.0, acc_stat.1 | stat.1, acc_stat.2 | stat.2, acc_stat.3 | stat.3);
    }
    
    assert!(acc_stat.0 > 0, "no reward from random policy");
    assert!(acc_stat.1, "no terminal");
    assert!(acc_stat.2, "no truncation");
    assert!(acc_stat.3, "no life loss");

    env.close();
}
