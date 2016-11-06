// Copyright 2016 Do Duy.
// Licensed under the MIT license, see the LICENSE file or
// <http://opensource.org/licenses/MIT>

use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::fmt;

/// Return set of Vietnamese's consonants (phụ âm)
pub fn consonants() -> &'static BTreeSet<char> {
    lazy_static! {
    static ref RES: BTreeSet<char> =
        vec!['b', 'c', 'd', 'đ', 'g', 'h', 'k', 'l', 'm',
             'n', 'p', 'q', 'r', 's', 't', 'v', 'x']
        .into_iter().collect();
    }
    &RES
}

/// Enum to store a character for Vietnamese's text processing.
/// The character does not need to be a valid Vietnamese's character.
#[derive(Ord, Eq, PartialEq, PartialOrd, Clone, Debug)]
pub enum VChar {
    Consonant(char),
    Vovel(Raw, Flag, Tone),
    Invalid(char),
}
#[derive(Ord, Eq, PartialEq, PartialOrd, Clone, Debug)]
pub enum Tone {
    N, // None
    S, // /
    F, // \
    R, // ?
    X, // ~
    J, // .
}
#[derive(Ord, Eq, PartialEq, PartialOrd, Clone, Debug)]
pub enum Raw {
    A, E, I, O, U, Y,
}
/// Enum to hold 'flag' of vovel.
/// `Flag::N`: Flag for a, e, i,... (No flag)
/// `Flag::W`: Flag for ă, ư, ơ
/// `Flag::D`: Flag for â, ô, ê
#[derive(Ord, Eq, PartialEq, PartialOrd, Clone, Debug)]
enum Flag {
    N, // Flag for a, e, i, o, y, u
    W, // Flag for ă, ơ,...
    D, // Flag for â ê ô 
}

// Return a table to lookup from `Vovel` to string.
fn vovel_table() -> &'static BTreeMap<(Raw, Flag, Tone), char> {

    macro_rules! map {
        ( $x:ident, $y:ident, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr ) => {
            {
                let mut temp = BTreeMap::new();
                temp.insert((Raw::$x, Flag::$y, Tone::N), $a);
                temp.insert((Raw::$x, Flag::$y, Tone::S), $b);
                temp.insert((Raw::$x, Flag::$y, Tone::F), $c);
                temp.insert((Raw::$x, Flag::$y, Tone::R), $d);
                temp.insert((Raw::$x, Flag::$y, Tone::X), $e);
                temp.insert((Raw::$x, Flag::$y, Tone::J), $f);
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
        static ref X: BTreeMap<(Raw, Flag, Tone), char> =
            mmap![
                map!(A, N, 'a', 'á', 'à', 'ả', 'ã', 'ạ'),
                map!(A, D, 'â', 'ấ', 'ầ', 'ẩ', 'ẫ', 'ậ'),
                map!(A, W, 'ă', 'ắ', 'ằ', 'ẳ', 'ẵ', 'ặ'),
                map!(E, N, 'e', 'é', 'è', 'ẻ', 'ẽ', 'ẹ'),
                map!(E, D, 'ê', 'ế', 'ề', 'ể', 'ễ', 'ệ'),
                map!(I, N, 'i', 'í', 'ì', 'ỉ', 'ĩ', 'ị'),
                map!(O, N, 'o', 'ó', 'ò', 'ỏ', 'õ', 'ọ'),
                map!(O, D, 'ô', 'ố', 'ồ', 'ổ', 'ỗ', 'ộ'),
                map!(O, W, 'ơ', 'ớ', 'ờ', 'ở', 'ỡ', 'ợ'),
                map!(U, N, 'u', 'ú', 'ù', 'ủ', 'ũ', 'ụ'),
                map!(U, W, 'ư', 'ứ', 'ừ', 'ử', 'ữ', 'ự'),
                map!(Y, N, 'y', 'ý', 'ỳ', 'ỷ', 'ỹ', 'ỵ')
            ];
    }
    &X
}
fn vovel_table_reverse() -> &'static BTreeMap<char, (Raw, Flag, Tone)> {
    lazy_static! {
        static ref X: BTreeMap<char, (Raw, Flag, Tone)> = vovel_table().iter()
            .map(|(key, value)| (value.clone(), key.clone())).collect();
    }
    &X
}

// Implicitly implement `ToString` for `VChar`
// Write a space for invalid Vietnamese vovels. Eg: `Vovel(I, W, N)`
impl fmt::Display for VChar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &VChar::Consonant(ref c) => write!(f, "{}", c),
            &VChar::Vovel(ref x, ref y, ref z) => {
                let c = vovel_table().get(&(x.clone(), y.clone(), z.clone()));
                if let Some(c) = c {
                    write!(f, "{}")
                } else {
                    write!(f, " ")
                }
            },
            &VChar::Invalid(ref c) => write!(f, "{}", c),
        }
    }
}

impl VChar {
    pub fn to_char(&self) -> char {
        self.to_string().chars().next().unwrap()
    }
    fn set_tone(&mut self, tone: &Tone) {
        if let &mut VChar::Vovel(_, _, ref mut x) = self {
            *x = tone.clone();
        }
    }
}

/// This struct hold a sequence of character for Vietnamese's text processing.
/// The data it hold does not need to be a valid Vietnamese text.
#[derive(Clone, Debug, PartialEq)]
struct VWord {
    data: Vec<VChar>,
    upcase: Vec<bool>,
}

impl fmt::Display for VWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut upcase = self.upcase.iter();
        for x in &self.data {
            let up = upcase.next();
            let x = if up.is_none() || *up.unwrap() {
                x.to_char().to_uppercase().collect::<Vec<char>>()[0]
            } else {
                x.to_char()
            };
            if let Err(x) = write!(f, "{}", x) {
                return Err(x);
            }
        }
        Ok(())
    }
}

impl VWord {
/// Return an empty VWord.
    pub fn new() -> VWord {
        VWord {
            data: Vec::new(),
            upcase: Vec::new(),
        }
    }
// Append a VChar at the end.
    fn push(&mut self, c: VChar, is_uppercase: bool) {
        self.data.push(c);
        self.upcase.push(is_uppercase);
    }
/// Return a `VWord` from a string.
    pub fn from_str(s: &str) -> VWord {
        // Return value
        let mut ret = VWord::new();
        for c in s.chars() {
            let c_lower: Vec<char> = c.to_lowercase().collect();
            // Some foreign characters have longer size when lowercased.
            // These characters are considered as `Invalid` as it is foreign.
            if c_lower.len() > 1 {
                ret.push(VChar::Invalid(c), false);
            } else {
                let c_lower = c_lower[0];
                if let Some(&(ref x, ref y, ref z)) = vovel_table_reverse().get(&c_lower) {
                    ret.push(VChar::Vovel(x.clone(), y.clone(), z.clone()), c.is_uppercase());
                } else if consonants().contains(&c_lower) {
                    ret.push(VChar::Consonant(c_lower), c.is_uppercase());
                } else {
                    ret.push(VChar::Invalid(c_lower), c.is_uppercase());
                }
            }
        }
        ret
    }
    fn new_raw(data: Vec<VChar>, upcase: Vec<bool>) -> VWord {
        VWord {
            data: data,
            upcase: upcase,
        }
    }
/// Set the tone of word
    pub fn set_tone(&mut self, tone: &Tone) {
        let mut vovels_index: Vec<usize> = Vec::new();
        let mut vovels: Vec<(Raw, Flag)> = Vec::new();
        // Get list of vovels and its index
        // We don't concern the vovel's tone so we only get `Raw` and `Flag`
        for i in (0..self.data.len()).rev() {
            if let VChar::Vovel(ref u, ref v, _) = self.data[i] {
                vovels_index.push(i);
                vovels.push((u.clone(), v.clone()));
                if vovels_index.len() >= 3 {
                    break;
                }
            } else if !vovels_index.is_empty() {
                break;
            }
        }
        vovels_index.reverse();
        vovels.reverse();

        assert!(vovels_index.len() == vovels.len()); // TEST
        assert!(vovels.len() <= 3); // TEST

        if vovels.len() == 0 {
            return ();
        }

        // if 'u' followed 'q' then 'u' is not a vovel
        if let Some(&VChar::Consonant(q)) = self.data.get(
            vovels_index[0].checked_sub(1).unwrap_or(vovels_index[0])
        ) {
            if q == 'q' {
                vovels.remove(0);
                vovels_index.remove(0);
            }
        }

        if vovels.len() == 1 {
            self.data[vovels_index[0]].set_tone(&tone);
            return ();
        }

        // Double vovel (nguyên âm đôi)
        for i in 0..vovels.len()-1 {
            macro_rules! two_vovels {
                ( $x:ident, $y:ident, $a:ident, $b:ident, $z:expr ) => {
                    {
                        let x = (Raw::$x, Flag::$y);
                        let y = (Raw::$a, Flag::$b);
                        if x == vovels[i] && y == vovels[i+1] {
                            self.data[vovels_index[i+$z]].set_tone(&tone);
                            return ();
                        };
                    }
                }
            };
            two_vovels!(I, N, A, N, 0); // ia => ỉa
            two_vovels!(Y, N, A, N, 0); // ya, this vovel has no tone, temporary set to 0
            two_vovels!(I, N, E, D, 1); // iê => iế
            two_vovels!(Y, N, E, D, 1); // yê => yế
            two_vovels!(U, N, O, D, 1); // uô => uố
            two_vovels!(U, N, A, N, 0); // ua => ủa
            two_vovels!(U, W, O, W, 1); // ươ => ướ
            two_vovels!(U, W, A, N, 0); // ưa => ừa
        }

        // 'y' is strictly a "phụ âm cuối".
        // 'o', 'u' can be a "phụ âm cuối".
        for i in 1..vovels.len() {
            if vovels[i] == (Raw::Y, Flag::N)
            || vovels[i] == (Raw::U, Flag::N)
            || vovels[i] == (Raw::O, Flag::N) {
                self.data[vovels_index[i-1]].set_tone(&tone);
                return ();
            }
        }
        // 'u', 'o' can be a "phụ âm đầu".
        for i in 0..vovels.len()-1 {
            if vovels[i] == (Raw::O, Flag::N)
            || vovels[i] == (Raw::U, Flag::N) {
                self.data[vovels_index[i+1]].set_tone(&tone);
                return ();
            }
        }

        // 'Self' may be an invalid Vietnamese word,
        // temporary set tone for the last vovels.
        self.data.last_mut().unwrap().set_tone(&tone);
    }
/// Toggle between 'd' and 'đ'.
/// Return `Err(())` when turn from 'đ' to 'd'.
    pub fn toggle_d(&mut self) -> Result<(), ()> {
        for i in (0..self.data.len()).rev() {
            if let VChar::Consonant('d') = self.data[i] {
                self.data[i] = VChar::Consonant('đ');
                return Ok(());
            } else if let VChar::Consonant('đ') = self.data[i] {
                self.data[i] = VChar::Consonant('d');
                return Err(());
            }
        }
        Ok(())
    }
/// Toggle a specific flag of a specific raw vovel.
/// Example, toggle flag `Flag::D` of `Raw::A` will turn:
/// - 'a' to 'â', 'ă' to 'â', return `Ok(())`
/// - 'â' to 'a', return `Err(())`
    pub fn toggle_vovel(&mut self, raw: &Raw, flag: &Flag) -> Result<(), ()> {
        for i in (0..self.data.len()).rev() {
            if let VChar::Vovel(ref x, ref mut y, _) = self.data[i] {
                if x == raw {
                    if y == flag {
                        *y = Flag::N;
                        return Err(());
                    } else {
                        *y = flag.clone();
                        return Ok(());
                    }
                }
            }
        }

        Ok(())
    }
}

#[test]
fn test_vchar_to_string() {
    macro_rules! test {
        ( $x:ident, $a:ident, $y:ident, $s:expr ) => {
            {
                let tmp = VChar::Vovel(Raw::$x, Flag::$a, Tone::$y);
                assert_eq!(tmp.to_string(), $s);
            }
        };
    }
    test!(A, W, F, "ằ");
    test!(O, N, N, "o");
    test!(E, D, J, "ệ");
    assert_eq!(&VChar::Consonant('x').to_string(), "x");
    assert_eq!(&VChar::Consonant('đ').to_string(), "đ");
    assert_eq!(&VChar::Invalid('_').to_string(), "_");
}
#[test]
fn test_vchar_set_tone() {
    macro_rules! test {
        ( $x:ident, $a:ident, $y:ident, $z:ident, $s:expr ) => {
            {
                let mut tmp = VChar::Vovel(Raw::$x, Flag::$a, Tone::$y);
                tmp.set_tone(&Tone::$z);
                assert_eq!(tmp.to_string(), $s);
            }
        };
    }
    test!(A, N, F, S, "á");
    test!(O, W, N, J, "ợ");
    test!(U, N, S, N, "u");
    test!(E, D, N, F, "ề");
    test!(I, N, N, X, "ĩ");
    test!(A, D, S, R, "ẩ");
}
#[test]
fn test_vword_parse() {
    assert_eq!(VWord::from_str("Ệ"), VWord::new_raw(
        vec![VChar::Vovel(Raw::E, Flag::D, Tone::J)], vec![true]
    ));
    assert_eq!(VWord::from_str("a"), VWord::new_raw(
        vec![VChar::Vovel(Raw::A, Flag::N, Tone::N)], vec![false]
    ));
    assert_eq!(VWord::from_str("İ"), VWord::new_raw(
        vec![VChar::Invalid('İ')], vec![false]
    ));
    assert_eq!(VWord::from_str("d"), VWord::new_raw(
        vec![VChar::Consonant('d')], vec![false]
    ));
    assert_eq!(VWord::from_str("Đ"), VWord::new_raw(
        vec![VChar::Consonant('đ')], vec![true]
    ));

    assert_eq!(VWord::from_str("cháu").to_string(), "cháu");
    assert_eq!(VWord::from_str(" _asx").to_string(), " _asx");
    assert_eq!(VWord::from_str("tiếng ViỆt").to_string(), "tiếng ViỆt");
    assert_eq!(VWord::from_str("ĐộNg").to_string(), "ĐộNg");
}
#[test]
fn test_vword_set_tone() {
    macro_rules! test {
        ( $x:expr, $y:ident, $z:expr ) => {
            {
                let mut tmp = VWord::from_str($x);
                tmp.set_tone(&Tone::$y);
                assert_eq!(tmp.to_string(), $z);
            }
        };
    }

    // Single vovel test
    test!("cha", R, "chả"); test!("chản", S, "chán");
    test!("quy", S, "quý"); test!("u", S, "ú");
    // Double vovel test
    test!("tiêu", S, "tiếu"); test!("yêu", S, "yếu");
    test!("uông", S, "uống"); test!("ua", R, "ủa");
    test!("sương", S, "sướng"); test!("ia", R, "ỉa");
    test!("ưa", F, "ừa");
    // Other cases
    test!("thuy", S, "thúy"); test!("toan", S, "toán");
    test!("oan", S, "oán"); test!("tau", F, "tàu");
    test!("ay", R, "ảy"); test!("nguyên", X, "nguyễn");
}
#[test]
fn teset_vword_toggle() {
    macro_rules! test {
        ( d $x:expr, $z:expr, $r:expr ) => {
            {
                let mut tmp = VWord::from_str($x);
                assert_eq!(tmp.toggle_d(), $r);
                assert_eq!(tmp.to_string(), $z);
            }
        };
        ( v $x:ident, $y:ident, $r:expr, $s:expr, $a:expr ) => {
            {
                let mut tmp = VWord::from_str($r);
                assert_eq!(tmp.toggle_vovel(&Raw::$x, &Flag::$y), $a);
                assert_eq!(tmp.to_string(), $s);
            }
        };
    }
    test!(d "dang", "đang", Ok(()) ); test!(d "đang", "dang", Err(()) );
    test!(v E, D, "e", "ê", Ok(())); test!(v A, D, "â", "a", Err(()));
    test!(v O, W, "ố", "ớ", Ok(())); test!(v O, D, "ồ", "ò", Err(()));
}
