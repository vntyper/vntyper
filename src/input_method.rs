// Copyright 2016 Do Duy.
// Licensed under the MIT license, see the LICENSE file or
// <http://opensource.org/licenses/MIT>

use modifier::Modifier;

#[derive(Debug, PartialEq, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct InputMethod {
    aa: Modifier,
    aw: Modifier,
    ee: Modifier,
    oo: Modifier,
    ow: Modifier,
    uw: Modifier,
    dd: Modifier,
    s: Modifier,
    f: Modifier,
    r: Modifier,
    x: Modifier,
    j: Modifier,
}

impl InputMethod {
    pub fn new(arr: [char; 12]) -> InputMethod {
        InputMethod {
            aa: Modifier::new(arr[0]),
            aw: Modifier::new(arr[1]),
            ee: Modifier::new(arr[2]),
            oo: Modifier::new(arr[3]),
            ow: Modifier::new(arr[4]),
            uw: Modifier::new(arr[5]),
            dd: Modifier::new(arr[6]),
            s: Modifier::new(arr[7]),
            f: Modifier::new(arr[8]),
            r: Modifier::new(arr[9]),
            x: Modifier::new(arr[10]),
            j: Modifier::new(arr[11]),
        } 
    }
    pub fn telex() -> InputMethod {
        InputMethod::new(['a', 'w', 'e', 'o', 'w', 'w', 'd', 's', 'f', 'r', 'x', 'j'])
    }
}
