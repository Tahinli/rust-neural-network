pub struct Axon {
    weight: f64,
    bias: f64,
}

impl Axon {
    pub fn new() -> Self {
        Axon {
            weight: 1.0,
            bias: 0.0,
        }
    }

    pub fn get(&self) -> Self {
        Axon {
            weight: self.weight,
            bias: self.bias,
        }
    }

    pub fn set(&mut self, weight: f64, bias: f64) {
        self.weight = weight;
        self.bias = bias;
    }

    pub fn transmit(&self, variable: f64) -> f64 {
        self.weight * variable + self.bias
    }

    pub fn set_weight(&mut self, weight: f64) {
        self.weight = weight;
    }

    pub fn set_bias(&mut self, bias: f64) {
        self.bias = bias;
    }
}
