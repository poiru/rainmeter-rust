// Copyright 2014 Birunthan Mohanathas
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>. This
// file may not be copied, modified, or distributed except according to those
// terms.

extern crate time;

use measure::Measureable;

pub struct TimeMeasure {
    name: String,
}

impl TimeMeasure {
    pub fn new(name: &str) -> TimeMeasure {
        TimeMeasure {
            name: name.to_string(),
        }
    }
}

impl<'a> Measureable<'a> for TimeMeasure {
    fn name(&'a self) -> &'a str {
        self.name.as_slice()
    }

    fn number_value(&'a self) -> f32 {
        0.0f32
    }

    fn string_value(&'a self) -> String {
        time::strftime("%H:%M", &time::now()).unwrap()
    }
}
