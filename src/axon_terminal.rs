use crate::signal::Signal;

#[derive(Debug)]
pub struct AxonTerminal {
    signal: Option<Signal>,
}

impl AxonTerminal {
    pub fn new() -> Self {
        AxonTerminal { signal: None }
    }
    pub fn receive_signal(&mut self, signal: Signal) {
        self.signal = Some(signal);
    }
    pub fn consume_signal(&mut self) -> Option<Signal> {
        match &mut self.signal {
            Some(signal) => {
                let signal = Signal::new_with_value(signal.get_signal_value());
                self.signal = None;
                Some(signal)
            }
            None => None,
        }
    }
}

impl Default for AxonTerminal {
    fn default() -> Self {
        Self::new()
    }
}
