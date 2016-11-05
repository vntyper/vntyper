// Copyright 2016 Do Duy.
// Licensed under the MIT license, see the LICENSE file or
// <http://opensource.org/licenses/MIT>

use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::fmt;

/// Return set of Vietnamese's consonants (phụ âm)
pub fn consonants() -> &'static BTreeSet<&'static str> {
    lazy_static! {
    static ref RES: BTreeSet<&'static str> =
        ["b", "c", "d", "đ", "g", "h", "k", "l", "m",
         "n", "p", "q", "r", "s", "t", "v", "x"]
        .iter().map(|x| x.to_owned()).collect();
    }
    &RES
}

/// Enum represents Vietnamese's single vovel (nguyên âm).
#[derive(Ord, Eq, PartialEq, PartialOrd, Clone)]
enum Vovel {
    A, Aa, Aw, E, Ee, I, O, Oo, Ow, U, Uw, Y,
}

// Implicitly implement `ToString` for `Vovel`
impl fmt::Display for Vovel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Vovel::A  => write!(f, "a"),
            &Vovel::Aa => write!(f, "â"),
            &Vovel::Aw => write!(f, "ă"),
            &Vovel::E  => write!(f, "e"),
            &Vovel::Ee => write!(f, "ê"),
            &Vovel::I  => write!(f, "i"),
            &Vovel::O  => write!(f, "o"),
            &Vovel::Oo => write!(f, "ô"),
            &Vovel::Ow => write!(f, "ơ"),
            &Vovel::U  => write!(f, "u"),
            &Vovel::Uw => write!(f, "ư"),
            &Vovel::Y  => write!(f, "y"),
        }
    }
}

/// Enum represents Vietnamese's tone (thanh điệu - dấu).
#[derive(Ord, Eq, PartialEq, PartialOrd, Clone)]
enum Tone {
    N, // None
    S, // /
    F, // \
    R, // ?
    X, // ~
    J, // .
}

/// Enum to store a Vietnamese's character, which is a consonant or a vovel.
enum VChar {
    Consonant(&'static str),
    Vovel(Vovel, Tone),
}

// Implicitly implement `ToString` for `Vovel`
impl fmt::Display for VChar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &VChar::Consonant(c) => write!(f, "{}", c),
            &VChar::Vovel(ref x, ref y) => {
                write!(f, "{}", vovel_table().get(&(x.clone(), y.clone())).unwrap())
            },
        }
    }
}

// Return a table to lookup from `VChar::Vovel` to string.
fn vovel_table() -> &'static BTreeMap<(Vovel, Tone), &'static str> {

    macro_rules! map {
        ( $x:ident, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr ) => {
            {
                let mut temp = BTreeMap::new();
                temp.insert((Vovel::$x, Tone::N), $a);
                temp.insert((Vovel::$x, Tone::S), $b);
                temp.insert((Vovel::$x, Tone::F), $c);
                temp.insert((Vovel::$x, Tone::R), $d);
                temp.insert((Vovel::$x, Tone::X), $e);
                temp.insert((Vovel::$x, Tone::J), $f);
                temp
            }
        };
    }

    macro_rules! mmap {
        ( $( $x:expr ),* ) => {
            {
                let mut temp = BTreeMap::new();
                $( temp.append(&mut $x); )*
                temp
            }
        };
    }

    lazy_static! {
        static ref X: BTreeMap<(Vovel, Tone), &'static str> =
            mmap![
                map!(A , "a", "á", "à", "ả", "ã", "ạ"),
                map!(Aa, "â", "ấ", "ầ", "ẩ", "ẫ", "ậ"),
                map!(Aw, "ă", "ắ", "ằ", "ẳ", "ẵ", "ặ"),
                map!(E , "e", "é", "è", "ẻ", "ẽ", "ẹ"),
                map!(Ee, "ê", "ế", "ề", "ể", "ễ", "ệ"),
                map!(I , "i", "í", "ì", "ỉ", "ĩ", "ị"),
                map!(O , "o", "ó", "ò", "ỏ", "õ", "ọ"),
                map!(Oo, "ô", "ố", "ồ", "ổ", "ỗ", "ộ"),
                map!(Ow, "ơ", "ớ", "ờ", "ở", "ỡ", "ợ"),
                map!(U , "u", "ú", "ù", "ủ", "ũ", "ụ"),
                map!(Uw, "ư", "ứ", "ừ", "ử", "ữ", "ự"),
                map!(Y , "y", "ý", "ỳ", "ỷ", "ỹ", "ỵ")
            ];
    }
    &X
}

#[test]
fn vovel_table_test() {
    let map = vovel_table();
    assert_eq!(map.get(&(Vovel::A, Tone::S)), Some(&"á"));
    assert_eq!(map.get(&(Vovel::Aw, Tone::F)), Some(&"ằ"));
    assert_eq!(map.get(&(Vovel::Y, Tone::N)), Some(&"y"));
    assert_eq!(map.get(&(Vovel::Oo, Tone::J)), Some(&"ộ"));
    assert_eq!(map.get(&(Vovel::U, Tone::R)), Some(&"ủ"));
    assert_eq!(map.get(&(Vovel::E, Tone::X)), Some(&"ẽ"));
    assert_eq!(map.get(&(Vovel::Ow, Tone::R)), Some(&"ở"));
}
#[test]
fn vchar_to_string() {
    assert_eq!(&VChar::Vovel(Vovel::Aw, Tone::F).to_string(), "ằ");
    assert_eq!(&VChar::Vovel(Vovel::O, Tone::N).to_string(), "o");
    assert_eq!(&VChar::Vovel(Vovel::Ee, Tone::J).to_string(), "ệ");
    assert_eq!(&VChar::Consonant("x").to_string(), "x");
    assert_eq!(&VChar::Consonant("đ").to_string(), "đ");
}
