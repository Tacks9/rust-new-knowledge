fn main() {
    println!("main init");
    another_func();
}
fn another_func() {
    println!("func init");
    another_func1(1);
}
fn another_func1(x:i32) {
    println!("func1 init {}", x);
    println!("func2 return {}", another_func2());
}
fn another_func2() -> i32 {
    println!("func2 init");
    100
}