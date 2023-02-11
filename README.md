# rust-book
Me learning RUST

### Interesting Stuff

- Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.
- Use `cargo check` to check for errors without compiling. Use `cargo run` to compile and run the program.
- In Rust, variables are immutable by default, meaning once we give the variable a value, the value won’t change. We can make a variable mutable by adding the mut keyword before the variable name.
- The `&` indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
- What about constants? Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables. First, you aren’t allowed to use mut with constants. Constants aren’t just immutable by default—they’re always immutable. You declare constants using the const keyword instead of the let keyword, and the type of the value must be annotated. The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.
- A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. You may recognize these from other programming languages. Let’s jump into how they work in Rust.
- Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays. 
- Vectors are similar to arrays, but they can grow or shrink in size.
- Return expression do not need a semicolon. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value.
