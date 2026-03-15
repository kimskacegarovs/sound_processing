use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

pub struct Knob {
    pub name: String,
    pub value: i32,
}

const MIN: i32 = 0;
const MAX: i32 = 100;


impl Knob {
    pub fn new(name: &str, value: i32) -> Self {
        Self {
            name: name.to_string(),
            value,
        }
    }

    pub fn increase(&mut self) {
        self.increase_by(1);
    }

    pub fn increase_by(&mut self, by: i32) {
        self.value = (self.value + by).min(MAX);
    }

    pub fn decrease(&mut self) {
        self.decrease_by(1);
    }

    pub fn decrease_by(&mut self, by: i32) {
        self.value = (self.value - by).max(MIN);
    }

    pub fn get_multiplier(&self) -> f32 {
        self.value as f32 / 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_multiplier() {
        let knob = Knob::new("test", 10);
        assert_eq!(knob.get_multiplier(), 0.1);
    }
}