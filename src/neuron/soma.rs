use super::dendrite::Dendrite;

#[derive(Debug, Default)]
pub struct Soma {
    value: f64,
    bias: f64,
}

impl Soma {
    pub fn consume(&mut self, dendrites: &Vec<Dendrite>) {
        let mut value = 0.0;
        for dendrite in dendrites {
            value += dendrite.get_value();
        }
        value += self.bias;

        self.value = value;
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn set_bias(&mut self, bias: f64) {
        self.bias = bias
    }
}
