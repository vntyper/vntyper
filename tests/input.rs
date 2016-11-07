extern crate vntyper;

use vntyper::input::Input;
use vntyper::input_method::InputMethod;

#[test]
fn non_vietnamese() {
    let input = Input::new("what".to_owned(), 's', InputMethod::telex());
    let output = input.output();
    assert_eq!(output, Err("what".to_owned()));
}
#[test]
fn vietnamese() {
    fn test_ok(a: &str, b: char, c: &str) {
        let input = Input::new(a.to_owned(), b, InputMethod::telex());
        assert_eq!(input.output(), Ok(c.to_owned()));
    }
    fn test_err(a: &str, b: char, c: &str) {
        let input = Input::new(a.to_owned(), b, InputMethod::telex());
        assert_eq!(input.output(), Err(c.to_owned()));
    }
    test_ok("chau", 's', "cháu"); test_ok("sương", 's', "sướng");
    test_ok("dm nha", 's', "dm nhá"); test_ok(".chau", 's', ".cháu");
    test_ok("giao", 's', "giáo"); test_ok("gi", 'f', "gì");
    test_err("cháu", 's', "chau"); test_err("cháu!", 's', "cháu!");

    test_ok("u", 'w', "ư");
}
