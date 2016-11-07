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
    pub fn get_type(&self, c: char) -> Vec<KeyType> {
        let mut ret = Vec::new();
        if c == self.aa { ret.push(KeyType::Toggle(Raw::A, Flag::D)); }
        if c == self.aw { ret.push(KeyType::Toggle(Raw::A, Flag::W)); }
        if c == self.ee { ret.push(KeyType::Toggle(Raw::E, Flag::D)); }
        if c == self.oo { ret.push(KeyType::Toggle(Raw::O, Flag::D)); }
        if c == self.ow { ret.push(KeyType::Toggle(Raw::O, Flag::W)); }
        if c == self.uw { ret.push(KeyType::Toggle(Raw::U, Flag::W)); }
        if c == self.dd { ret.push(KeyType::ToggleD); }
        if c == self.s { ret.push(KeyType::Tone(Tone::S)); }
        if c == self.f { ret.push(KeyType::Tone(Tone::F)); }
        if c == self.r { ret.push(KeyType::Tone(Tone::R)); }
        if c == self.x { ret.push(KeyType::Tone(Tone::X)); }
        if c == self.j { ret.push(KeyType::Tone(Tone::J)); }
        ret.push(KeyType::None);
        ret
    }
}
