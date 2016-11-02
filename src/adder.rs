#[test]
fn it_works() {
    assert!(true);
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_2() {
        assert_eq!(4, add_two(2));
    }
}