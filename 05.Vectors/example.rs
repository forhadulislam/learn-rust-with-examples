fn main() {
    // Statements here are executed when the compiled binary is called

    let numbers = vec![34, 50, 25, 100, 65];
    println!("The length of the vector is: {}", numbers.len());
    let mut smallest = numbers[0];
    for number in numbers {
       if number < smallest {
           smallest = number;
       }
    }

    println!("The smallest number in the list is: {}", smallest);

}
