#[derive(Debug, Default)]
pub struct Dendrite {
    value: f64,
    weight: f64,
}

impl Dendrite {
    pub fn consume(&mut self, input: &f64) {
        self.value = input * self.weight;
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}
