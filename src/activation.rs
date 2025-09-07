pub struct Activation {
    coefficient_a: f64,
    variable: f64,
    coefficient_b: f64,
}

impl Activation {
    pub fn new() -> Self {
        Activation {
            coefficient_a: 1.0,
            variable: 0.0,
            coefficient_b: 0.0,
        }
    }

    pub fn get(&self) -> Self {
        Activation {
            coefficient_a: self.coefficient_a,
            variable: self.variable,
            coefficient_b: self.coefficient_b,
        }
    }

    pub fn set(&mut self, coefficient_a: f64, variable: f64, coefficient_b: f64) {
        self.coefficient_a = coefficient_a;
        self.variable = variable;
        self.coefficient_b = coefficient_b;
    }

    pub fn calculate(&self) -> f64 {
        self.coefficient_a * self.variable + self.coefficient_b
    }

    pub fn set_coefficient_a(&mut self, coefficient_a: f64) {
        self.coefficient_a = coefficient_a;
    }

    pub fn set_variable(&mut self, variable: f64) {
        self.variable = variable;
    }

    pub fn set_coefficient_b(&mut self, coefficient_b: f64) {
        self.coefficient_b = coefficient_b;
    }
}
