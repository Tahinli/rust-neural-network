use crate::neuron::Neuron;

pub struct NeuralLayer {
    neural_layer: Vec<Neuron>,
}

impl NeuralLayer {
    pub fn new() -> Self {
        NeuralLayer {
            neural_layer: vec![],
        }
    }

    pub fn push_neuron(&mut self, neuron: Neuron) {
        self.neural_layer.push(neuron);
    }

    pub fn iter(&self) -> std::slice::Iter<Neuron> {
        self.neural_layer.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<Neuron> {
        self.neural_layer.iter_mut()
    }
}