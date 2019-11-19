fn main() {
    let my_string = String::from("Current string");
    let mut another_string = String::from("Foo bar");

    // Print text to the console

    println!("My string is : {}", my_string);
    println!("Current length of my_string is : {}", my_string.len());

    println!("Another string is : {}", another_string);

    another_string.push_str(" whatever");
    println!("Updated another string: {}", another_string);
}
