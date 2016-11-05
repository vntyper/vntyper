// Copyright 2016 Do Duy.
// Licensed under the MIT license, see the LICENSE file or
// <http://opensource.org/licenses/MIT>

#[derive(Debug, PartialEq, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct InputMethod {
    aa: u32,
    aw: u32,
    ee: u32,
    oo: u32,
    ow: u32,
    uw: u32,
    dd: u32,
    s: u32,
    f: u32,
    r: u32,
    x: u32,
    j: u32,
}

impl InputMethod {
    pub fn new(arr: [char; 12]) -> InputMethod {
        InputMethod {
            aa: arr[0] as u32,
            aw: arr[1] as u32,
            ee: arr[2] as u32,
            oo: arr[3] as u32,
            ow: arr[4] as u32,
            uw: arr[5] as u32,
            dd: arr[6] as u32,
            s: arr[7] as u32,
            f: arr[8] as u32,
            r: arr[9] as u32,
            x: arr[10] as u32,
            j: arr[11] as u32,
        } 
    }
    pub fn telex() -> InputMethod {
        InputMethod::new(['a', 'w', 'e', 'o', 'w', 'w', 'd', 's', 'f', 'r', 'x', 'j'])
    }
}
