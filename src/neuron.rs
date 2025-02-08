use crate::{axon::Axon, dendrite::Dendrite, soma::Soma};

#[derive(Debug, Default)]
pub struct Neuron {
    dendtrites: Vec<Dendrite>,
    soma: Soma,
    axon: Axon,
}
