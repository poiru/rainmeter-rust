// Copyright 2014 Birunthan Mohanathas
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>. This
// file may not be copied, modified, or distributed except according to those
// terms.

use measure::Measure;

mod measure;

fn main() {
    let measure = Measure::new("foo");
    println!("{}", measure.name());
}
