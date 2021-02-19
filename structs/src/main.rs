struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // declare an instance of a struct
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // creating new instance with struct update 
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("aotherusername567"),
        ..user1
    };

    // tuple structs, the fields dont have names
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Color(0, 0, 0);
}


fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user2(email: String, username: String) -> User {
    // if the variable has the same name as the field, you can omit the
    // field's name
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}