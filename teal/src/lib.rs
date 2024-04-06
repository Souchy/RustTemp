
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;

use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;



pub mod onyx;
pub mod hi {
    pub mod hello;
}
pub mod net;

pub type Reader = Arc<Mutex<OwnedReadHalf>>;
pub type Writer = Arc<Mutex<OwnedWriteHalf>>;



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
