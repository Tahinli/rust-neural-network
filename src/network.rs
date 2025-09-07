use crate::{neural_layer::NeuralLayer, neuron::Neuron};

pub struct NeuralNetwork {
    pub input_layer: NeuralLayer,
    hidden_layers: Vec<NeuralLayer>,
    pub output_layer: NeuralLayer,
    expectation: Vec<f64>,
    result: Vec<f64>,
}
impl NeuralNetwork {
    pub fn new() -> Self {
        NeuralNetwork {
            input_layer: NeuralLayer::new(),
            hidden_layers: vec![],
            output_layer: NeuralLayer::new(),
            expectation: vec![],
            result: vec![],
        }
    }
    pub fn create_neurons(&mut self, neuron_capacity: u128, neural_layer_capacity: u128) {
        self.hidden_layers.clear();
        for _ in 0..neural_layer_capacity {
            let mut neural_layer = NeuralLayer::new();
            for _ in 0..neuron_capacity {
                neural_layer.push_neuron(Neuron::new());
            }
            self.push_neural_layer(neural_layer);
        }
    }
    fn push_neural_layer(&mut self, neural_layer: NeuralLayer) {
        self.hidden_layers.push(neural_layer);
    }

    pub fn run(&mut self) {
        self.forward();
        self.backward();
    }

    fn forward(&mut self) {
        let mut input_layer = &mut self.input_layer;
        for hidden_layer in self.hidden_layers.iter_mut() {
            for (index, neuron) in hidden_layer.into_iter().enumerate() {
                let mut dendrites = vec![];
                for input_neuron in input_layer.into_iter() {
                    dendrites.push(input_neuron.borrow_dendrite(index))
                }
                neuron.activate(&mut dendrites);
            }
            input_layer = hidden_layer;
        }
    }

    fn backward(&mut self) {
        let mut losses = vec![];
        for (result, expectation) in self.result.iter().zip(&self.expectation) {
            let loss = (result - expectation).powi(2);
            losses.push(loss);
        }
        todo!();
    }
}

impl Default for NeuralNetwork {
    fn default() -> Self {
        Self::new()
    }
}
