// Copyright 2015 Birunthan Mohanathas
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>. This
// file may not be copied, modified, or distributed except according to those
// terms.

use measure::Measureable;

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

    pub fn add_measure(&mut self, measure: Box<Measureable<'a> + 'a>) {
        self.measures.push(measure);
    }

    pub fn measures(&self) -> &Vec<Box<Measureable<'a> + 'a>> {
        &self.measures
    }
}

#[test]
fn test_name() {
    let skin = Skin::new("skin");
    assert_eq!("skin", skin.name());
}

#[test]
fn test_add_measure() {
    use time_measure::TimeMeasure;

    let mut skin = Skin::new("skin");
    skin.add_measure(box TimeMeasure::new("foo"));
    assert_eq!(1, skin.measures().len());
}
