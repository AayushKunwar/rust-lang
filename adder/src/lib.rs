pub fn add(left: usize, right: usize) -> usize {
    let mut _foo = 69;
    _foo += 1;
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

    #[test]
    fn foobar(){
        assert_eq!(2,3);
    }

    #[test]
    fn whatwhat(){
        panic!("oh no what happen?");
    }
}
