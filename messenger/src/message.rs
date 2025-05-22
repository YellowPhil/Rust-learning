pub mod user {
    #[derive(Debug)]
    pub struct User {
        pub username: String,
    }
}
pub mod message {
    use super::user;

    #[derive(Debug)]
    pub struct Message {
        from: user::User,
        to: user::User,
        content: String,
    }

    impl TryFrom<Vec<String>> for Message {
        type Error = &'static str;
        fn try_from(v: Vec<String>) -> Result<Self, Self::Error> {
            let from_username = v.get(0).ok_or("No from username found")?;
            let to_username = v.get(0).ok_or("No to username found")?;
            let content = v.get(0).ok_or("No content found")?;
            Ok(Message {
                from: user::User {
                    username: from_username.clone(),
                },
                to: user::User {
                    username: to_username.clone(),
                },
                content: content.clone(),
            })
        }
    }
}
