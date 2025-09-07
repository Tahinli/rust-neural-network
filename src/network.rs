use crate::neuron::Neuron;
struct NeuralLine {
    neural_line: Vec<Neuron>,
}

impl NeuralLine {
    fn new() -> Self {
        NeuralLine {
            neural_line: vec![],
        }
    }

    fn push_neuron(&mut self, neuron: Neuron) {
        self.neural_line.push(neuron);
    }
}

pub struct NeuralNetwork {
    neurons: Vec<NeuralLine>,
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
    pub fn create_neurons(&mut self, neuron_capacity: u128, neuron_line_capacity: u128) {
        self.neurons.clear();
        for _ in 0..neuron_line_capacity {
            let mut neural_line = NeuralLine::new();
            for _ in 0..neuron_capacity {
                neural_line.push_neuron(Neuron::new());
            }
            self.push_neuron_line(neural_line);
        }
    }
    fn push_neuron_line(&mut self, neuron_line: NeuralLine) {
        self.neurons.push(neuron_line);
    }

    pub fn run(&mut self) {
        self.forward();
        self.backward();
    }

    fn forward(&mut self) {
        let mut input = self.input;
        todo!()
    }

    fn backward(&mut self) {
        let loss = self.expectation - self.result;
        todo!()
    }
}
