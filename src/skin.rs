// Copyright 2015 Birunthan Mohanathas
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>. This
// file may not be copied, modified, or distributed except according to those
// terms.

use measure::Measureable;
use time_measure::TimeMeasure;

pub struct Skin<'a> {
    name: String,
    measures: Vec<Box<Measureable<'a> + 'a>>,
}

impl<'a> Skin<'a> {
    pub fn new(name: &str) -> Skin {
        Skin {
            name: name.to_string(),
            measures: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_slice()
    }
}
