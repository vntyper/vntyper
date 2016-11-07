// Copyright 2016 Do Duy.
// Licensed under the MIT license, see the LICENSE file or
// <http://opensource.org/licenses/MIT>

use util;
use input_method::InputMethod;
use input_method::KeyType;
use rustc_serialize::json;
use vword::VWord;

#[derive(Debug, PartialEq)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct Input {
    word: String,
    modifier: char,
    input_method: InputMethod,
}

impl Input {
    pub fn new(a: String, b: char, c: InputMethod) -> Input {
        Input {
            word: a,
            modifier: b,
            input_method: c,
        }
    }

/// Get output of the input - the most important function of the
/// crate.
/// This function return a new string as a replacement for `Input.word`.
/// If the `Input.word` is not a Vietnamese string, return `None`.
    pub fn output(&self) -> Option<String> {
        let mut word = VWord::from_str(self.word.split_whitespace().last().unwrap_or(""));
        if util::is_vietnamese(&word) {
            let key_type = self.input_method.get_type(self.modifier);
            match key_type {
                KeyType::None => None,
                KeyType::Toggle(ref x, ref y) => {
                    match word.toggle_vovel(x, y) {
                        Err(_) => None,
                        Ok(_) => Some(word.to_string()),
                    }
                },
                KeyType::Tone(ref x) => {
                    match word.toggle_tone(x) {
                        Err(_) => None,
                        _ => Some(word.to_string()),
                    }
                },
                KeyType::ToggleD => {
                    match word.toggle_d() {
                        Err(_) => None,
                        Ok(_) => Some(word.to_string()),
                    }
                },
            }
        } else {
            None
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

