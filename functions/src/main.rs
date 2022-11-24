// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;
    // 1. Try running this code with `cargo run` and take a look at the error.
    //
    // See if you can fix the error. It is right around here, somewhere.  If you succeed, then
    // doing `cargo run` should succeed and print something out.
    // inner scope error: either write code in the scope or remove the scope
    // {
    //     let area = area_of(width, height);
    //     println!("Area is {}", area); //corrected

    //     let volume = volume_of(width, height, depth);
    //     println!("Volume is {}", volume)
    // }
    //corrected
    let area = area_of(width, height);
    println!("Area is {}", area);

    // method 1
    // let volume = volume_of(width, height, depth);
    // println!("Volume is {}", volume)

    // method 2
    println!("Volume is {}", volume_of(width, height, depth));

    // 2. The area that was calculated is not correct! Go fix the area_of() function below, then run
    //    the code again and make sure it worked (you should get an area of 28).

    // 3. Uncomment the line below.  It doesn't work yet because the `volume` function doesn't exist.
    //    Create the `volume` function!  It should:
    //    - Take three arguments of type i32
    //    - Multiply the three arguments together
    //    - Return the result (which should be 280 when you run the program).
    //
    // If you get stuck, remember that this is *very* similar to what `area_of` does.
    //
    //println!("Volume is {}", volume(width, height, depth));
}

// funtion to calculate the area
fn area_of(x: i32, y: i32) -> i32 {
    // 2a. Fix this function to correctly compute the area of a rectangle given
    // dimensions x and y by multiplying x and y and returning the result.
    //
    // return 0; // returns the value of 0
    // return x * y; // return the area value
    x * y // tail expression
          // Challenge: It isn't idiomatic (the normal way a Rust programmer would do things) to use
          //            `return` on the last line of a function. Change the last line to be a
          //            "tail expression" that returns a value without using `return`.
          //            Hint: `cargo clippy` will warn you about this exact thing.
}

// funtion to calculate the volume
fn volume_of(x: i32, y: i32, z: i32) -> i32 {
    x * y * z // tail expression
}
