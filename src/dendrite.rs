use crate::signal::Signal;

#[derive(Debug)]
pub struct Dendrite {
    weight: f64,
    signal: Option<Signal>,
}

impl Dendrite {
    pub fn new() -> Self {
        Dendrite {
            weight: 1.0,
            signal: None,
        }
    }

    pub fn consume_signal(&mut self) -> Option<Signal> {
        match &mut self.signal {
            Some(signal) => {
                signal.set_signal_value(self.weight * signal.get_signal_value());
                let signal = Signal::new_with_value(signal.get_signal_value());
                self.signal = None;
                Some(signal)
            }
            None => None,
        }
    }

    pub fn set_weight(&mut self, weight: f64) {
        self.weight = weight;
    }
}

impl Default for Dendrite {
    fn default() -> Self {
        Self::new()
    }
}
