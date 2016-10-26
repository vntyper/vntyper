extern crate vntyper;

use vntyper::output::Output;
use vntyper::parser;

#[test]
fn output() {
    let x = parser::encode(&Output::new("some string".to_owned()));
    assert!(x.is_ok());
    assert!(x.unwrap() == "{\"word\":\"some string\"}")
}
