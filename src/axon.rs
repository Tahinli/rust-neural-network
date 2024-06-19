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
            while let Some(element) = self.signals.pop() {
                combined_signal.combine(element);
            }
            Some(combined_signal)
        }
    }
}

impl Default for Axon {
    fn default() -> Self {
        Self::new()
    }
}
