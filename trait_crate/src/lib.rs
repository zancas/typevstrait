pub trait B {
    fn default_foo(&self) {
        println!("I am a default implementation on B!");
    }
}

impl B for type_crate::AType {}
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
