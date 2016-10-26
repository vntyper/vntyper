// Copyright 2016 Do Duy
// Licenses under the MIT license, see the LICENSE file or
// <http://opensource.org/license/MIT>

extern crate vntyper;

use vntyper::output::Output;
use vntyper::parser;

fn main() {
    let x = Output::new("some string".to_owned());
    println!("{:?}", parser::encode(&x));
}
