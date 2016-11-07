// Copyright 2016 Do Duy.
// Licensed under the MIT license, see the LICENSE file or
// <http://opensource.org/licenses/MIT>

use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::fmt;
use std::slice;
use std::iter;

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

#[derive(Ord, Eq, PartialEq, PartialOrd, Clone, Debug)]
pub enum VResult {
    Set,
    Unset,
    None,
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
pub enum Flag {
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
                    write!(f, "{}", c)
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
    fn toggle_tone(&mut self, tone: &Tone) -> VResult {
        if let &mut VChar::Vovel(_, _, ref mut x) = self {
            if x == tone {
                *x = Tone::N;
                VResult::Unset
            } else {
                *x = tone.clone();
                VResult::Set
            }
        } else {
            VResult::None
        }
    }
    pub fn from_char(c: char) -> (Self, bool) {
        let c_lower: Vec<char> = c.to_lowercase().collect();
        // Some foreign characters have longer size when lowercased.
        // These characters are considered as `Invalid` as it is foreign.
        if c_lower.len() > 1 {
            (VChar::Invalid(c), false)
        } else {
            let c_lower = c_lower[0];
            if let Some(&(ref x, ref y, ref z)) = vovel_table_reverse().get(&c_lower) {
                (VChar::Vovel(x.clone(), y.clone(), z.clone()), c.is_uppercase())
            } else if consonants().contains(&c_lower) {
                (VChar::Consonant(c_lower), c.is_uppercase())
            } else {
                (VChar::Invalid(c_lower), c.is_uppercase())
            }
        }
    }
}

/// This struct hold a sequence of character for Vietnamese's text processing.
/// The data it hold does not need to be a valid Vietnamese text.
#[derive(Clone, Debug, PartialEq)]
pub struct VWord {
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
/// Append a VChar at the end.
    pub fn push(&mut self, c: VChar, is_uppercase: bool) {
        self.data.push(c);
        self.upcase.push(is_uppercase);
    }
/// Return a `VWord` from a string.
    pub fn from_str(s: &str) -> VWord {
        // Return value
        let mut ret = VWord::new();
        for c in s.chars() {
            let (x, y) = VChar::from_char(c);
            ret.push(x, y);
        }
        ret
    }
    pub fn new_raw(data: Vec<VChar>, upcase: Vec<bool>) -> VWord {
        VWord {
            data: data,
            upcase: upcase,
        }
    }
/// Set the tone of word
    pub fn toggle_tone(&mut self, tone: &Tone) -> VResult {
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

        macro_rules! empty {
            () => {
                if vovels.len() == 0 {
                    return VResult::None;
                }
            };
        };

        assert!(vovels_index.len() == vovels.len()); // TEST
        assert!(vovels.len() <= 3); // TEST

        empty!();

        // if 'u' followed 'q' then 'u' is not a vovel
        if (Raw::U, Flag::N) == vovels[0] {
            if Some(&VChar::Consonant('q')) == self.data.get(
                vovels_index[0].checked_sub(1).unwrap_or(vovels_index[0])
            ) {
                vovels.remove(0);
                vovels_index.remove(0);
            }
        }

        empty!();

        // In 'gia', 'i' is not vovel, but in 'gi', 'i' is vovel.
        if vovels.len()>1 && (Raw::I, Flag::N) == vovels[0] {
            if Some(&VChar::Consonant('g')) == self.data.get(
                vovels_index[0].checked_sub(1).unwrap_or(vovels_index[0])
            ) {
                vovels.remove(0);
                vovels_index.remove(0);
            }
        }

        empty!();

        if vovels.len() == 1 {
            return self.data[vovels_index[0]].toggle_tone(&tone);
        }

        // Double vovel (nguyên âm đôi)
        for i in 0..vovels.len()-1 {
            macro_rules! two_vovels {
                ( $x:ident, $y:ident, $a:ident, $b:ident, $z:expr ) => {
                    {
                        let x = (Raw::$x, Flag::$y);
                        let y = (Raw::$a, Flag::$b);
                        if x == vovels[i] && y == vovels[i+1] {
                            return self.data[vovels_index[i+$z]].toggle_tone(&tone);
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

        // 'y', 'i' is strictly a "phụ âm cuối".
        // 'o', 'u' can be a "phụ âm cuối".
        for i in 1..vovels.len() {
            if vovels[i] == (Raw::Y, Flag::N)
            || vovels[i] == (Raw::U, Flag::N)
            || vovels[i] == (Raw::I, Flag::N)
            || vovels[i] == (Raw::O, Flag::N) {
                return self.data[vovels_index[i-1]].toggle_tone(&tone);
            }
        }
        // 'u', 'o' can be a "phụ âm đầu".
        for i in 0..vovels.len()-1 {
            if vovels[i] == (Raw::O, Flag::N)
            || vovels[i] == (Raw::U, Flag::N) {
                return self.data[vovels_index[i+1]].toggle_tone(&tone);
            }
        }

        // 'Self' may be an invalid Vietnamese word,
        // temporary set tone for the last vovels.
        return self.data.last_mut().unwrap().toggle_tone(&tone);
    }
/// Toggle between 'd' and 'đ'.
/// Return `Err(())` when turn from 'đ' to 'd'.
    pub fn toggle_d(&mut self) -> VResult {
        for i in (0..self.data.len()).rev() {
            if let VChar::Consonant('d') = self.data[i] {
                self.data[i] = VChar::Consonant('đ');
                return VResult::Set
            } else if let VChar::Consonant('đ') = self.data[i] {
                self.data[i] = VChar::Consonant('d');
                return VResult::Unset;
            }
        }
        VResult::None
    }
/// Toggle a specific flag of a specific raw vovel.
/// Example, toggle flag `Flag::D` of `Raw::A` will turn:
/// - 'a' to 'â', 'ă' to 'â', return `Ok(())`
/// - 'â' to 'a', return `Err(())`
    pub fn toggle_vovel(&mut self, raw: &Raw, flag: &Flag) -> VResult {
        for i in (0..self.data.len()).rev() {
            if let VChar::Vovel(ref x, ref mut y, _) = self.data[i] {
                if x == raw {
                    if y == flag {
                        *y = Flag::N;
                        return VResult::Unset;
                    } else {
                        *y = flag.clone();
                        return VResult::Set;
                    }
                }
            }
        }

        VResult::None
    }
    pub fn iter(&self) -> iter::Zip<slice::Iter<VChar>, slice::Iter<bool>> {
        self.data.iter().zip(self.upcase.iter())
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
fn test_vchar_toggle_tone() {
    macro_rules! test {
        ( $x:ident, $a:ident, $y:ident, $z:ident, $s:expr, $r:ident ) => {
            {
                let mut tmp = VChar::Vovel(Raw::$x, Flag::$a, Tone::$y);
                assert_eq!(($s, tmp.toggle_tone(&Tone::$z)), ($s, VResult::$r));
                assert_eq!(tmp.to_string(), $s);
            }
        };
    }
    test!(A, N, F, S, "á", Set); test!(O, W, N, J, "ợ", Set);
    test!(U, N, S, S, "u", Unset); test!(E, D, N, F, "ề", Set);
    test!(I, N, N, X, "ĩ", Set); test!(A, D, S, R, "ẩ", Set);
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

    assert_eq!(VWord::from_str(".cháu").to_string(), ".cháu");
    assert_eq!(VWord::from_str(" _asx").to_string(), " _asx");
    assert_eq!(VWord::from_str("tiếng ViỆt").to_string(), "tiếng ViỆt");
    assert_eq!(VWord::from_str("ĐộNg").to_string(), "ĐộNg");
}
#[test]
fn test_vword_toggle_tone() {
    macro_rules! test {
        ( $x:expr, $y:ident, $z:expr, $r:ident) => {
            {
                let mut tmp = VWord::from_str($x);
                assert_eq!(($x,tmp.toggle_tone(&Tone::$y)), ($x,VResult::$r));
                assert_eq!(tmp.to_string(), $z);
            }
        };
    }

    // Single vovel test
    test!("cha", R, "chả", Set); test!("chản", S, "chán", Set);
    test!("quy", S, "quý", Set); test!("u", S, "ú", Set);
    test!("qu", S, "qu", None); test!("chả", R, "cha", Unset);
    // Double vovel test
    test!("tiêu", S, "tiếu", Set); test!("yêu", S, "yếu", Set);
    test!("uông", S, "uống", Set); test!("ua", R, "ủa", Set);
    test!("sương", S, "sướng", Set); test!("ia", R, "ỉa", Set);
    test!("hôi", J, "hội", Set);
    test!("ưa", F, "ừa", Set);
    // Other cases
    test!("thuy", S, "thúy", Set); test!("toan", S, "toán", Set);
    test!("oan", S, "oán", Set); test!("tau", F, "tàu", Set);
    test!("ay", R, "ảy", Set); test!("nguyên", X, "nguyễn", Set);
    test!(".chau", S, ".cháu", Set); test!("gi", F, "gì", Set);
    test!("dxf", S, "dxf", None); test!("có", S, "co", Unset);
}
#[test]
fn test_vword_toggle() {
    macro_rules! test {
        ( d $x:expr, $z:expr, $r:ident ) => {
            {
                let mut tmp = VWord::from_str($x);
                assert_eq!(tmp.toggle_d(), VResult::$r);
                assert_eq!(tmp.to_string(), $z);
            }
        };
        ( v $x:ident, $y:ident, $r:expr, $s:expr, $a:ident ) => {
            {
                let mut tmp = VWord::from_str($r);
                assert_eq!(tmp.toggle_vovel(&Raw::$x, &Flag::$y), VResult::$a);
                assert_eq!(tmp.to_string(), $s);
            }
        };
    }
    test!(d "x", "x", None);
    test!(d "dang", "đang", Set); test!(d "đang", "dang", Unset);
    test!(v E, D, "e", "ê", Set); test!(v A, D, "â", "a", Unset);
    test!(v O, W, "ố", "ớ", Set); test!(v O, D, "ồ", "ò", Unset);
    test!(v U, W, "u", "ư", Set); test!(v U, W, "ư", "u", Unset);
    test!(v O, W, "o", "ơ", Set); test!(v O, W, "ơ", "o", Unset);
}
