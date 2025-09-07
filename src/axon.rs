use crate::signal::Signal;

#[derive(Debug)]
pub struct Axon {
    signals: Vec<Signal>,
}

impl Axon {
    pub fn new() -> Self {
        Axon { signals: vec![] }
    }

    pub fn consume_signal(&mut self) -> Option<Signal> {
        if self.signals.is_empty() {
            None
        } else {
            let mut combined_signal = Signal::new();
            while !self.signals.is_empty() {
                combined_signal.combine(self.signals.pop().unwrap());
            }
            Some(combined_signal)
        }
    }
}
