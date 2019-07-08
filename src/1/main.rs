/*
 * Rustの高度な機能（型）。
 * CreatedAt: 2019-07-08
 */
fn main() {
    let f: Box<Fn() + Send + 'static> = Box::new(|| println!("hi"));
}
fn takes_long_type(f: Box<Fn() + Send + 'static>) {
    // --snip--
}
fn returns_long_type() -> Box<Fn() + Send + 'static> {
    // --snip--
}
