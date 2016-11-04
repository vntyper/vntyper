// Copyright 2016 Do Duy.
// Licensed under the MIT license, see the LICENSE file or
// <http://opensource.org/licenses/MIT>

#[derive(Debug, PartialEq)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct Modifier {
    pub key: char,
}

impl Modifier {
    pub fn new(c: char) -> Modifier { Modifier{ key: c, } }
}
