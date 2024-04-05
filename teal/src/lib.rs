pub mod onyx;
pub mod hi {
    pub mod hello;
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use self::{hi::hello::salute, onyx::User};

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn thing() {
    //    let a: hi::hello::salute;
        let user = User {
            id: "asd".to_owned(),
            rating: 12434
        };

        let salute = salute {
            num: 3
        };

    }

}
