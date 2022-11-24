// part 1
// Make a new project named variables using cargo
// Declare the variable missiles and initialize it to 8
// Declare the variable ready and initialize it to 2
// println!("Firing {} of my {} missiles...", ready, missiles);

// part 2
// After the println!(...), subtract ready from missiles like this: missiles = missiles - ready;
// Add a second println!(...) to the end: println!("{} missiles left", missiles);
// Run your program again using cargo
// Did you run into an error about mutability? Go back and add mut at the right spot on the line where you declare missiles.
// Declare a constant named STARTING_MISSILES and set it to 8 (the type is i32).
// Declare a constant named READY_AMOUNT and set it to 2 (also i32).
// Use the constants to initialize missiles and ready

// part 3
// Explicitly annotate the variables with the type i32
// Try binding the variables all at once on one line using a pattern (parentheses and commas) -- can you figure out where mut goes?
// Instead of changing missiles, use the value missiles - ready directly in the second println!(...)
// Add another variable to your program but don't use it.
// Try modifying a constant in main() (for example, READY_AMOUNT = 1)

const STARTING_MISSILES: i32 = 8; //"const" are to be put to above main at module scope to make them available at everywhere
const READY_AMOUNT: i32 = 2; //"const" are to be put to above main at module scope to make them available at everywhere
fn main() {
    // println!("Hello, world!");
    // part 1
    // let missiles = 8;
    // let ready = 2;
    // println!("Firing {} of my {} missiles", ready, missiles);

    // part 2
    // let mut missiles = 8;
    // let ready = 2;
    // println!("Firing {} of my {} missiles", ready, missiles);
    // missiles = missiles - ready;
    // println!("{} missiles left", missiles);

    // part 2.1
    // const STARTING_MISSILES: i32 = 8; to be put to above main at module scope to make them available at everywhere
    // const READY_AMOUNT: i32 = 2; to be put to above main at module scope to make them available at everywhere
    // let mut missiles = STARTING_MISSILES;
    // let ready = READY_AMOUNT;
    // println!("Firing {} of my {} missiles", ready, missiles);
    // missiles = missiles - ready;
    // println!("{} missiles left", missiles);

    // part 3
    // let (mut missiles, ready) = (STARTING_MISSILES, READY_AMOUNT); warning: variable does not need to be mutable in this case
    let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    // let anothevariable: i32; //warning: unused varaiable : if this is intentional, prefix it with an underscore: `_anothervariable`
    // READY_AMOUNT = 1; // : error: cannot assign to this expression, invalid left-hand side of assignment
    println!("Firing {} of my {} missiles", ready, missiles);
    println!("{} missiles left", missiles - ready);
}
