use super::soma::Soma;

#[derive(Debug, Default)]
pub struct Axon {
    treshold: f64,
}

impl Axon {
    pub fn activate(&self, soma: &Soma) -> f64 {
        // ReLU
        let soma_value = soma.get_value();
        if soma_value > 0.0 {
            if soma_value > self.treshold {
                soma_value
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    pub fn set_treshold(&mut self, treshold: f64) {
        self.treshold = treshold
    }
}
