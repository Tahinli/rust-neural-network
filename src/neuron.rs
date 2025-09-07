use crate::{axon::Axon, axon_terminal::AxonTerminal};

pub struct Neuron {
    axon: Axon,
    axon_terminal: AxonTerminal,
}

impl Neuron {
    pub fn new() -> Self {
        Neuron {
            axon: Axon::new(),
            axon_terminal: AxonTerminal::new(),
        }
    }

    pub fn set(&mut self, weight: f64, bias: f64) {
        self.axon.set(weight, bias);
    }

    pub fn use_electro_signal(&mut self, variable: f64) -> f64 {
        let calculated_value = self.axon.transmit(variable);
        self.axon_terminal.generate_voltage(calculated_value)
    }

    fn set_weight(&mut self, weight: f64) {
        self.axon.set_weight(weight);
    }

    fn set_bias(&mut self, bias: f64) {
        self.axon.set_bias(bias);
    }

    pub fn recalculate_coefficients(&mut self, loss: f64) {
        self.set_weight(todo!());
        self.set_bias(todo!())
    }
}
