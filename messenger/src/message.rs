const USERS_MSG_PATH: &str = "/tmp/users";

pub mod user {
    use glob::glob;
    use std::fs;

    use super::message::Message;

    #[derive(Debug, Clone)]
    pub struct User {
        pub username: String,
    }

    impl User {
        // TODO: currently only one message is supported
        pub fn get_inbox(&self) -> Vec<Message> {
            let mut inbox: Vec<Message> = Vec::new();
            for file in glob(format!("{0}/{1}/*", super::USERS_MSG_PATH, self.username).as_str())
                .expect("Failed to read inbox")
            {
                if let Ok(entry) = file {
                    let from = entry.file_stem().unwrap().to_os_string();
                    let msg_contents = fs::read_to_string(entry).expect("Failed to read message");
                    for msg in msg_contents.split_terminator("\r\n") {
                        inbox.push(Message {
                            from: User::from(from.clone().into_string().unwrap()),
                            to: self.clone(),
                            content: msg.to_string(),
                        })
                    }
                }
            }
            inbox
        }
    }
    impl From<String> for User {
        fn from(s: String) -> Self {
            User { username: s }
        }
    }
}
pub mod message {
    use super::user;
    use std::fs;
    use std::io;

    #[derive(Debug)]
    pub struct Message {
        pub from: user::User,
        pub to: user::User,
        pub content: String,
    }

    impl Message {
        pub fn save(&self) -> io::Result<()> {
            if !fs::exists(format!("{0}/{1}", super::USERS_MSG_PATH, self.to.username)).unwrap() {
                let _ = fs::create_dir(format!("{0}/{1}", super::USERS_MSG_PATH, self.to.username));
            }
            let inbox_path = format!(
                "{0}/{1}/{2}.txt",
                super::USERS_MSG_PATH,
                self.to.username,
                self.from.username
            );
            fs::write(inbox_path, self.content.clone())
        }
    }
    impl TryFrom<Vec<String>> for Message {
        type Error = &'static str;
        fn try_from(v: Vec<String>) -> Result<Self, Self::Error> {
            let Some(from_username) = v.get(0) else {
                return Err("No from username");
            };
            let Some(to_username) = v.get(1) else {
                return Err("No to username found");
            };
            let Some(content) = v.get(2) else {
                return Err("No content found");
            };
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
