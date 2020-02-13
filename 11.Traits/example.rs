struct Amazon{
    wallet: u32,
    pin: u8
}
struct Google{
    wallet: u32,
    pin: u8
}
struct Facebook{
    id: u32
}

impl Pay for Amazon {
    fn pay_with(&self, id: u32) -> bool{
        if()
    }
}

trait Pay {
    fn pay_with(&self, id: u32) -> bool;
}

fn main() {
    // Statements here are executed when the compiled binary is called

    let amazon_tx = Amazon{ wallet: 538000, pin: 5481 };

    // Print text to the console
    println!("Hello World!");
}
