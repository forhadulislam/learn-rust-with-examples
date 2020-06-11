#[derive(Debug)]
struct User{
    id: i32,
    name: String,
    email: String
}

fn main() {
    // Ownership and Borrowing example 

    let mut user = User{ name: "Forhad".to_string(), email: "forhad@example.com".to_string(), id: 980};

    update_me(&mut user);

    // modified user
    println!("modified user: {:?}", user);
}

fn update_me(user: &mut User){
    println!("updating user: {:?}", user);
    user.email = "updated@example.com".to_string();
    
}