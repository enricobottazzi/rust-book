# rust-book
Me learning RUST

### Interesting Stuff

- Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.
- Use `cargo check` to check for errors without compiling. Use `cargo run` to compile and run the program.
- In Rust, variables are immutable by default, meaning once we give the variable a value, the value won’t change. We can make a variable mutable by adding the mut keyword before the variable name.
- The `&` indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.