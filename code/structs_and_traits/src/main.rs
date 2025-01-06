// Think of traits as contracts (like interfaces)
pub trait SimpleTrait {
    fn describe(&self) -> String;
    fn update_email(&mut self, email: &str);
}

#[derive(Debug)]
struct Person {
    id: u8,
    name: String,
    email: String,
}

// Now we implement the trait for the specific Struct
impl SimpleTrait for Person {
    fn describe(&self) -> String {
        let desc = format!("id:{}, name:{}, email:{}", self.id, self.name, self.email);
        return desc;
    }

    fn update_email(&mut self, email: &str) {
        self.email = email.to_string();
    }
}

fn main() {
    let mut p = Person {
        id: 1,
        name: String::from("BBC"),
        email: String::from("bbc@demo.com"),
    };

    println!("{}", p.describe());

    // Now lets update a property in the person object
    p.update_email("bbc@email.com");
    println!("{}", p.describe());
}
