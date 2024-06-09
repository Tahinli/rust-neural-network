use crate::activation::Activation;

pub struct Neuron {
    calculated_value: f64,
    activation: Activation,
}

impl Neuron {
    pub fn new() -> Self {
        Neuron {
            calculated_value: 0.0,
            activation: Activation::new(),
        }
    }

    pub fn set(&mut self, coefficient_a: f64, variable: f64, coefficient_b: f64) {
        self.activation.set(coefficient_a, variable, coefficient_b);
    }

    pub fn activate(&mut self) -> f64 {
        self.calculated_value = self.activation.calculate();
        self.calculated_value
    }

    fn set_coefficient_a(&mut self, coefficient_a: f64) {
        self.activation.set_coefficient_a(coefficient_a);
    }

    pub fn set_variable(&mut self, variable: f64) {
        self.activation.set_variable(variable);
    }

    fn set_coefficient_b(&mut self, coefficient_b: f64) {
        self.activation.set_coefficient_b(coefficient_b);
    }

    pub fn recalculate_coefficients(&mut self, loss: f64) {
        self.set_coefficient_a(todo!());
        self.set_coefficient_b(todo!())
    }
}
