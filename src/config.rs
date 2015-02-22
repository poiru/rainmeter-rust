// Copyright 2015 Birunthan Mohanathas
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>. This
// file may not be copied, modified, or distributed except according to those
// terms.

use core::num::from_str_radix;
use ini::Ini;

struct Config {
    ini: Ini,
}

impl Config {
    pub fn load_from_str(string: &str) -> Config {
        Config {
            ini: Ini::load_from_str(string).unwrap(),
        }
    }

    pub fn get_str(&mut self, section: &str, key: &str) -> Option<String> {
        self.ini.begin_section(section);
        let value = match self.ini.get(key) {
            Some(v) => Some(String::from_str(v)),
            None => None,
        };
        self.ini.end_section();
        value
    }

    pub fn get_i32(&mut self, section: &str, key: &str) -> Option<i32> {
        self.ini.begin_section(section);
        let value = match self.ini.get(key) {
            Some(v) => from_str_radix(v, 10).ok(),
            None => None,
        };
        self.ini.end_section();
        value
    }

    pub fn get_u32(&mut self, section: &str, key: &str) -> Option<u32> {
        self.ini.begin_section(section);
        let value = match self.ini.get(key) {
            Some(v) => from_str_radix(v, 10).ok(),
            None => None,
        };
        self.ini.end_section();
        value
    }

    pub fn get_f32(&mut self, section: &str, key: &str) -> Option<f32> {
        self.ini.begin_section(section);
        let value = match self.ini.get(key) {
            Some(v) => from_str_radix(v, 10).ok(),
            None => None,
        };
        self.ini.end_section();
        value
    }
}

#[test]
fn test_get() {
    let mut conf = Config::load_from_str(
        "[s]
         i32=-123
         u32=-123
         f32=1.23");
    assert_eq!(-123i32, conf.get_i32("s", "i32").unwrap());
    assert!(conf.get_u32("s", "u32").is_none());
    assert_eq!(1.23f32, conf.get_f32("s", "f32").unwrap());
}
