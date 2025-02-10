use std::sync::LazyLock;

pub mod neuron;

pub static LEARNING_RATE: LazyLock<f64> = LazyLock::new(set_learning_rate);

fn set_learning_rate() -> f64 {
    1.0
}
