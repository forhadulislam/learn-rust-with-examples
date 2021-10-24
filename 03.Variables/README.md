## Variables and types

In Rust, we define variables with let. Here is an example -

    let x = true;

Rust is a strictly typed programming language. 


### Scope

Scope if an important topic for Rust. 
    `
        let x = 1;
        {
            let y = 2;
            println!("{}, {}" x, y);
        }
        println!("{}, {}" x, y); // This line will cause an error
    `

Variables can be shadowed. 
    `
        let a = 1;
        {
            let a = 2;
            println!("{}", a); // will print 2
        }
        println!("{}", a); // will print 1
    `


