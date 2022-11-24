// fn main() {
//     println!("Hello, world!");
// }

// A "use" declaration creates one or more local name bindings synonymous with some other path. Usually a "use" declaration is used to shorten the path required to refer to a module item. These declarations may appear in modules and blocks, usually at the top.
// use std::collections::hash_map; // import from standard library
// import
use hello::greet;

fn main() {
    // all funchtions in lib.rs are private functions by default
    // specifying the absolute path at every call site could be painful if the path is long; so make the "use" statement
    // hello::greet();
    greet();
}
