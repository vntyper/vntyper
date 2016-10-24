extern crate rustc_serialize;

use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
struct Foo {
    a: i64,
    x: i32,
}

fn main() {
    let foo = Foo{
        a: 3,
        x: 2,
    };
    println!("{}", json::encode(&foo).unwrap());
}
