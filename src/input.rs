// Copyright 2016 Do Duy.
// Licensed under the MIT license, see the LICENSE file or
// <http://opensource.org/licenses/MIT>

use modifier::Modifier;
use input_method::InputMethod;

pub struct Input {
    word: String,
    modifier: Modifier,
    input_method: InputMethod,
}
