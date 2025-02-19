mod atari;
mod bindings;
mod vec_atari;
use pyo3::prelude::*;



#[pymodule]
mod ale_env {
    #[pymodule_export]
    use super::atari::Atari;
    #[pymodule_export]
    use super::vec_atari::VecAtari;
}