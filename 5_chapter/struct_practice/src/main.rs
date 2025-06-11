
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    //defining user2 here ends user1's lifetime becuase we moved
    //username which is a String
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let _user3 = User {
        email: String::from("yetanother@email.com"),
        ..user2
    };

    //rust also supports tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let origin= Point(0,0,0);

}