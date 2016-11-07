// Copyright 2016 Do Duy.
// Licensed under the MIT license, see the LICENSE file or
// <http://opensource.org/licenses/MIT>

use vword::{ Raw, Flag, Tone };

#[derive(Debug, PartialEq, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct InputMethod {
    aa: char,
    aw: char,
    ee: char,
    oo: char,
    ow: char,
    uw: char,
    dd: char,
    s: char,
    f: char,
    r: char,
    x: char,
    j: char,
}

pub enum KeyType {
    None,
    Toggle(Raw, Flag),
    Tone(Tone),
    ToggleD,
}

impl InputMethod {
    pub fn new(arr: [char; 12]) -> InputMethod {
        InputMethod {
            aa: arr[0],
            aw: arr[1],
            ee: arr[2],
            oo: arr[3],
            ow: arr[4],
            uw: arr[5],
            dd: arr[6],
            s: arr[7],
            f: arr[8],
            r: arr[9],
            x: arr[10],
            j: arr[11],
        }
    }
    pub fn telex() -> InputMethod {
        InputMethod::new(['a', 'w', 'e', 'o', 'w', 'w', 'd', 's', 'f', 'r', 'x', 'j'])
    }
    pub fn get_type(&self, c: char) -> KeyType {
        match c {
            c if c == self.aa => KeyType::Toggle(Raw::A, Flag::D),
            c if c == self.aw => KeyType::Toggle(Raw::A, Flag::W),
            c if c == self.ee => KeyType::Toggle(Raw::E, Flag::D),
            c if c == self.oo => KeyType::Toggle(Raw::O, Flag::D),
            c if c == self.ow => KeyType::Toggle(Raw::O, Flag::W),
            c if c == self.uw => KeyType::Toggle(Raw::U, Flag::W),
            c if c == self.dd => KeyType::ToggleD,
            c if c == self.s => KeyType::Tone(Tone::S),
            c if c == self.f => KeyType::Tone(Tone::F),
            c if c == self.r => KeyType::Tone(Tone::R),
            c if c == self.x => KeyType::Tone(Tone::X),
            c if c == self.j => KeyType::Tone(Tone::J),
            _ => KeyType::None,
        }
    }
}
