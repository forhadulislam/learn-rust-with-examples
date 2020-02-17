mod add;
use add::add_ints;
use remove::remove_ints;

mod remove {
    pub fn remove_ints(){
        println!("Remove integers")
    }
}


fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    add_ints();
    remove_ints();
}
