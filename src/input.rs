// Copyright 2016 Do Duy.
// Licensed under the MIT license, see the LICENSE file or
// <http://opensource.org/licenses/MIT>

use modifier::Modifier;
use input_method::InputMethod;
use output::Output;

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
            input_method: c,
        }
    }
    pub fn output() -> Output { panic!() }
}

#[test]
fn encode() {
    use parser;
    let x = Input::new("text".to_owned(), 'c', None);
    let res = "{\"word\":\"text\",\"modifier\":{\"key\":\"c\"},\"input_method\":null}";
    assert!(parser::encode(&x).is_ok());
    assert_eq!(parser::encode(&x).unwrap(), res);
}
#[test]
fn decode() {
    use parser;
    let x = Input::new("text".to_owned(), 'c', None);
    let res = "{\"word\":\"text\",\"modifier\":{\"key\":\"c\"},\"input_method\":null}";
    assert!(parser::decode::<Input>(&res).is_ok());
    assert_eq!(parser::decode::<Input>(&res).unwrap(), x);
}
