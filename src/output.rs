// Copyright 2016 Do Duy.
// Licensed under the MIT license, see the LICENSE file or
// <http://opensource.org/licenses/MIT>

#[derive(Debug)]
#[derive(RustcEncodable)]
pub struct Output {
    word: String,
}

impl Output {
    pub fn new(word: String) -> Output {
        Output { word: word }
    }
}
