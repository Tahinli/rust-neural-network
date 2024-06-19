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
}

impl<'a> IntoIterator for &'a NeuralLayer {
    type Item = &'a Neuron;

    type IntoIter = std::slice::Iter<'a, Neuron>;

    fn into_iter(self) -> Self::IntoIter {
        self.neural_layer.iter()
    }
}

impl<'a> IntoIterator for &'a mut NeuralLayer {
    type Item = &'a mut Neuron;

    type IntoIter = std::slice::IterMut<'a, Neuron>;

    fn into_iter(self) -> Self::IntoIter {
        self.neural_layer.iter_mut()
    }
}

impl Default for NeuralLayer {
    fn default() -> Self {
        Self::new()
    }
}
