use crate::{
    axon::Axon, axon_terminal::AxonTerminal, dendrite::Dendrite, signal::Signal, soma::Soma,
};

#[derive(Debug)]
pub struct Neuron {
    dendrites: Vec<Dendrite>,
    soma: Soma,
    axon: Axon,
    axon_terminals: Vec<AxonTerminal>,
}

impl Neuron {
    pub fn new() -> Self {
        Neuron {
            dendrites: vec![],
            soma: Soma::new(),
            axon: Axon::new(),
            axon_terminals: vec![],
        }
    }

    pub fn activate(&mut self, other_dendrites: &mut [&mut Dendrite]) -> Option<Signal> {
        Neuron::collect_signals(&mut self.axon_terminals, other_dendrites);
        match self.axon.consume_signal() {
            Some(signal) => {
                self.soma.activate(signal);
                self.soma.consume_signal()
            }
            None => None,
        }
    }

    pub fn borrow_dendrite(&mut self, index: usize) -> &mut Dendrite {
        &mut self.dendrites[index]
    }

    fn collect_signals(axon_terminals: &mut [AxonTerminal], other_dendrites: &mut [&mut Dendrite]) {
        for (other_dendrite, axon_terminal) in
            other_dendrites.iter_mut().zip(axon_terminals.iter_mut())
        {
            match other_dendrite.consume_signal() {
                Some(signal) => {
                    axon_terminal.receive_signal(signal);
                }
                None => return,
            }
        }
    }

    pub fn learn(&mut self, loss: f64) {
        // we need actual calculation
        let weight = loss;
        for dendrite in &mut self.dendrites {
            dendrite.set_weight(weight)
        }
    }
}

impl Default for Neuron {
    fn default() -> Self {
        Self::new()
    }
}
