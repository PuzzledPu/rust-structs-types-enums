struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }
    fn deactivate(&mut self) {
        self.active = false;
    }
    fn from_email(email: String) -> Self {
        let username = email.split('@').next().unwrap_or("").to_string();
        Self::new(username, email, String::from(""))
    }
    fn update_uri(&mut self, new_uri: String) {
        self.uri = new_uri;
    }
}

fn main() {
    let mut new_user = User::from_email(
        "puzzledpu@gmail.com".to_string()
    );
    println!("Hello, {}!", new_user.username);
    println!("Account {} status is: {}", new_user.username, new_user.active);
    new_user.deactivate();
    println!("Account {} status is: {}", new_user.username, new_user.active);
    new_user.update_uri("https://puzzledpu.wordpress.com".to_string());
    println!("Account {} URI is: {}", new_user.username, new_user.uri);
}
