// Copyright 2014 Birunthan Mohanathas
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>. This
// file may not be copied, modified, or distributed except according to those
// terms.

pub trait Measureable<'a> {
    fn name(&'a self) -> &'a str;
    fn update(&'a mut self);
    fn number_value(&'a self) -> f32;
    fn string_value(&'a self) -> String;
}
