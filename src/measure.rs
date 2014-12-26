// Copyright 2014 Birunthan Mohanathas
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>. This
// file may not be copied, modified, or distributed except according to those
// terms.

pub struct Measure {
    name: String,
    number_value: f32,
    string_value: String,
}

impl Measure {
    pub fn new(name: &str) -> Measure {
        Measure {
            name: name.to_string(),
            number_value: 0.0f32,
            string_value: "".to_string(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_slice()
    }

    pub fn number_value(&self) -> f32 {
        self.number_value
    }

    pub fn string_value(&self) -> &str {
        self.string_value.as_slice()
    }
}
