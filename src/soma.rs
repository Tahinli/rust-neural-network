use std::f64::consts::E;

use crate::signal::Signal;

#[derive(Debug)]
pub struct Soma {
    signal: Option<Signal>,
    bias: f64,
}

impl Soma {
    pub fn new() -> Self {
        Soma {
            signal: None,
            bias: 0.0,
        }
    }
    pub fn activate(&mut self, signal: Signal) {
        let signal_value = signal.get_signal_value();
        self.signal = Some(Signal::new_with_value(
            E.powf(signal_value) / (1.0 + E.powf(signal_value)) + self.bias,
        ));
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

    pub fn set_bias(&mut self, bias: f64) {
        self.bias = bias;
    }
}
