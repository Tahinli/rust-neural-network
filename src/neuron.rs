use crate::{axon::Axon, axon_terminal::AxonTerminal};

#[derive(Debug)]
pub struct Neuron {
    axon: Axon,
    axon_terminal: AxonTerminal,
    signal: f64,
}

impl Neuron {
    pub fn new() -> Self {
        Neuron {
            axon: Axon::new(),
            axon_terminal: AxonTerminal::new(),
            signal: 0.0,
        }
    }

    pub fn set(&mut self, weight: f64, bias: f64) {
        self.axon.set(weight, bias);
    }

    pub fn use_electro_signal(&mut self, input: f64) {
        let calculated_value = self.axon.transmit(input);
        self.signal = self.axon_terminal.generate_voltage(calculated_value);
    }

    pub fn collect_electro_signal(&mut self, input: f64) {
        let calculated_value = self.axon.transmit(input);
        self.signal += self.axon_terminal.generate_voltage(calculated_value);
    }

    fn set_weight(&mut self, weight: f64) {
        self.axon.set_weight(weight);
    }

    fn set_bias(&mut self, bias: f64) {
        self.axon.set_bias(bias);
    }

    pub fn get_signal(&self) -> f64 {
        self.signal
    }

    pub fn recalculate_coefficients(&mut self, loss: f64) {
        self.set_weight(todo!());
        self.set_bias(todo!())
    }
}
