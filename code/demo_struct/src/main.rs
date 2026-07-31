struct DbUser {
    enabled: bool,
    name: String,
    uid: u16,
}

// Implementation block
impl DbUser {
    
    // Associated function is used like constructors
    fn new (enabled:bool, name:String, uid:u16) -> Self {
        Self {
            enabled: enabled,
            name = String::from(name),
            uid = uid,
        }
    }
}

// Traits define required behaviuor
trait UserProperties {
    fn print(&self) -> &str;
}


impl UserProperties for DbUser {
    fn print(&self) -> &str {
        "User#{}: {} enabled:{}", self.uid, self.name, self.enabled
    }
}

// ---------------------------------- //

fn main() {
    let u1 = DbUser {
        uid: 1,
        enabled: true,
        name: String::from("BBC"),        
    };

    println!("User#{}: {}", u1.uid, u1.name);

    let mut u2 = DbUser::new(
        enabled: true,
        name: String::from("ABC"),
        uid: 2,
    );

    println!("{}", u2.print());
}
