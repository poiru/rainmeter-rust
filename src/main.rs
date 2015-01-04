// Copyright 2014 Birunthan Mohanathas
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>. This
// file may not be copied, modified, or distributed except according to those
// terms.

use measure::Measureable;
use skin::Skin;
use time_measure::TimeMeasure;

mod measure;
mod skin;
mod time_measure;

fn main() {
    let measure = TimeMeasure::new("foo");
    println!("{}", measure.string_value());

    let skin = Skin::new("skin");
    println!("{}", skin.name());
}
