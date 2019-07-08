/*
 * Rustの高度な機能（型）。
 * CreatedAt: 2019-07-08
 */
use std::boxed::Box;
type Thunk = Box<Fn() + Send + 'static>;
fn main() {
    let f: Thunk = Box::new(|| println!("hi"));
}
fn takes_long_type(f: Thunk) {
    // --snip--
}
fn returns_long_type() -> Thunk {
    // --snip--
}
