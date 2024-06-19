#[derive(Debug)]
pub struct Signal {
    signal_value: f64,
}

impl Signal {
    pub fn new() -> Self {
        Signal { signal_value: 0.0 }
    }

    pub fn new_with_value(signal_value: f64) -> Self {
        Signal { signal_value }
    }
    pub fn set_signal_value(&mut self, signal_value: f64) {
        self.signal_value = signal_value;
    }

    pub fn combine(&mut self, other_signal: Signal) {
        self.set_signal_value(self.get_signal_value() + other_signal.get_signal_value())
    }

    pub fn get_signal_value(&self) -> f64 {
        self.signal_value
    }
}

impl Default for Signal {
    fn default() -> Self {
        Self::new()
    }
}
