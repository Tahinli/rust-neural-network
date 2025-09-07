mod axon;
mod dendrite;
mod soma;

use axon::Axon;
use dendrite::Dendrite;
use soma::Soma;

#[derive(Debug, Default)]
pub struct Neuron {
    dendtrites: Vec<Dendrite>,
    soma: Soma,
    axon: Axon,
}

impl Neuron {
    pub fn run(&mut self, inputs: &Vec<f64>, outputs: &Vec<f64>) -> f64 {
        self.forward(inputs);
        self.backward(outputs);
    }

    fn forward(&mut self, inputs: &Vec<f64>) -> f64 {
        for (dendrite, input) in self.dendtrites.iter_mut().zip(inputs) {
            dendrite.consume(input);
        }

        self.soma.consume(&self.dendtrites);
        self.axon.activate(&self.soma)
    }

    fn backward(&mut self, outputs: &Vec<f64>) {
        todo!()
    }
}
