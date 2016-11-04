// Copyright 2016 Do Duy.
// Licensed under the MIT license, see the LICENSE file or
// <http://opensource.org/licenses/MIT>

use modifier::Modifier;

#[derive(Debug, PartialEq)]
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

static TELEX: InputMethod = InputMethod {
    aa: Modifier { key: 'a' },
    aw: Modifier { key: 'w' },
    ee: Modifier { key: 'e' },
    oo: Modifier { key: 'o' },
    ow: Modifier { key: 'w' },
    uw: Modifier { key: 'w' },
    dd: Modifier { key: 'd' },
    s: Modifier { key: 's' },
    f: Modifier { key: 'f' },
    r: Modifier { key: 'r' },
    x: Modifier { key: 'x' },
    j: Modifier { key: 'j' },
};

impl InputMethod {
    pub fn telex() -> &'static InputMethod { &TELEX }
}
