#[derive(Debug)]

struct User<'a>{
    name: &'a str,
    email: &'a str
}

fn main() {
    // Statements here are executed when the compiled binary is called


    let new_user = User{
        name: &String::from("John Doe"),
        email: &String::from("john.doe@email.com"),
    };

    println!("New User name: {}", new_user.name);
    println!("New User email: {}", new_user.email);
    println!("Struct content: {:?}", new_user);
}
