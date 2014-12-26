// Copyright 2014 Birunthan Mohanathas
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>. This
// file may not be copied, modified, or distributed except according to those
// terms.

pub struct Measure {
    name: String,
    value: f32,
}

impl Measure {
    pub fn new(name: &str) -> Measure {
        Measure {
            name: name.to_string(),
            value: 0.0f32,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_slice()
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn update_value(&mut self) {
        self.value += 1.0f32
    }
}
