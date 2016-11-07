// Copyright 2016 Do Duy.
// Licensed under the MIT license, see the LICENSE file or
// <http://opensource.org/licenses/MIT>

use util;
use input_method::InputMethod;
use input_method::KeyType;
use rustc_serialize::json;
use vword::{ VWord, VChar };

#[derive(Debug, PartialEq)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct Input {
    word: String,
    modifier: char,
    input_method: InputMethod,
}

impl Input {
    pub fn new(a: String, mut b: char, c: InputMethod) -> Input {
        let b_lower: Vec<_> = b.to_lowercase().collect();
        if b_lower.len() > 1 {
            b = '\0';
        } else {
            b = b_lower[0];
        }
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
    pub fn output(&self) -> Result<String, String> {
        let all = VWord::from_str(&self.word);
        let (mut word, rest) = {
            let last_non_alphabet_index = all.iter().enumerate()
                .fold(-1i32, |mut res, (i, (ref x, _))| {
                    if let VChar::Invalid(c) = **x {
                        if !c.is_alphabetic() && !c.is_digit(10) {
                            res = i as i32;
                        }
                    }
                    res
                });

            let word: Vec<_> = all.iter()
                .skip((last_non_alphabet_index+1) as usize).collect();
            let (x, y) = word.into_iter()
                .fold((Vec::new(), Vec::new()), |mut res, (x, y)| {
                    (res.0).push(x.clone());
                    (res.1).push(y.clone());
                    res
                });
            let word: Vec<_> = all.iter().take((last_non_alphabet_index+1) as usize)
                .collect();
            let (z, t) = word.into_iter()
                .fold((Vec::new(), Vec::new()), |mut res, (x, y)| {
                    (res.0).push(x.clone());
                    (res.1).push(y.clone());
                    res
                });
            (VWord::new_raw(x, y), VWord::new_raw(z, t))
        };
        if util::is_vietnamese(&word) {
            let key_type = self.input_method.get_type(self.modifier);
            match key_type {
                KeyType::None => Err(rest.to_string() + &word.to_string()),
                KeyType::Toggle(ref x, ref y) => {
                    match word.toggle_vovel(x, y) {
                        Err(_) => Err(rest.to_string() + &word.to_string()),
                        Ok(_) => Ok(rest.to_string() + &word.to_string()),
                    }
                },
                KeyType::Tone(ref x) => {
                    match word.toggle_tone(x) {
                        Err(_) => Err(rest.to_string() + &word.to_string()),
                        Ok(_) => Ok(rest.to_string() + &word.to_string()),
                    }
                },
                KeyType::ToggleD => {
                    match word.toggle_d() {
                        Err(_) => Err(rest.to_string() + &word.to_string()),
                        Ok(_) => Ok(rest.to_string() + &word.to_string()),
                    }
                },
            }
        } else {
            Err(rest.to_string() + &word.to_string())
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
