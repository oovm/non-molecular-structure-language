mod mass_data;

pub use mass_data::mass_data;

#[derive(Debug)]
pub struct Expression {
    number: f32,
    expression: String,
    mass: f32,
    electrical: f32,
}

#[derive(Debug)]
pub struct Particle {
    name: String,
    number: usize,
}

impl Default for Expression {
    fn default() -> Self {
        Self {
            number: 1.0,
            expression: String::from("H"),
            mass: 1.0, // FIXME: not 1.0
            electrical: 0.0,
        }
    }
}

impl Default for Particle {
    fn default() -> Self {
        Self { name: String::from("H"), number: 1 }
    }
}

impl Expression {
    pub fn new(n: f32, e: &str) -> Self {
        Self { number: n, expression: String::from(e), mass: 0.0, electrical: 0.0 }
    }
    pub fn eval(&mut self) {
        unimplemented!()
    }
    pub fn mass(&self) -> u128 {
        let m = self.number * self.mass;
        if m <= 0.0 {
            0
        }
        else {
            // TODO: bigger than u128
            m.floor() as u128
        }
    }
    pub fn mode(&self) -> usize {
        if self.electrical < 0.0 {
            2
        }
        else if self.electrical > 0.0 {
            1
        }
        else {
            0
        }
    }
}

impl Particle {
    pub fn mass(&self) -> f32 {
        match mass_data().get(self.name.as_str()) {
            None => 0.0,
            Some(s) => s * self.number as f32,
        }
    }
}
