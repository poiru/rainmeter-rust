// Copyright 2014 Birunthan Mohanathas
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>. This
// file may not be copied, modified, or distributed except according to those
// terms.

extern crate time;

use measure::Measureable;

pub struct TimeMeasure {
    name: String,
    value: time::Tm,
}

impl TimeMeasure {
    pub fn new(name: &str) -> TimeMeasure {
        TimeMeasure {
            name: name.to_string(),
            value: time::now(),
        }
    }
}

impl<'a> Measureable<'a> for TimeMeasure {
    fn name(&'a self) -> &'a str {
        self.name.as_slice()
    }

    fn update(&'a mut self) {
        self.value = time::now();
    }

    fn number_value(&'a self) -> f32 {
        self.value.to_timespec().sec as f32
    }

    fn string_value(&'a self) -> String {
        time::strftime("%H:%M", &self.value).unwrap_or(String::new())
    }
}
