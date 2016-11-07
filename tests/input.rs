extern crate vntyper;

use vntyper::input::Input;
use vntyper::input_method::InputMethod;

#[test]
fn non_vietnamese() {
    let input = Input::new("what".to_owned(), 's', InputMethod::telex());
    let output = input.output();
    assert_eq!(output, None);
}
#[test]
fn vietnamese() {
    fn test_ok(a: &str, b: char, c: &str) {
        let input = Input::new(a.to_owned(), b, InputMethod::telex());
        assert_eq!(input.output(), Some(c.to_owned()));
    }
    test_ok("chau", 's', "cháu");
    test_ok("sương", 's', "sướng");
}
