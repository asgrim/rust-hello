#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn new(username: String, email: String) -> User {
        User {
            email,
            username,
            sign_in_count: 0,
            active: false,
        }
    }
    fn change_email(&mut self, new_email: &str) {
        self.email = String::from(new_email);
    }
    fn with_email(self, new_email: &str) -> User {
        // same as change-email
        //   but: struct update syntax shown here
        //   also: we're not borrowing self, we're taking ownership and returning ownership
        User {
            email: String::from(new_email),
            ..self
        }
    }
    fn sign_in(&mut self) -> u64 {
        self.active = true;
        self.sign_in_count += 1;
        return self.sign_in_count;
    }
}

struct Colour (u8, u8, u8);

pub fn run() {
    let c = Colour(255, 0, 255);
    println!("#{:x}{:x?}{:x}", c.0, c.1, c.2);

    let user1 = User::new(
        String::from("asgrim"),
        String::from("oldemail@example.com")
    );

    // Note: original user1 lost "ownership" since we used "with_email", but we can re-use the var
    let mut user1 = user1.with_email("newemail@example.com");

    user1.change_email("foo@bar.com");
    user1.sign_in();

    // println!("User: {} {} {} {}", user1.email, user1.username, user1.active, user1.sign_in_count);
    // When you use the `#[derive(Debug)]` trait, you can use {:?}, or for pretty print {:#?}
    println!("User: {:?}", user1);
}
