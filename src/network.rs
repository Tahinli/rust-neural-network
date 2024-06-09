use crate::neuron::Neuron;

pub struct NeuralNetwork {
    neurons: Vec<Neuron>,
    pub input: f64,
    expectation: f64,
    result: f64,
}

impl NeuralNetwork {
    pub fn new() -> Self {
        NeuralNetwork {
            neurons: vec![],
            input: 0.0,
            expectation: 0.0,
            result: 0.0,
        }
    }
    pub fn create_neurons(&mut self, neuron_capacity: u128) {
        self.neurons.clear();
        for _ in 0..neuron_capacity {
            self.push_neuron(Neuron::new());
        }
    }
    fn push_neuron(&mut self, neuron: Neuron) {
        self.neurons.push(neuron);
    }

    pub fn run(&mut self) {
        self.forward();
        self.backward();
    }

    fn forward(&mut self) {
        let mut input = self.input;
        for nerve in &mut self.neurons {
            nerve.set_variable(input);
            input = nerve.activate();
        }
        self.result = input;
    }

    fn backward(&mut self) {
        let loss = self.expectation - self.result;
        for nerve in &mut self.neurons {
            nerve.recalculate_coefficients(loss);
        }
    }
}
