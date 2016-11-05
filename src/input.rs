// Copyright 2016 Do Duy.
// Licensed under the MIT license, see the LICENSE file or
// <http://opensource.org/licenses/MIT>

use util;
use modifier::Modifier;
use input_method::InputMethod;
use output::Output;
use rustc_serialize::json;

#[derive(Debug, PartialEq)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct Input {
    word: String,
    modifier: Modifier,
    input_method: Option<InputMethod>,
}

impl Input {
    fn new(a: String, b: char, c: Option<InputMethod>) -> Input {
        Input {
            word: a,
            modifier: Modifier::new(b),
            input_method: c.or(Some(InputMethod::telex())),
        }
    }
    pub fn output(&self) -> Output {
        let method = self.input_method.clone().unwrap();
        if !util::is_vietnamese(&self.word) {
            Output::new(format!("{}{}", self.word.clone(), self.modifier.key))
        } else {
            panic!();
        }
    }
    pub fn decode(s: &str) -> Result<Input, json::DecoderError> {
        match json::decode::<Input>(s) {
            Err(e) => Err(e),
            Ok(mut x) => {
                if x.input_method.is_none() {
                    x.input_method = Some(InputMethod::telex());
                }
                Ok(x)
            },
        }
    }
}

#[test]
fn decode() {
    let x = Input::new("text".to_owned(), 'c', None);
    let res = "{\"word\":\"text\",\"modifier\":{\"key\":\"c\"},\"input_method\":null}";
    assert!(Input::decode(res).is_ok());
    assert_eq!(Input::decode(res).unwrap(), x);
}
#[test]
fn english() {
    let input = Input::new("what".to_owned(), 's', None);
    let output = input.output();
    assert_eq!(output, Output::new("whats".to_owned()));
}
