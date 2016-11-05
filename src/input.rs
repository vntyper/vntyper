// Copyright 2016 Do Duy.
// Licensed under the MIT license, see the LICENSE file or
// <http://opensource.org/licenses/MIT>

use util;
use modifier::Modifier;
use input_method::InputMethod;
use rustc_serialize::json;

#[derive(Debug, PartialEq)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct Input {
    word: String,
    modifier: Modifier,
    input_method: InputMethod,
}

impl Input {
    fn new(a: String, b: char, c: InputMethod) -> Input {
        Input {
            word: a,
            modifier: Modifier::new(b),
            input_method: c,
        }
    }
    pub fn output(&self) -> Option<String> {
        if !util::is_vietnamese(&self.word) {
            None
        } else {
            panic!();
        }
    }
    pub fn decode(s: &str) -> Result<Input, json::DecoderError> {
        json::decode::<Input>(s)
    }
}

#[test]
fn encode_decode() {
    let x = Input::new("text".to_owned(), 'c', InputMethod::telex());
    let res = json::encode(&x).unwrap();
    assert!(Input::decode(&res).is_ok());
    assert_eq!(Input::decode(&res).unwrap(), x);
}
#[test]
fn non_vietnamese() {
    let input = Input::new("what".to_owned(), 's', InputMethod::telex());
    let output = input.output();
    assert_eq!(output, None);
}
