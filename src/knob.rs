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
        self.value = (self.value + 1).min(MAX);
    }

    pub fn decrease(&mut self) {
        self.value = (self.value - 1).max(MIN);
    }
}