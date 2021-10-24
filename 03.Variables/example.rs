fn main() {
    // Statements here are executed when the compiled binary is called

    let x = true;

    println!("x is {}", x);

    println!("Hello World!");

    // Defined typed variable
    let my_number : i32 = 10; // will show warning if with camelCase variable name
    println!("my_number value is: {}", my_number);

    // Define multiple variables in one statement
    let (foo, bar) = (10, 199.99);
    println!("value of `foo` is {} and value of `bar` is {}", foo, bar);

    // Variables are immutable by default because of Safety, Concurrency and Speed
    // If we try to assign some other values to an already assigned variable then compilation will fail
}
