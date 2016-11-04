// Copyright 2016 Do Duy.
// Licensed under the MIT license, see the LICENSE file or
// <http://opensource.org/licenses/MIT>

#[derive(Debug, PartialEq)]
#[derive(RustcEncodable)]
pub struct Output {
    word: String,
}

impl Output {
    pub fn new(word: String) -> Output {
        Output { word: word }
    }
}

#[test]
fn encode() {
    use parser;
    let x = parser::encode(&Output::new("some string".to_owned()));
    assert!(x.is_ok());
    assert_eq!(x.unwrap(), "{\"word\":\"some string\"}")
}
