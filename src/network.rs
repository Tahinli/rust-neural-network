use crate::{neural_layer::NeuralLayer, neuron::Neuron};

pub struct NeuralNetwork {
    hidden_layers: Vec<NeuralLayer>,
    pub input_layer: NeuralLayer,
    expectation: f64,
    result: f64,
}
impl NeuralNetwork {
    pub fn new() -> Self {
        NeuralNetwork {
            hidden_layers: vec![],
            input_layer: NeuralLayer::new(),
            expectation: 0.0,
            result: 0.0,
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
            for neuron in hidden_layer.into_iter() {
                for input_neuron in input_layer.into_iter() {
                    neuron.activate(input_neuron.transmission());
                }
            }
            input_layer = hidden_layer;
        }
    }

    fn backward(&mut self) {
        let loss = self.expectation - self.result;
        println!("{}", loss);
    }
}
