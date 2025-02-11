use super::soma::Soma;

#[derive(Debug, Default)]
pub struct Axon {
    treshold: f64,
}

impl Axon {
    pub fn activate(&self, soma: &Soma) -> f64 {
        // ReLU
        soma.get_value().max(0.0)
    }

    pub fn set_treshold(&mut self, treshold: f64) {
        self.treshold = treshold
    }
}
