// Copyright 2016 Do Duy.
// Licensed under the MIT license, see the LICENSE file or
// <http://opensource.org/licenses/MIT>

use vword::{ VWord, VChar, Flag, Tone };
use std::collections::HashSet;

pub fn is_vietnamese(input: &VWord) -> bool {
    let raw: &Vec<VChar> = input.vchars();

    // Return false if input cointains Invalid character
    for x in raw.iter() {
        if let VChar::Invalid(_) = *x {
            return false;
        }
    }

    lazy_static! {
        static ref ALLOW_PRE: HashSet<VWord> = vec![
            "ph", "th", "tr", "gi", "d", "ch", "nh", "ng", "ngh", "kh", "g", "gh",
            "c", "q", "k", "t", "r", "h", "b", "m", "v", "đ", "n", "l", "x", "p",
            "s",
        ].iter().map(|x| VWord::from_str(x)).collect();
        static ref ALLOW_POST: HashSet<VWord> = vec![
            "n", "ng", "t", "c", "m", "nh", "ch", "p",
        ].iter().map(|x| VWord::from_str(x)).collect();
    }

    // Split input into slice of same VChar kind (Consonant or Vovel)
    // Example: 'chich' will be split into ['ch', 'i', 'ch']
    let split: Vec<VWord> = {
        raw.iter().fold(Vec::new(), |mut vec, u| {
            if match vec.last() {
                None => true,
                Some(ref x) => match x.iter().next() {
                    None => false,
                    Some((x, _)) => {
                        if let VChar::Consonant(_) = *x {
                            if let VChar::Vovel(_, _, _) = *u {true} else {false}
                        } else { // `x` is `Vovel`, as we have no `Invalid`.
                            if let VChar::Consonant(_) = *u {true} else {false}
                        }
                    }
                }
            } {
                vec.push(VWord::new());
            }
            vec.last_mut().unwrap().push(u.clone(), false);
            vec
        })
    };

    if split.len() > 3 {
        return false;
    }

    fn is_consonants(x: &VWord) -> bool {
        match x.iter().next() {
            None => false,
            Some((x, _)) => match *x {
                VChar::Consonant(_) => true,
                _ => false,
            }
        }
    }
    
    if split.len() == 1 {
        true
    } else if split.len() == 2 {
        let check_1 = !is_consonants(&split[0]) || ALLOW_PRE.contains(&split[0]);
        let check_2 = !is_consonants(&split[1]) || ALLOW_POST.contains(&split[1]);
        check_1 && check_2
    } else if split.len() == 3 {
        if !is_consonants(&split[0]) {
            false
        } else {
            ALLOW_PRE.contains(&split[0]) && ALLOW_POST.contains(&split[2])
        }
    } else { // == 0 ?
        true
    }
}

#[test]
fn test_is_vietnamese() {
    fn test(x: &'static str, b: bool) {
        let tmp = VWord::from_str(x);
        assert_eq!((x, is_vietnamese(&tmp)), (x, b));
    }
    test("hi!", false); test(" x", false); // Nonsene test
    test("way", false); test("fact", false); test("las", false);
    test("sheet", false); test("cor", false); test("xas", false);
    test("phugn", false);

    test("giao", true);
}
