use std::f64::consts::E;
#[derive(Debug)]
pub struct AxonTerminal {}

impl AxonTerminal {
    pub fn new() -> Self {
        AxonTerminal {}
    }
    pub fn generate_voltage(&self, variable: f64) -> f64 {
        E.powf(variable) / (1.0 + E.powf(variable))
    }
}
