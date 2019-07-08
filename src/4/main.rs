/*
 * Rustの高度な機能（型）。
 * CreatedAt: 2019-07-08
 */
fn main() {
    a();
}
fn a() -> i32 {
    match 1 {
        1 => 5,
        _ => panic!(),
    }
}
