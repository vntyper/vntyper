// Copyright 2016 Do Duy.
// Licensed under the MIT license, see the LICENSE file or
// <http://opensource.org/licenses/MIT>

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
}
