struct User{
    name: String,
    email: String
}

impl User{
    fn get_user_as_string(&self) -> String {
        format!("name: {}, email: {}", self.name, self.email)
    }

    fn update_email(&mut self, email: String){
        self.email = email
    }
}

fn main() {
    // Statements here are executed when the compiled binary is called


    let mut user = User{
        name: "John Doe".to_string(),
        email: "john.doe@email.com".to_string(),
    };

    println!("New User name: {}", user.name);
    println!("New User email: {}", user.email);

    println!("get_user_as_string: {}", user.get_user_as_string());

    user.update_email("ejohn.parker@email.com".to_string());

    println!("get_user_as_string: {}", user.get_user_as_string());
}
