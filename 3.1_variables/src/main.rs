const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = THREE_HOURS_IN_SECONDS; // compile error if x is not declared as mut(able)
    println!("The value of x is: {x}");

    let y = 5;
    let y = y + 1; // creates a new variable y, shadowing the first one
    {
        let y = y * 2; // in the inner scope, this variable shadows the outer scope variable
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}"); // in the outer scope, the variable is unchanged
     
}
