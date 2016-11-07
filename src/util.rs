// Copyright 2016 Do Duy.
// Licensed under the MIT license, see the LICENSE file or
// <http://opensource.org/licenses/MIT>

use vword::{ VWord, VChar, Flag, Tone };

pub fn get_english_form(input: &VWord) -> VWord {
    let mut word: Vec<_> = input.iter().map(|(x, _)| x.clone()).collect();
    for x in &mut word {
        if let VChar::Vovel(_, ref mut f, ref mut t) = *x {
            *f = Flag::N;
            *t = Tone::N;
        }
    }
    word.into_iter().fold(VWord::new(), |mut new, x| {
        new.push(x, false);
        new
    })
}

pub fn is_vietnamese(input: &VWord) -> bool {
    for (x, _) in input.iter() {
        if let VChar::Invalid(_) = *x {
            return false;
        }
    }

    true
}

#[test]
fn test_get_english_form() {
    fn test(x: &'static str, y: &'static str) {
        let tmp = VWord::from_str(x);
        assert_eq!(&get_english_form(&tmp).to_string(), y);
    }
    test("Cháu", "chau");
    test("Nhập môn lập trình", "nhap mon lap trinh");
}
#[test]
fn test_is_vietnamese() {
    fn test(x: &'static str, b: bool) {
        let tmp = VWord::from_str(x);
        assert_eq!((x, is_vietnamese(&tmp)), (x, b));
    }
    test("hi!", false); test(" x", false);
    test("way", false); test("fact", false); test("last", false);
    test("sheet", false); test("cor", false); test("xas", false);
    test("phugn", false);
}
