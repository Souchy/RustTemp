use std::time::Instant;

// pub mod onyx {
    pub struct User {
        pub id: String,
        pub rating: i32,
    }

    pub struct Party {
        pub users: Vec<User>,
    }

    pub struct Queuee {
        pub user: User,
        pub queueTime: Instant,
    }
// }



pub mod fire {
    use std::time::Instant;
    pub struct User {
        pub id: String,
        pub rating: i32,
    }
}
