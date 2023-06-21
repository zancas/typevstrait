use type_crate;
fn main() {
    let a_instance = type_crate::AType {};
    use trait_crate::B;
    a_instance.default_foo();
    println!("Hello, world!");
}
